# Concurrency: Rust versus C++

Examples use Rust 1.99, edition 2024, and C++20. Tokio examples require Tokio 1 with `rt`, `macros`,  
`sync`, and `time`. Tokio is an example runtime, not a mandatory dependency. Prefer the project's  
established runtime or CPU task pool.

## Task selection and lazy futures

**Use:** async tasks for concurrent nonblocking I/O; an established bounded CPU pool for sustained CPU  
parallelism; direct calls for small work. Await directly when independent scheduling is unnecessary. Do  
not create a thread for every application operation, add a runtime to a synchronous CLI without need, or  
run blocking/long CPU work on executor workers.

**Difference:** calling a Rust `async fn` returns a future without running its body. Awaiting/polling  
drives it. C++20 `std::async(std::launch::async, ...)` starts execution; `get()` waits and collects its  
result. C++20 coroutines can be lazy or eager according to their promise/runtime; they are not all  
`std::async`. Neither language's async syntax inherently implies CPU parallelism or a built-in I/O  
runtime.

```cpp
#include <cassert>
#include <future>
int main() {
    auto result = std::async(std::launch::async, [] { return 42; });
    assert(result.get() == 42);
}
```

```rust
async fn answer() -> i32 { 42 }
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let future = answer(); // The body has not run yet.
    assert_eq!(future.await, 42);
}
```

Use `join!` to advance independent I/O futures in one task; it does not distribute CPU loops over cores. A  
`try_join!` error can drop unfinished sibling futures, so define their cancellation contract before  
composing them. Use a task pool/Rayon for CPU data parallelism when it fits the project. Standard scoped  
threads are useful for finite borrowed work, not a reusable executor.

## Transfer and sharing contracts

**Use:** move owned inputs into retained tasks; share immutable snapshots when genuinely needed; partition  
mutable outputs or return results for reduction. Let the compiler derive `Send` and `Sync`. Do not add  
unsafe trait implementations merely to make spawning compile.

**Difference:** Rust `Send` checks transfer between threads; `Sync` checks cross-thread sharing of `&T`.  
C++20 standard references and ownership handles do not express these auto-trait restrictions. For example,  
Rust rejects sending `Rc` to another thread; C++ permits sending a `shared_ptr`, while the programmer  
still must synchronize mutable payload access.

```cpp
#include <cassert>
#include <future>
#include <memory>
int main() {
    auto text = std::make_shared<const int>(42);
    auto task = std::async(std::launch::async, [text] { return *text; });
    assert(task.get() == 42);
}
```

```rust,compile_fail
use std::rc::Rc;
fn main() {
    let value = Rc::new(42);
    std::thread::spawn(move || println!("{value}")); // Rc is not Send.
}
```

```rust
use std::sync::Arc;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let value = Arc::new(42);
    let task = tokio::spawn(async move { *value });
    assert_eq!(task.await.unwrap(), 42);
}
```

Tokio ordinary `spawn` requires its future and output to be `Send + 'static`, even on this current-thread  
runtime. `async move` moves captures; moving a borrowed reference does not extend its lifetime. Directly  
awaited futures may borrow local data. A `LocalSet` permits non-`Send` tasks on its owning thread but does  
not eliminate task-lifetime requirements. Native async trait methods do not by themselves promise `Send`  
returned futures; state that bound when callers need it.

## Lock guards and protected data

**Use:** `Mutex<T>` for unavoidable synchronized state; perform short state changes, end the lexical guard  
scope, then await. For an asynchronous resource that must be held across suspension, consider its owner  
task or an async mutex with deliberate serialization. Define poison recovery rather than blindly  
discarding the signal.

**Difference:** C++ `mutex` and its payload are normally separate objects; discipline or an authored  
wrapper enforces the relationship. Rust `Mutex<T>` exposes shared mutation through its guard. It prevents  
unlocked mutation through safe shared access, but does not prevent deadlock or make a multi-step operation  
transactional. Unique access through `get_mut` needs no runtime lock. Standard Rust mutex poisoning is  
advisory and not a soundness proof.

```cpp
#include <cassert>
#include <mutex>
struct Counter {
    std::mutex mutex;
    int value = 0;
    void increment() { std::lock_guard guard(mutex); ++value; }
};
int main() { Counter counter; counter.increment(); assert(counter.value == 1); }
```

```rust
use std::sync::Mutex;
fn increment(counter: &Mutex<i32>) -> Result<(), &'static str> {
    let mut guard = counter.lock().map_err(|_| "counter poisoned")?;
    *guard += 1;
    Ok(())
}
fn main() {
    let counter = Mutex::new(0);
    increment(&counter).unwrap();
    assert_eq!(*counter.lock().unwrap(), 1);
}
```

Atomics require the same publication/order/reclamation reasoning as modern C++; Rust does not remove that  
proof obligation. Use `Relaxed` only where atomicity alone is sufficient. For payload publication, name  
the release observed by the acquire. Do not use `SeqCst` as a transaction or invent lock-free reclamation  
without need.

## Task handles and cancellation

**Use:** retain handles, bound admission before spawning, drain completions, and inspect application  
errors separately from task panic/cancellation. On shutdown, stop admission, cancel or drain by policy,  
and observe termination. Bound message bytes and completed-result retention as well as item counts.

**Difference:** dropping a Tokio `JoinHandle` detaches the task; dropping a Rust future stops polling that  
future, but neither action rolls back external effects. A C++ future holding the last reference to a state  
created by `std::async` can wait on destruction. A C++ future generally has no universal cancellation  
operation. Tokio `abort` requests ordinary async task cancellation; await the retained handle to observe  
the result. A started `spawn_blocking` closure cannot generally be stopped this way.

```cpp
#include <future>
int main() {
    auto task = std::async(std::launch::async, [] { return 42; });
    (void)task.get(); // Own completion explicitly, rather than relying on destruction.
}
```

```rust
use std::time::Duration;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut task = tokio::spawn(std::future::pending::<()>());
    if tokio::time::timeout(Duration::from_millis(1), &mut task).await.is_err() {
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
    }
}
```

Tokio `JoinHandle::is_finished()` inspects task completion without collecting its result. Await the handle  
to collect results or observe cancellation; requesting abort may precede completion. Unlike C++  
`future::valid()`, this is a readiness query. Borrowing the handle into `timeout` retains control.  
Passing it by value and timing out can detach it. A supervisor that is itself dropped cannot await  
cleanup; its owner must retain the shutdown operation. `JoinSet` drop requests abort but does not wait for  
termination. Do not treat a token signal, timeout, or `abort_all` as observed completion.

For partial input, keep buffer/progress in an owner outside canceled branch futures. Verify the actual  
operation's cancellation contract; moving the buffer alone does not recover an offset lost inside  
`read_exact`. For side effects, use explicit transaction/idempotency/reconciliation policy. `Drop` can  
release local synchronous resources; it cannot await remote cleanup.

## Sources

- [Future contract](https://doc.rust-lang.org/std/future/trait.Future.html)
- [Send](https://doc.rust-lang.org/std/marker/trait.Send.html)
- [Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)
- [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [Tokio spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)
- [Tokio JoinHandle](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html)
- [Tokio select cancellation](https://docs.rs/tokio/latest/tokio/macro.select.html#cancellation-safety)
- [Tokio blocking work](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
- [C++ task guidance](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#rconc-task)
