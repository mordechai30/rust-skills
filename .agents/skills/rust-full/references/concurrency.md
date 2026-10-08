# Concurrency

## Contents

- [Select the execution model](#select-the-execution-model)  
- [Send and Sync contracts](#send-and-sync-contracts)  
- [Disjoint work before shared state](#disjoint-work-before-shared-state)  
- [Locks and state transitions](#locks-and-state-transitions)  
- [Bounded work and shutdown](#bounded-work-and-shutdown)  
- [Atomic ordering](#atomic-ordering)  
- [Testing concurrent behavior](#testing-concurrent-behavior)  

## Select the execution model

An async `join!` can make progress  
on multiple I/O operations in one task; it does not distribute CPU work across  
cores. Adding tasks does not make a CPU-bound algorithm faster by itself.  

- **Work and lifetime:** Small independent work  
  **Starting point:** Sequential code  
  **Important cost or obligation:** Scheduling overhead can exceed useful work  

- **Work and lifetime:** Finite work borrowing local data  
  **Starting point:** Scoped threads  
  **Important cost or obligation:** Threads join before borrowed data expires  

- **Work and lifetime:** Repeated CPU data parallelism  
  **Starting point:** Existing worker pool; Rayon if appropriate  
  **Important cost or obligation:** Granularity, contention, and reduction semantics  

- **Work and lifetime:** Many outstanding nonblocking I/O operations  
  **Starting point:** Existing async runtime  
  **Important cost or obligation:** Poll fairness, cancellation, and admission limits  

- **Work and lifetime:** A synchronous dependency in an async service  
  **Starting point:** Bounded blocking workers  
  **Important cost or obligation:** Blocking work may outlive task cancellation  

- **Work and lifetime:** Thread-affine API or `!Send` state  
  **Starting point:** Dedicated thread or supported local executor  
  **Important cost or obligation:** Correct construction, use, and destruction thread  


Choose from workload, latency, memory, and deployment constraints. There is no  
universal connection-count threshold where async becomes better. Scoped threads  
create OS threads; they are not a reusable worker pool. For repeated tiny jobs,  
thread creation and joining can dominate.  

## Send and Sync contracts

Auto-trait composition follows the fields and implementation contracts.  
The important distinction is which bounds the access wrapper actually requires:  

- **Type, ignoring allocator-specific bounds:** `&T`  
  **`Send` condition:** `T: Sync`  
  **`Sync` condition:** `T: Sync`  

- **Type, ignoring allocator-specific bounds:** `&mut T`  
  **`Send` condition:** `T: Send`  
  **`Sync` condition:** `T: Sync`  

- **Type, ignoring allocator-specific bounds:** `Box<T>`  
  **`Send` condition:** `T: Send`  
  **`Sync` condition:** `T: Sync`  

- **Type, ignoring allocator-specific bounds:** `Rc<T>`  
  **`Send` condition:** Not `Send`  
  **`Sync` condition:** Not `Sync`  

- **Type, ignoring allocator-specific bounds:** `Arc<T>`  
  **`Send` condition:** `T: Send + Sync`  
  **`Sync` condition:** `T: Send + Sync`  

- **Type, ignoring allocator-specific bounds:** `Cell<T>` , `RefCell<T>`  
  **`Send` condition:** `T: Send`  
  **`Sync` condition:** Not `Sync`  

- **Type, ignoring allocator-specific bounds:** `Mutex<T>`  
  **`Send` condition:** `T: Send`  
  **`Sync` condition:** `T: Send`  

- **Type, ignoring allocator-specific bounds:** `RwLock<T>`  
  **`Send` condition:** `T: Send`  
  **`Sync` condition:** `T: Send + Sync`  

- **Type, ignoring allocator-specific bounds:** `*const T` , `*mut T` , `NonNull<T>`  
  **`Send` condition:** Not automatic  
  **`Sync` condition:** Not automatic  


Moving an exclusive reference to a scoped thread can be safe: its original  
thread no longer accesses the referent independently during that borrow.  
`Sync` for a mutable reference does not permit freely duplicating exclusive  
mutation. Its shared reference exposes only shared operations.  

`Arc<Mutex<T>>` can make shared access possible for a `Send` value that is not  
`Sync` by itself, because the mutex supplies exclusivity. `Arc<RefCell<T>>`  
cannot supply the missing synchronization. A handle's atomic reference count  
protects ownership bookkeeping, not arbitrary payload operations.  

Let the compiler derive these traits where possible. A manual unsafe impl must  
justify payload access, ownership transfer, callbacks, foreign thread affinity,  
and destruction on another thread. Consider future safe methods too. A type  
that looks immutable can still call foreign code with thread-local state.  

To deliberately prevent thread transfer on stable Rust, a private marker such  
as `PhantomData<Rc<()>>` can express the constraint when appropriate. Do not  
copy a nightly negative-impl example into stable production code. Verify the  
auto traits with compile-pass and compile-fail cases.  

## Disjoint work before shared state

The simplest synchronization is often no shared mutation. Partition an output  
buffer into disjoint mutable slices, share immutable inputs, and combine results  
after workers finish. This can eliminate reference-count traffic and lock  
contention while making ownership visible in the types.  

### Example: scoped work with exclusive regions

```rust
use std::thread;

/// Fills two disjoint regions concurrently and joins before returning.
/// A small input may be faster sequentially; this illustrates the borrow model.
fn fill_halves(bytes: &mut [u8]) {
    let mid = bytes.len() / 2;
    let (left, right) = bytes.split_at_mut(mid);
    thread::scope(|scope| {
        scope.spawn(|| left.fill(1));
        scope.spawn(|| right.fill(2));
    });
}
```

A scoped child panic is joined and handled according to the scope/handle  
contract. An ordinary `std::thread::JoinHandle` dropped without joining detaches  
its thread; it does not wait like a scope. Record a policy for worker panics.  

Parallel reductions can change arithmetic behavior. Floating-point addition  
depends on order. Checked integer accumulation can have order-dependent failure  
when intermediate results overflow. Use a defined grouping or a suitable wider  
accumulator when reproducibility matters. Do not claim that a parallel reduction  
is an interchangeable faster version without checking the result contract.  

## Locks and state transitions

A lock protects a particular invariant. State which fields form that invariant  
and keep operations on them together. A locked lookup followed by an unlocked  
decision and a later locked update is not one atomic transaction.  

Keep critical sections short and avoid blocking I/O or callbacks whose behavior  
is outside the lock owner's control. Establish an order for multiple locks.  
Do not hold a read guard while trying to acquire a write guard on the same lock.  
`RwLock` can help with simultaneous substantive reads, but overhead and writer  
fairness are implementation-dependent. Benchmark it against `Mutex` .  

Poisoning is an advisory signal that a panic may have interrupted an invariant.  
It is not a memory-safety mechanism and is not guaranteed to detect every bad  
state. Recover only after checking or restoring the protected invariant; blindly  
using `PoisonError::into_inner` can expose an inconsistent application state.  

### Example: one critical section for check and update

```rust
use std::sync::Mutex;

/// Debits an account while holding one lock over the whole state transition.
/// Poisoning and insufficient funds are reported separately.
fn debit(balance: &Mutex<u64>, amount: u64) -> Result<(), &'static str> {
    let mut guard = balance.lock().map_err(|_| "account state is poisoned")?;
    let next = guard.checked_sub(amount).ok_or("insufficient funds")?;
    *guard = next;
    Ok(())
}
```

In async code, a synchronous guard should usually end before `.await` . An async  
mutex is designed to allow a guard across `.await` , but a long critical section  
still serializes others and can deadlock through dependency cycles. Use that  
ability for an actual resource protocol, not automatically for every data field.  

A snapshot-compute-commit design must consider intervening updates. Include a  
version or compare-and-commit condition if publishing stale work is invalid.  
Alternatively put the state transition in one owner task and send it commands.  

## Bounded work and shutdown

A bounded channel limits queued item count. It does not limit item byte size,  
already spawned producers, retained outputs, or work waiting elsewhere. Set  
limits on the whole path: admission, message size, queued work, active jobs,  
blocking jobs, and completed-result retention.  

Choose a full-queue policy: wait, reject, shed replaceable work, or retain only  
the latest state. That policy is part of service behavior. Blocking while holding  
a lock needed by the consumer can deadlock even with a safe bounded channel.  

When using a semaphore, acquire before spawning if the number of waiting tasks  
must also be bounded. Move the permit into the worker so it covers the full work  
lifetime. A semaphore acquired inside every spawned task limits active work but  
can still leave an unbounded number of waiting tasks and captured allocations.  

For channels, distinguish delivery contracts:  

- A work queue delivers each item to one consumer.  
- A broadcast channel can report lag and drop old messages for slow receivers.  
- A latest-value channel intentionally skips intermediate values.  
- A one-shot reply can disappear if its receiver is canceled.  

Dropping all senders ends a receive loop after queued work is consumed. Keeping  
an unused sender alive can make it wait forever. Close input deliberately during  
shutdown, then drain or reject pending work according to policy.  

Shutdown requires tracking completion, not sleeping for an estimated duration.  
Stop admission, signal cooperative cancellation where appropriate, finish or  
abort in-flight work by policy, join workers, and explicitly finish fallible  
flush/commit operations. A deadline needs an explicit consequence when it expires.  

## Atomic ordering

An atomic operation gives atomic access to one location. It does not make a  
multi-field invariant atomic or automatically protect pointed-to data.  
Write the synchronization argument before choosing the weakest ordering.  

- **Ordering:** `Relaxed`  
  **Useful meaning:** Atomicity and per-location modification ordering  
  **Typical application:** Independent statistics with no payload publication  

- **Ordering:** `Release`  
  **Useful meaning:** Publishes preceding writes through a matching read  
  **Typical application:** Store of a ready flag after initialization  

- **Ordering:** `Acquire`  
  **Useful meaning:** Observes writes published by the release it reads  
  **Typical application:** Load that reads the published ready value  

- **Ordering:** `AcqRel`  
  **Useful meaning:** Both aspects on a read-modify-write  
  **Typical application:** A justified shared-state transition  

- **Ordering:** `SeqCst`  
  **Useful meaning:** Acquire/release aspects plus a total order of SeqCst operations  
  **Typical application:** Algorithms requiring that global order  


Release and acquire synchronize when the acquire reads the relevant release  
value or applicable release sequence. Merely using these names on unrelated  
locations or not observing the published value does not establish the argument.  
`SeqCst` is not a global lock and does not make all relaxed or ordinary accesses  
part of one ordered transaction. Do not use a simplified hardware-reordering  
story as the proof.  

### Example: publication without a raw pointer

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Holds a one-shot publication protocol with an atomic payload.
/// One writer stores the payload before publishing readiness; it never resets it.
struct Published {
    // Stores the value before the readiness publication.
    value: AtomicUsize,
    // Marks completion of the single writer's initialization.
    ready: AtomicBool,
}

/// Returns a payload only after observing its release publication.
/// This example requires one publication and no concurrent reuse of the protocol.
fn observe(state: &Published) -> Option<usize> {
    if state.ready.load(Ordering::Acquire) {
        Some(state.value.load(Ordering::Relaxed))
    } else {
        None
    }
}
```

The writer stores `value` with `Relaxed` , then stores `true` to `ready` with  
`Release` . The acquire read of that `true` makes the earlier payload store  
visible. A production one-time immutable object often needs only `OnceLock` ,  
which avoids implementing this publication protocol yourself.  

For `compare_exchange` , success ordering covers the read-modify-write; failure  
ordering covers only the failed load and cannot be `Release` or `AcqRel` .  
`Acquire` on an RMW leaves its store part relaxed; `Release` leaves its load part  
relaxed. A weak compare exchange can fail spuriously and normally belongs in a  
retry loop. Review ABA, wraparound, reclamation, and progress separately from  
ordering. A correct flag does not prove the pointee is still allocated.  

## Testing concurrent behavior

Exercise shutdown with an empty queue, a full queue, slow workers, worker panic,  
receiver closure, and producers still holding senders. Test boundedness as an  
observable count/byte limit, not only successful outputs.  

For custom synchronization, use a small model test with Loom where supported.  
It explores interleavings within its model and configured bounds; it is not an  
unrestricted proof of every compiled execution. Stress tests and tests on weaker  
memory-ordering hardware add evidence but cannot replace the synchronization  
argument. Miri can check memory behavior on executed supported paths.  

Sources: [scoped threads](https://doc.rust-lang.org/stable/std/thread/fn.scope.html) ,  
[Send](https://doc.rust-lang.org/stable/std/marker/trait.Send.html) ,  
[Sync](https://doc.rust-lang.org/stable/std/marker/trait.Sync.html) ,  
[Arc thread safety](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#thread-safety) ,  
[Mutex poisoning](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#poisoning) ,  
[atomic module](https://doc.rust-lang.org/stable/std/sync/atomic/index.html) ,  
[Ordering](https://doc.rust-lang.org/stable/std/sync/atomic/enum.Ordering.html) ,  
[Microsoft concurrency](https://microsoft.github.io/RustTraining/c-cpp-book/ch13-concurrency.html) .
