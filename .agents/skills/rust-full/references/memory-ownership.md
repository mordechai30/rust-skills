# Memory and ownership

## Contents

- [Ownership at API boundaries](#ownership-at-api-boundaries)  
- [Borrow lifetimes and disjoint access](#borrow-lifetimes-and-disjoint-access)  
- [Shared ownership and interior mutability](#shared-ownership-and-interior-mutability)  
- [Allocation and retained memory](#allocation-and-retained-memory)  
- [Drop and pinning](#drop-and-pinning)  
- [Design review](#design-review)  

## Ownership at API boundaries

Rust moves do not call a user-defined move constructor. Moving a `Vec` or  
`String` transfers  
the owning handle, not all elements in its allocation. Moving a large inline  
array can move many bytes, although the compiler can eliminate the transfer.  
Choose by measured behavior and resource semantics, not a fixed byte threshold.  

`Copy` is a semantic commitment, not a promise that every copy is cheap: a large  
array of `Copy` elements is also `Copy` . `Clone` can allocate, share a  
reference-counted allocation, copy a scalar, or run arbitrary  
code that can panic. Do not equate all cloning with deep copying.  

- **Required operation:** Grow or replace a caller's vector  
  **Useful parameter:** `&mut Vec<T>`  
  **Reason:** Growth and capacity are part of the operation  

- **Required operation:** Borrow or allocate depending on a transformation  
  **Useful parameter:** `Cow<'a, str>`  
  **Reason:** Express conditional ownership  

- **Required operation:** Retain shared immutable data  
  **Useful parameter:** `Rc<T>` or `Arc<T>`  
  **Reason:** Share lifetime responsibility when it is needed  


`&Vec<T>` and `&String` are appropriate when vector or string capacity is itself  
required. Otherwise their narrower input domain adds no useful contract.  
`AsRef` or `Into` can make an API more flexible, but add generics only when  
callers benefit. `AsRef` borrows; `Into` consumes and may allocate during conversion.  

Cloning merely to satisfy a borrow error can conceal the wrong ownership  
boundary. First check whether the consumer only needs a view, whether the  
source is used again, and whether independent ownership is required. A deliberate  
copy may be the right trade-off for a snapshot or an independent long-lived task.  

### Example: allocate only when a transformation needs it

```rust
use std::borrow::Cow;

/// Replaces underscores while preserving unchanged input as a borrow.
/// The owned branch allocates because the output bytes differ.
fn display_name(input: &str) -> Cow<'_, str> {
    if input.contains('_') {
        Cow::Owned(input.replace('_', " "))
    } else {
        Cow::Borrowed(input)
    }
}
```

`Cow::into_owned` clones borrowed data. `Cow::to_mut` clones only when the current  
variant is borrowed. A `Cow` is useful when these branches are meaningful; it  
does not resolve an unexplained lifetime or make retained data free.  

## Borrow lifetimes and disjoint access

Non-lexical lifetimes allow a borrow to end after its last use. Destructors,  
stored references, closure captures, and returned guards can keep it relevant  
longer. A block can make the end of a guard explicit, but does not repair an  
ownership relationship that the program still needs.  

Exclusive-borrow restrictions concern overlapping memory, not a ban on every  
mutation of a composite object. Refactor independent regions before introducing  
interior mutability to work around a borrow that is too broad.  

### Example: split independent fields and slice regions

```rust
/// Applies independent updates to two non-overlapping slice regions.
/// The standard library checks the split and supplies exclusive borrows.
fn mark_regions(bytes: &mut [u8], mid: usize) -> bool {
    let Some((left, right)) = bytes.split_at_mut_checked(mid) else {
        return false;
    };
    left.fill(1);
    right.fill(2);
    true
}
```

Use `split_at_mut` , `split_at_mut_checked` , `chunks_mut` , `iter_mut` , or, when  
available at the MSRV, `get_disjoint_mut` for multiple independent elements.  
These APIs express disjointness that repeated indexing may not prove. A custom  
mutable iterator must never yield access to an element again while a previously  
returned mutable reference remains valid.  

Avoid a self-referential structure that owns a growable buffer and stores  
references into that buffer. Growth, replacement, and moves complicate its  
invariants. Offsets or stable IDs often express the relationship more simply.  
Use generational IDs if removed entries can be replaced; a reused numeric index  
alone does not distinguish old and new objects.  

For parsing, an offset is not necessarily a valid UTF-8 boundary. Keep byte  
offsets separate from character positions. Validate boundaries before turning  
an offset back into `&str` .  

## Shared ownership and interior mutability

- **Type:** `Box<T>`  
  **What it supplies:** Exclusive heap ownership  
  **What it does not supply:** Shared mutation or stable address after moving out  

- **Type:** `Rc<T>`  
  **What it supplies:** Shared ownership in one thread  
  **What it does not supply:** Cross-thread reference-count synchronization  

- **Type:** `Arc<T>`  
  **What it supplies:** Atomic shared ownership  
  **What it does not supply:** Automatic thread safety of `T` or mutation access  

- **Type:** `Cell<T>`  
  **What it supplies:** Replacement through a shared reference  
  **What it does not supply:** Concurrent synchronization or general borrowed access  

- **Type:** `RefCell<T>`  
  **What it supplies:** Runtime-checked borrowing  
  **What it does not supply:** Cross-thread sharing or panic-free borrow conflicts  

- **Type:** `Mutex<T>`  
  **What it supplies:** Synchronized exclusive access  
  **What it does not supply:** Deadlock prevention or application rollback  

- **Type:** `RwLock<T>`  
  **What it supplies:** Multiple readers or one writer  
  **What it does not supply:** Guaranteed fairness or a speed advantage  

- **Type:** `OnceLock<T>`  
  **What it supplies:** Shared initialization once  
  **What it does not supply:** Freedom from recursive-initialization problems  


`Cell::get` needs `Copy` , but `Cell::set` , `replace` , and other operations can work  
with non-`Copy` values. Do not describe `Cell<T>` itself as limited to `Copy` .  
Use `try_borrow` or `try_borrow_mut` when a `RefCell` conflict is a recoverable  
condition. A runtime borrow check is still a real correctness requirement.  

`Arc::clone` adds ownership of the same allocation; it does not create an  
independent snapshot of its contents. `Arc::make_mut` supplies copy-on-write  
semantics, with its own cost and weak-reference behavior. Choose it only when  
that behavior matches the data model.  

Shared immutable data often needs only `Arc<T>` , not `Arc<Mutex<T>>` .  
`Arc<RefCell<T>>` cannot be used to evade `RefCell` 's lack of `Sync` .  
Reference cycles can retain allocations indefinitely. Represent non-owning  
back-references with `Weak` when appropriate; `upgrade` can return `None` .  

## Allocation and retained memory

An array stores its elements inline in its containing allocation. It is not  
inherently stack-allocated: it can be a struct field, boxed value, or static.  
`Vec<T>` keeps elements in a separate contiguous allocation and tracks length  
and capacity; only the initialized prefix belongs to its logical contents.  

Preallocate from a credible bound or distribution. A capacity derived from  
untrusted input needs a resource limit and checked arithmetic. `try_reserve`  
can report allocation failure for that reservation, but later operations can  
allocate too. It is not a universal guarantee that the entire operation is  
fallible rather than aborting.  

Reuse buffers when repeated allocations dominate. `clear` removes values but  
retains capacity. This saves allocation work and can also retain an unusually  
large allocation for the rest of a service's life. Define a retention policy  
from expected request sizes rather than shrinking after every use.  

### Example: bounded, reusable scratch storage

```rust
use std::collections::TryReserveError;

/// Rebuilds scratch storage after the caller has checked its size policy.
/// Reservation is fallible; clearing drops values but keeps the allocation.
fn copy_into_scratch(src: &[u8], dst: &mut Vec<u8>) -> Result<(), TryReserveError> {
    dst.clear();
    dst.try_reserve(src.len())?;
    dst.extend_from_slice(src);
    Ok(())
}
```

For a finished sequence that will not grow, `Box<[T]>` can express fixed length  
and remove the vector's capacity field. It may reduce handle size and spare  
capacity, with a conversion cost to consider. `SmallVec` and compact strings  
are options for measured size distributions, not universal defaults: inline  
storage enlarges every instance, even those that spill to the heap.  

Zero-copy processing reduces copying but can increase retention. A tiny view  
may keep a multi-megabyte shared buffer alive. Copy a small retained field when  
that releases a much larger owner earlier. Measure peak and steady-state memory,  
not only allocation count.  

An arena helps when values share a bulk lifetime. Check whether the chosen arena  
runs destructors, whether objects can refer to each other safely, and whether  
request-scoped data can accidentally escape. Fast allocation does not supply a  
sound lifetime model by itself.  

## Drop and pinning

Rust normally drops locals in reverse declaration order within their drop  
scope. Struct fields drop in declaration order after the type's `Drop::drop` .  
Temporaries have their own scopes, including edition-dependent cases. Lock,  
transaction, and tracing guards can therefore outlive the operation that appears  
to create them. Use explicit scopes for meaningful resource lifetimes.  

Use `mem::take` , `mem::replace` , or `Option::take` to move a value out of a  
borrowed owner while leaving a valid replacement. For domain invariants, check  
that the replacement is a permissible state. An empty default buffer is not  
automatically a valid live connection or initialized transaction.  

Destructor execution is not guaranteed: safe code can leak, abort, or exit.  
Unsafe abstractions must remain memory-safe if a guard is forgotten. RAII  
supports normal resource management; it cannot be the sole proof that a borrowed  
worker has stopped or that a temporarily invalid container has been repaired.  

`Pin<P>` restricts moving the pointee when it is `!Unpin` ; the pointer handle  
can still move. Pinning does not make an object immortal, allocate by itself,  
or stop replacement of arbitrary unpinned fields. The drop guarantee and field  
projection are part of its contract. A pinned `Unpin` value can be moved through  
safe APIs. Prefer `pin!` , `Box::pin` , and maintained projection helpers when  
needed; do not hand-write projection merely to hide a borrow problem.  

## Design review

Trace each retained reference back to its owner and each shared mutation back  
to its synchronization mechanism. Check whether a supposedly cheap handle keeps  
large storage alive, whether `Clone` changes identity or sharing, whether drop  
order affects an invariant, and whether a move or reallocation invalidates raw  
pointers held elsewhere.  

Sources: [pointer types](https://doc.rust-lang.org/stable/reference/types/pointer.html) ,  
[destructors](https://doc.rust-lang.org/stable/reference/destructors.html) ,  
[Cow](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html) ,  
[Vec guarantees](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#guarantees) ,  
[Arc thread safety](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#thread-safety) ,  
[Cell](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html) ,  
[pinning](https://doc.rust-lang.org/stable/std/pin/index.html) ,  
[Microsoft ownership chapter][link-1] .  

[link-1]: https://microsoft.github.io/RustTraining/c-cpp-book/ch07-ownership-and-borrowing.html
