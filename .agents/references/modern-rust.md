# Modern Rust and source interpretation

## Contents

- [Baseline and adoption](#baseline-and-adoption)  
- [Rust 1.99 decisions](#rust-199-decisions)  
- [Raw layout metadata](#raw-layout-metadata)  
- [Ownership APIs](#ownership-apis)  
- [Teaching claims that need correction](#teaching-claims-that-need-correction)  
- [Source authority](#source-authority)  
- [Sources](#sources)  

## Baseline and adoption

The source review used Rust 1.99.0 and documentation retrieved on 2026-10-07.  
This is a reviewed baseline, not a requirement to raise every project's MSRV.  
Use an API's stability annotation, the declared MSRV, and supported targets when  
deciding whether to adopt it. Prefer versioned documentation when reproducing a  
past decision; `/stable/` and crate `/latest/` links move with releases.  

Release notes identify changed capability. They do not replace the API's safety  
contract or establish that a feature improves a particular workload. A compact  
decision table captures the changes relevant to this skill:  

## Rust 1.99 decisions

- **Capability or guidance:** C and C-unwind variadic definitions  
  **Useful application:** Implement an ABI that actually needs variadics  
  **Obligation that remains:** Correct promoted argument types, count, lifetime, and target ABI  

- **Capability or guidance:** Raw size/alignment/layout APIs  
  **Useful application:** Query DST metadata without constructing a reference  
  **Obligation that remains:** Metadata validity and representable extent depend on pointee kind  

- **Capability or guidance:** Box NonNull ownership APIs  
  **Useful application:** Transfer/reclaim ownership with a non-null pointer  
  **Obligation that remains:** Exactly one matching owner and allocator/layout contract  

- **Capability or guidance:** Vec NonNull parts APIs  
  **Useful application:** Transfer vector ownership with its pointer, length, and capacity  
  **Obligation that remains:** Initialization, capacity, allocator, and element layout  

- **Capability or guidance:** Owned lossy UTF-8 conversion  
  **Useful application:** Convert owned bytes without first creating a borrowed lossy result  
  **Obligation that remains:** Replacement semantics and actual allocation/reuse behavior  

- **Capability or guidance:** Boxed-array IntoIterator implementations  
  **Useful application:** Iterate owned or borrowed boxed fixed arrays  
  **Obligation that remains:** Choose consumption versus borrowing intentionally  

- **Capability or guidance:** Guidance against reclaiming leaked storage  
  **Useful application:** Replace leak/reclaim patterns with ownership-transfer APIs  
  **Obligation that remains:** A documentation recommendation, not a new language-semantic ban  

- **Capability or guidance:** raw_borrows_via_references lint  
  **Useful application:** Detect needless references immediately converted to raw borrows  
  **Obligation that remains:** A raw borrow does not prove later dereference validity  


Keep release-specific examples separate from broadly compatible patterns. A  
borrowed slice parser need not use a new API merely to look modern.  

## Raw layout metadata

`Layout::for_value_raw` , `size_of_val_raw` , and `align_of_val_raw` avoid requiring  
an actual reference to a value. That can help before allocation or when memory  
is not initialized, provided their documented metadata conditions hold.  

For sized pointees, size and alignment come from the type. For slice tails,  
the length metadata must be initialized and the total dynamic extent must meet  
the function's representability conditions; zero-length tails have a documented  
special case. For trait-object tails, the vtable must be valid and derived  
through the documented mechanism. Extern-type tails have their own limitations.  
Read the exact function contract instead of treating every raw fat pointer as  
valid metadata.  

Do not replace this with forming `&*ptr` and applying `size_of_val` : doing so  
claims reference validity even if only layout metadata is justified. Conversely,  
a valid size query does not prove that the data allocation exists or that its  
elements may be read.  

## Ownership APIs

Use `Box::into_non_null` /`from_non_null` or the established raw-pointer pair when  
ownership leaves and returns to Rust. `NonNull` changes the pointer's type-level  
null guarantee, not the ownership proof. With vector parts, preserve original  
length, capacity, allocator, and element layout. A raw-parts tuple is not a  
portable foreign or persistent representation.  

`String::from_utf8_lossy_owned` and `FromUtf8Error::into_utf8_lossy` fit owned  
input and explicit replacement semantics. A protocol that requires valid UTF-8  
must reject malformed bytes rather than silently replacing them. Check the API  
documentation and measurements before promising that any conversion never  
allocates or always reuses the same buffer.  

The release's `UnsafeCell` access clarification does not grant ordinary shared  
references permission to mutate arbitrary storage or bypass synchronization.  
Allocation growth clarifications likewise do not justify arbitrary pointer  
extension or shrinking. Use the exact allocation and pointer-operation  
contracts rather than extrapolating from a release-note sentence.  

## Teaching claims that need correction

Several useful training and local sources simplify contracts. Apply the  
following corrections when encountering similar advice:  

- Safe Rust relies on sound unsafe implementations, dependencies, FFI, and  
  compiler behavior. “Only inspect unsafe blocks” misses safe methods that  
  corrupt an unsafe abstraction's invariant. Audit the whole proof boundary.  
- Arrays are inline in their owner, not inherently stack allocated. `Copy` does  
  not imply a small copy; `Clone` does not imply allocation or deep copying.  
- `Cell<T>` is not restricted to `Copy` types. The `get` operation has a `Copy`  
  bound, while other operations have different bounds.  
- `&mut T` can be `Send` when `T: Send` . `Arc<T>` requires the appropriate  
  `Send + Sync` bounds; atomic reference counting does not synchronize `T` .  
- A read-heavy workload does not prove an `RwLock` is faster than a mutex.  
  Reader bookkeeping, critical-section size, writer latency, and scheduling  
  determine the tradeoff.  
- Sequentially consistent atomics establish the specified total order for  
  SeqCst operations plus their acquire/release behavior. They do not serialize  
  all program operations or make a multi-field state machine transactional.  
- `TaskTracker::close` is not an admission barrier. Dropping a `JoinHandle`  
  detaches its task. Timeout and shutdown logic must retain completion ownership.  
- A semaphore acquired inside a task does not bound spawned waiting tasks.  
  A bounded item count does not bound retained bytes.  
- Native async trait methods do not automatically guarantee `Send` returned  
  futures. A callable's `Send` property is distinct from its returned future's.  
- `MaybeUninit` , union storage, and padding make blanket statements about all  
  uninitialized bytes misleading. Reading a value still needs type validity.  
- A zero-length foreign buffer may be null by its ABI, but a Rust raw-slice  
  constructor still requires a non-null aligned pointer.  
- Macro hygiene does not prevent double evaluation of supplied expressions.  
  A token expansion that uses `$expr` twice can run it twice.  
- C/C++ examples of invalidated pointers exhibit undefined behavior, not a  
  guaranteed crash. Whether a particular vector growth reallocates is not an  
  invariant on which the Rust API should rely.  
- Negative trait impls, portable SIMD, async iterator interfaces, and other  
  developing features need a current stability check; a training snippet is  
  not sufficient evidence for a stable compiler.  

The Nomicon issue discussions reinforce this distinction between pedagogy and  
contract. Examples include  
[negative impl syntax](https://github.com/rust-lang/nomicon/issues/497) ,  
[SeqCst explanations](https://github.com/rust-lang/nomicon/issues/479) ,  
[dangling terminology](https://github.com/rust-lang/nomicon/issues/521) ,  
[allocation panic assumptions](https://github.com/rust-lang/nomicon/issues/509) ,  
and [uninitialized-data wording](https://github.com/rust-lang/nomicon/issues/435) .  
An issue's existence identifies a review question; its discussion is not a  
replacement for the standard-library or language contract.  

## Source authority

Use the Rust Reference for documented language rules, standard-library rustdoc  
for individual API contracts, Cargo/rustc manuals for build behavior, and the  
release notes for stabilization. The Nomicon supplies deeper unsafe reasoning  
but includes teaching examples and areas under revision. Microsoft training  
supplies migration and async context; resolve simplified claims against the  
relevant primary contract.  

Existing local skills offer useful topics and patterns. They do not impose  
approval workflows, personas, architecture, dependencies, scripts, or commands  
on this skill. Retain an idea only when it fits the task and the authoritative  
Rust sources support its technical premise.  

## Sources

[Rust 1.99 announcement](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/) ,  
[full release notes](https://github.com/rust-lang/rust/releases/tag/1.99.0) ,  
[raw size](https://doc.rust-lang.org/stable/std/mem/fn.size_of_val_raw.html) ,  
[raw alignment](https://doc.rust-lang.org/stable/std/mem/fn.align_of_val_raw.html) ,  
[Layout](https://doc.rust-lang.org/stable/std/alloc/struct.Layout.html) ,  
[Box](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html) , and  
[Vec](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html)  
provide the release-specific evidence.
