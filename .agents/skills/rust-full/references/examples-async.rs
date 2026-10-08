//! Complete Tokio examples of bounded streaming admission and retained read progress.
//! Dependencies: tokio with rt, macros, sync, time, and io-util features; test-util for tests.
//! A supervisor must retain and await these operations; dropping their future cannot await cleanup.

use std::future::Future;
use std::io;
use std::num::NonZeroUsize;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::sync::{mpsc, watch};
use tokio::task::{JoinError, JoinSet};

/// Preserves the distinction between an application failure and task termination.
/// The first observed failure is returned after ordinary async siblings are aborted and joined.
#[derive(Debug)]
pub enum RunError<E> {
    /// An admitted worker returned an application error.
    Worker(E),
    /// An admitted worker panicked or was cancelled unexpectedly.
    Join(JoinError),
}

/// Aborts and joins all outstanding ordinary async tasks.
/// Cleanup task results are discarded here; production may aggregate them separately.
async fn abort_and_drain<E: 'static>(tasks: &mut JoinSet<Result<(), E>>) {
    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
}

/// Processes a channel without spawning more than limit tracked workers.
/// Shutdown stops admission, aborts siblings, and observes their async termination.
///
/// The producer must bound channel bytes as well as items. Dropping this supervisor
/// aborts its JoinSet but cannot await it; an outer owner must keep shutdown alive.
/// The work factory must not panic; worker-future panics are reported through JoinError.
pub async fn run_bounded<R, F, Fut, E>(
    mut input: mpsc::Receiver<R>,
    limit: NonZeroUsize,
    mut shutdown: watch::Receiver<bool>,
    mut work: F,
) -> Result<(), RunError<E>>
where
    F: FnMut(R) -> Fut,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    E: Send + 'static,
{
    let mut tasks = JoinSet::new();
    let mut accepting = true;
    loop {
        if *shutdown.borrow() {
            input.close();
            abort_and_drain(&mut tasks).await;
            return Ok(());
        }
        if !accepting && tasks.is_empty() {
            return Ok(());
        }
        tokio::select! {
            // Shutdown has deliberate priority over always-ready input/completions.
            biased;
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow_and_update() {
                    input.close();
                    abort_and_drain(&mut tasks).await;
                    return Ok(());
                }
            }
            result = tasks.join_next(), if !tasks.is_empty() => {
                let error = match result.expect("nonempty JoinSet") {
                    Ok(Ok(())) => None,
                    Ok(Err(error)) => Some(RunError::Worker(error)),
                    Err(error) => Some(RunError::Join(error)),
                };
                if let Some(error) = error {
                    input.close();
                    abort_and_drain(&mut tasks).await;
                    return Err(error);
                }
            }
            item = input.recv(), if accepting && tasks.len() < limit.get() => {
                match item {
                    Some(item) => { tasks.spawn(work(item)); }
                    None => accepting = false,
                }
            }
        }
    }
}

/// Stores the caller's partial-read progress across dropped read futures.
/// The input buffer is prebounded by its owner rather than by untrusted wire length.
pub struct PendingRead {
    /// Destination retained across cancellation and subsequent resumption.
    bytes: Vec<u8>,
    /// Initialized protocol bytes already received into the destination.
    filled: usize,
}

