# Async execution and cancellation

## Contents

- [Future state and task ownership](#future-state-and-task-ownership)  
- [Send, static, and trait APIs](#send-static-and-trait-apis)  
- [Blocking and lock scope](#blocking-and-lock-scope)  
- [Admission and backpressure](#admission-and-backpressure)  
- [Cancellation is a state transition](#cancellation-is-a-state-transition)  
- [Tracked shutdown](#tracked-shutdown)  
- [Errors, fairness, and observability](#errors-fairness-and-observability)  
- [Custom polling and streams](#custom-polling-and-streams)  
- [Sources](#sources)  

## Future state and task ownership

Values needed after suspension become part of the future's stored state. Large  
inline buffers or nested futures can therefore increase task memory. Measure  
future size and concurrent task count before boxing everything. Boxing adds an  
allocation and indirection; it can reduce the size of an outer future when that  
tradeoff matters.  

Joining futures in one task gives concurrent progress, not automatic CPU  
parallelism. A CPU loop with no suspension can monopolize its executor thread.  
Spawning introduces independently scheduled work, ownership requirements, and a  
completion responsibility. Use direct awaiting when the work has the same  
lifetime and no independent scheduling requirement.  

## Send, static, and trait APIs

For Tokio's ordinary `spawn` , the future and its output must be `Send + 'static` .  
`'static` here means the future contains no borrow that can expire while the task  
runs. It does not mean the task or its owned data must live forever. Moving an  
owned `String` , `Vec` , or `Arc` into the future satisfies a different ownership  
requirement from borrowing a local variable.  

`async move` captures values by value; moving a reference still leaves a  
reference with its original lifetime. An owned `Arc<Service>` can be moved into  
a task and then borrowed inside it. There is no requirement to clone data or  
put every borrowed value behind `Arc<Mutex<_>>` .  

```rust
use std::sync::Arc;

/// Returns the length while borrowing the task's owned service value.
/// This borrow is internal to the future and does not escape it.
async fn name_len(name: &str) -> usize {
    name.len()
}

/// Spawns work with shared ownership of immutable text.
/// The spawned future owns the Arc and borrows it only inside the task.
fn spawn_name(name: Arc<str>) -> tokio::task::JoinHandle<usize> {
    tokio::spawn(async move { name_len(&name).await })
}
```

A future is `Send` only when its captured and suspended state permits transfer  
between threads. Inspect guards and non-`Send` values that remain across awaits.  
Use a lexical block to end a lock guard before suspension. An owned local  
non-`Send` value with a clearly ended scope can be used before an await; a  
non-`Send` captured parameter can still make the initial future state non-`Send`.  
Check the complete future rather than using a blanket rule about locals.  

Native async functions in traits do not automatically promise a `Send` returned  
future. If callers must spawn generic implementations, express that promise in  
the public API, for example with a return-position `impl Future + Send` :  

```rust
use std::future::Future;

/// Fetches owned content with a future transferable between threads.
/// Implementations must keep all suspended state compatible with Send.
trait Fetch: Sync {
    fn fetch(&self) -> impl Future<Output = Result<Vec<u8>, std::io::Error>> + Send;
}
```

`impl Future` preserves static dispatch but affects object compatibility. A  
boxed `dyn Future + Send + 'a` is a possible object-safe boundary with allocation  
and dynamic dispatch costs. Choose it when runtime polymorphism is needed.  
Similarly, `AsyncFn + Send` describes the callable's transfer property; it does  
not by itself guarantee that every future returned by a call is `Send` .  

Local task sets support non-`Send` futures on their owning thread, but retain  
task lifetime requirements. Choose them for a real thread-affinity requirement,  
not to conceal shared-state problems.  

## Blocking and lock scope

Keep blocking filesystem calls, synchronous network operations, and long CPU  
work away from executor workers. Use the runtime's async operation when it fits.  
Use `spawn_blocking` for suitable synchronous work and a bounded CPU pool for  
sustained parallel computation. Limit admission before dispatch: the blocking  
pool's thread limit does not bound the bytes retained by queued closures.  

A started `spawn_blocking` closure generally cannot be stopped by aborting its  
handle. Make long-running work cooperate with shutdown or define a separate  
process boundary when termination is required. Runtime shutdown timeouts can  
stop waiting without stopping that closure.  

A standard mutex is suitable for short non-suspending state changes. Release  
the guard before awaits and callbacks. Tokio's mutex permits a guard across an  
await, which can be necessary for a stateful asynchronous resource, but can also  
serialize unrelated requests and turn network latency into lock hold time.  
Consider a task that owns the resource and accepts bounded commands.  

```rust
use std::sync::Mutex;

/// Copies the current configuration under a short synchronous lock.
/// The owned snapshot can then cross an await without retaining the guard.
fn snapshot(config: &Mutex<String>) -> Result<String, &'static str> {
    let guard = config.lock().map_err(|_| "configuration lock poisoned")?;
    Ok(guard.clone())
}
```

If a later result must be committed only against the same configuration, a  
snapshot alone is insufficient. Capture a version, then compare it under the  
lock before commit. Decide whether a conflict causes retry, rejection, or merge.  

## Admission and backpressure

Bound admitted tasks, queue entries, and retained bytes separately. A semaphore  
acquired inside an already spawned task limits active resource use but leaves  
the number of waiting tasks unbounded. Acquire before spawning, or stop taking  
input while a bounded `JoinSet` is full. Carry the permit through the complete  
resource lifetime, including response buffers when those dominate memory.  

Drain completions during sustained input. Otherwise completed task outputs can  
accumulate even when active work is limited. A bounded queue is useful only if  
producers await capacity or follow an explicit rejection policy.  

Do not hold a permit while awaiting work that needs the same exhausted permit  
pool. Separate admission classes or restructure the dependency. A bounded  
pipeline can still deadlock through resource cycles.  

Bounded channels count messages, not necessarily bytes. A queue of 100 messages  
can retain gigabytes if each message owns a large buffer. Validate maximum  
message size before enqueueing or use a byte budget alongside the item budget.  

## Cancellation is a state transition

Dropping a future usually abandons its in-progress state. It does not undo  
external effects. A timeout after a payment request can mean the payment  
succeeded but its response was lost. Use idempotency keys, transaction state,  
or reconciliation when retrying has side effects.  

`select!` drops losing branch futures. Check each operation's documented  
cancellation safety. In Tokio, methods such as `read_exact` and `write_all` can  
lose visible progress when their futures are dropped. For a parser, store the  
buffer and offset outside the selected future, perform cancellation-safe  
incremental `read` , and update the offset after each completed read. The caller  
must retain that parser state if cancellation ends the operation.  

For channel sends, losing a selected `send(value)` drops that value. Reserve  
capacity first when the caller must retain ownership until admission, then  
send through the permit. Repeatedly cancelling a queued lock or semaphore  
acquisition can lose queue position; memory safety does not imply fairness.  

Passing a `JoinHandle` by value to `timeout` is a common trap: expiry drops the  
handle and detaches the spawned task. Borrow the handle to retain control:  

```rust
use std::time::Duration;

/// Waits up to the deadline and then requests task cancellation.
/// Awaiting the retained handle observes completion of the abort request.
async fn wait_or_abort(mut task: tokio::task::JoinHandle<()>) {
    if tokio::time::timeout(Duration::from_secs(1), &mut task)
        .await
        .is_err()
    {
        task.abort();
        let _ = task.await;
    }
}
```

This policy concerns ordinary async tasks. Abort is cooperative with executor  
polling and cannot interrupt a blocking closure that has already started.  
Dropping this helper itself can detach its handle; a top-level supervisor must  
own shutdown if cancellation can occur there too.  

For multi-step state changes, arrange the invariant so cancellation leaves a  
valid intermediate state. Use a synchronous RAII rollback only where rollback  
can actually be synchronous. `Drop` cannot await a network cleanup. Record a  
pending transaction for a tracked cleanup task or provide explicit async close.  

## Tracked shutdown

Define shutdown in terms of admitted work and observable completion:  

1. Stop accepting new work and prevent new producers from entering.  
2. Close or drop channel senders as appropriate; remove all retained sender  
   clones if receivers must see end-of-stream.  
3. Signal cancellation or allow admitted work to drain according to policy.  
4. Await task completion and inspect errors. If a deadline expires, abort  
   ordinary async tasks and await their termination where the policy permits.  
5. Flush or reconcile external state that task destruction cannot finish.  

Dropping a `JoinSet` aborts its tasks but does not await their completion.  
`abort_all` requests cancellation; drain `join_next` to observe termination.  
`JoinSet::shutdown` aborts and waits but discards task results, so use explicit  
draining if shutdown failures must be reported.  

Tokio-util's `TaskTracker::close` affects when `wait` can complete; it does not  
prevent later task creation. Enforce admission shutdown separately. A tracker  
tracks lifetimes but is not a substitute for inspecting each task's result.  

## Errors, fairness, and observability

Distinguish a task's application error from `JoinError` for panic or cancellation.  
On failure in a group, define whether siblings continue, drain, or abort. Do not  
return early while silently leaving them running. Preserve the primary error  
and decide how to report cleanup failures.  

Default `select!` polling order and `biased;` express different scheduling  
policies. A biased always-ready branch can starve later branches. Place shutdown  
first only when that priority is deliberate, and test progress under continuous  
input. No branch order fixes every fairness problem.  

Instrument queue depth, queued bytes, in-flight work, permit wait time, lock  
hold time, cancellation, and task errors. Track high-percentile latency under  
load; average throughput alone can hide starvation or memory growth.  

## Custom polling and streams

If a custom future is justified, close the check/register race: checking that  
work is incomplete and then storing a waker must not miss a completion between  
those steps. Use one protected state transition or the documented register-and-  
recheck protocol of an established wake primitive. Refresh the stored waker  
when the polling context changes; an old task's waker may no longer suffice.  
Wake outside a state lock where possible to avoid executor reentrancy or  
contention while the protected state is inaccessible.  

A pending result needs a wakeup path; busy polling is not a wakeup protocol.  
Bound work performed per poll and arrange a later wake when voluntarily yielding.  
Treat polling after completion according to the future's contract; do not  
accidentally take an already-consumed result or assume repeated polling must  
return the same value. Pin projections must preserve any structurally pinned  
inner future through state replacement and destruction.  

For streams, bound buffered concurrency and outputs independently. Ordered  
buffering can retain completed later outputs behind a slow earlier item;  
unordered buffering changes delivery order. Choose the error and cancellation  
policy for in-flight items before using either adapter. A fold avoids collecting  
results but still needs bounded accumulator memory and defined numeric behavior.  

## Sources

The [Microsoft async book](https://microsoft.github.io/RustTraining/async-book/)  
provides the teaching context. Exact runtime contracts come from  
[Tokio select](https://docs.rs/tokio/latest/tokio/macro.select.html) ,  
[JoinSet](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html) ,  
[spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html) , and  
[Tokio mutex](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html) .