impl PendingRead {
    /// Creates a destination after enforcing the supplied size budget.
    /// Fallible reservation avoids silently converting budget enforcement into infallible allocation.
    pub fn new(length: usize, maximum: usize) -> io::Result<Self> {
        if length > maximum {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input exceeds limit",
            ));
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(length).map_err(io::Error::other)?;
        bytes.resize(length, 0);
        Ok(Self { bytes, filled: 0 })
    }

    /// Resumes an exact-sized input using cancellation-safe incremental reads.
    /// Each completed read updates owner-held progress before the next suspension.
    pub async fn resume<R: AsyncRead + Unpin>(&mut self, input: &mut R) -> io::Result<&[u8]> {
        while self.filled < self.bytes.len() {
            match input.read(&mut self.bytes[self.filled..]).await {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "incomplete input",
                    ));
                }
                Ok(count) => self.filled += count,
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }
        Ok(&self.bytes)
    }

    pub fn received(&self) -> usize {
        self.filled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;
    use tokio::sync::Semaphore;

    /// Counts active worker lifetimes, including cancellation destruction.
    /// The owned counter stays alive until the task's guard is destroyed.
    struct Active {
        /// Shared active count decremented when this worker scope ends.
        count: Arc<AtomicUsize>,
    }

    impl Drop for Active {
        fn drop(&mut self) {
            self.count.fetch_sub(1, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn admission_is_bounded_and_completion_is_drained() {
        let (sender, receiver) = mpsc::channel(2);
        let (shutdown, cancel) = watch::channel(false);
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let complete = Arc::new(AtomicUsize::new(0));
        let producer = tokio::spawn(async move {
            for item in 0..100 {
                sender.send(item).await.unwrap();
            }
        });
        run_bounded(receiver, NonZeroUsize::new(3).unwrap(), cancel, |_| {
            let active = active.clone();
            let peak = peak.clone();
            let complete = complete.clone();
            async move {
                let current = active.fetch_add(1, Ordering::SeqCst) + 1;
                let _guard = Active { count: active };
                peak.fetch_max(current, Ordering::SeqCst);
                tokio::task::yield_now().await;
                complete.fetch_add(1, Ordering::SeqCst);
                Ok::<_, &'static str>(())
            }
        })
        .await
        .unwrap();
        producer.await.unwrap();
        assert!(peak.load(Ordering::SeqCst) <= 3);
        assert_eq!(complete.load(Ordering::SeqCst), 100);
        assert_eq!(active.load(Ordering::SeqCst), 0);
        drop(shutdown);
    }

    #[tokio::test]
    async fn first_worker_error_waits_for_sibling_destruction() {
        let (sender, receiver) = mpsc::channel(2);
        sender.send(false).await.unwrap();
        sender.send(true).await.unwrap();
        drop(sender);
        let (_shutdown, cancel) = watch::channel(false);
        let active = Arc::new(AtomicUsize::new(0));
        let started = Arc::new(Semaphore::new(0));
        let result = run_bounded(receiver, NonZeroUsize::new(2).unwrap(), cancel, |fails| {
            let active = active.clone();
            let started = started.clone();
            async move {
                if fails {
                    let permit = started.acquire().await.unwrap();
                    permit.forget();
                    Err("failure")
                } else {
                    active.fetch_add(1, Ordering::SeqCst);
                    let _guard = Active { count: active };
                    started.add_permits(1);
                    std::future::pending::<Result<(), &'static str>>().await
                }
            }
        })
        .await;
        assert!(matches!(result, Err(RunError::Worker("failure"))));
        assert_eq!(active.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn existing_shutdown_prevents_initial_admission() {
        let (sender, receiver) = mpsc::channel(1);
        sender.send(()).await.unwrap();
        let (_shutdown, cancel) = watch::channel(true);
        run_bounded(receiver, NonZeroUsize::new(1).unwrap(), cancel, |_| {
            panic!("shutdown must prevent admission");
            #[allow(unreachable_code)]
            async {
                Ok::<_, &'static str>(())
            }
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn shutdown_waits_for_active_worker_destruction() {
        let (sender, receiver) = mpsc::channel(1);
        sender.send(()).await.unwrap();
        let (shutdown, cancel) = watch::channel(false);
        let active = Arc::new(AtomicUsize::new(0));
        let started = Arc::new(Semaphore::new(0));
        let signal = {
            let started = started.clone();
            tokio::spawn(async move {
                let permit = started.acquire().await.unwrap();
                permit.forget();
                shutdown.send(true).unwrap();
            })
        };
        run_bounded(receiver, NonZeroUsize::new(1).unwrap(), cancel, |_| {
            let active = active.clone();
            let started = started.clone();
            async move {
                active.fetch_add(1, Ordering::SeqCst);
                let _guard = Active { count: active };
                started.add_permits(1);
                std::future::pending::<Result<(), &'static str>>().await
            }
        })
        .await
        .unwrap();
        signal.await.unwrap();
        assert_eq!(active.load(Ordering::SeqCst), 0);
        assert!(sender.send(()).await.is_err());
    }

    #[tokio::test]
    async fn worker_panic_is_a_join_failure() {
        let (sender, receiver) = mpsc::channel(1);
        sender.send(()).await.unwrap();
        drop(sender);
        let (_shutdown, cancel) = watch::channel(false);
        let result = run_bounded(receiver, NonZeroUsize::new(1).unwrap(), cancel, |_| async {
            panic!("worker panic");
            #[allow(unreachable_code)]
            Ok::<_, &'static str>(())
        })
        .await;
        assert!(matches!(result, Err(RunError::Join(error)) if error.is_panic()));
    }

    #[tokio::test(start_paused = true)]
    async fn cancellation_preserves_partial_read_for_resumption() {
        let (mut writer, mut reader) = tokio::io::duplex(16);
        writer.write_all(b"ab").await.unwrap();
        let mut pending = PendingRead::new(4, 4).unwrap();
        let result =
            tokio::time::timeout(Duration::from_secs(1), pending.resume(&mut reader)).await;
        assert!(result.is_err());
        assert_eq!(pending.received(), 2);
        writer.write_all(b"cd").await.unwrap();
        assert_eq!(pending.resume(&mut reader).await.unwrap(), b"abcd");
    }

    #[tokio::test]
    async fn partial_eof_and_budget_failure_are_explicit() {
        assert!(PendingRead::new(5, 4).is_err());
        let (mut writer, mut reader) = tokio::io::duplex(16);
        writer.write_all(b"a").await.unwrap();
        drop(writer);
        let mut pending = PendingRead::new(2, 2).unwrap();
        assert_eq!(
            pending.resume(&mut reader).await.unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        assert_eq!(pending.received(), 1);
        let mut empty = PendingRead::new(0, 0).unwrap();
        assert!(empty.resume(&mut reader).await.unwrap().is_empty());
    }
}
