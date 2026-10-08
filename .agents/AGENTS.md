# Modern Rust instructions

Use stable Rust 1.99 as the reviewed baseline. Preserve the crate's edition, MSRV, target support,  
runtime, panic policy, and public contracts. Rust 1.99 is a compiler/library release, not an edition. Use  
the Reference and individual API contracts to verify semantics; training and local skills supply  
possibilities, not required architecture or commands.

## Memory and ownership

- Use values and owning containers first: `[T; N]`, `Vec<T>`, `String`, and `Box<T>` for exclusive heap  
  ownership. Do not introduce raw allocation or raw-pointer ownership where these express the contract.
- Borrow for inspection; move for ownership transfer; clone for deliberate independent ownership. Do not  
  clone simply to silence the borrow checker. Pass cheap `Copy` inputs by value.
- Use `&[T]` /`&mut [T]` for bounded immediate input and `&str` for immediate text. Use `&mut Vec<T>`  
  /`&mut String` when resizing is part of the operation. Use owned values for retained work.
- **C++ difference:** Rust non-`Copy` moves invalidate the source binding; C++ moved-from objects remain  
  subject to their type's contract. Rust has no custom move constructor.
- Read [moves and copies](references/cpp-ownership.md#moves-and-copies).

## Exclusive access

- Use `&T` for shared access and `&mut T` for exclusive access. End overlapping borrows before growth or  
  replacement. Use short scopes when guard lifetimes matter.
- Split independent fields or slice regions with safe APIs. Use checked disjoint-element access for  
  external indices. Do not add `RefCell`, cloning, or unsafe simply to evade a broad borrow.
- **C++ difference:** writable C++ references may alias; Rust rejects conflicting overlapping exclusive  
  borrows. Shared Rust access permits mutation only through an appropriate interior-mutability  
  abstraction.
- Read [exclusive access](references/cpp-ownership.md#exclusive-access).

## Reference lifetime enforcement

- Tie returned and stored references to their real owners. Let elision express simple relationships;  
  annotate only the relationships the API needs.
- Return or retain an owned value when a borrow cannot fit the consumer's lifetime. Do not manufacture  
  `'static`. Moving a reference into a task does not extend its validity.
- Prefer offsets/IDs plus owned storage over a self-referential growable buffer. Validate identity after  
  removal/reuse and validate UTF-8 boundaries for text offsets.
- **C++ difference:** Rust checks owner lifetime and invalidating access for safe references/slices. C++  
  `span` /`string_view` retain a programmer-maintained contract. Lifetime annotations do not keep storage  
  alive.
- Read [lifetime enforcement](references/cpp-ownership.md#reference-lifetime-enforcement).

## Shared ownership and interior mutability

- Use `Rc` for single-thread shared lifetime, `Arc` for cross-thread shared lifetime, and `Weak` for  
  non-owning back-references. Prefer borrowing when an additional owner is unnecessary.
- Use `Cell` for replacement through shared access; `RefCell` for single-thread runtime borrow checks;  
  `Mutex` for synchronized access. Use recoverable borrow methods where conflicts are expected.
- Use `RwLock` only when measurements justify it. Do not treat `Arc::clone` as a snapshot or payload lock;  
  `Arc<RefCell<_>>` does not supply cross-thread synchronization.
- **C++ difference:** sharing ownership does not grant Rust writable aliases. Interior mutability is  
  explicit; `RefCell` checks live guards, while C++ `mutable` does not check outstanding references.
- Read [shared ownership choices and comparisons][width-1].

## RAII, destruction, and pinning

- Store resources in owning members. Use `Drop` for synchronous release; provide explicit fallible  
  commit/close and tracked async cleanup.
- Move from borrowed fields with `take` /`replace` only when the replacement is a valid domain state. Keep  
  resource state private and construction validated.
- Do not make memory safety depend on a guard destructor definitely running: safe Rust can leak or forget  
  values. Keep validity during panics and early returns.
- Use `pin!` /`Box::pin` and maintained projection helpers only for a real address-stability contract.  
  Prefer indices or separate owned buffers when simpler.
- **C++ differences:** Rust cannot leave an ordinary borrowed field uninitialized after moving it. Struct  
  fields drop in declaration order; C++ members drop in reverse order. Pinning replaces neither ownership  
  nor lifetime checking.
- Read [RAII and taking fields](references/cpp-ownership.md#taking-a-field-and-enforcing-raii) and  
  [pinning](references/cpp-ownership.md#pinning-versus-deleting-moves).

## Concurrency: tasks first

- **Use task/result APIs instead of managing a thread and synchronization per operation.** Use the  
  established async runtime for nonblocking I/O; use a bounded CPU pool for repeated CPU work. Keep small  
  work sequential when scheduling costs dominate.
- Minimize shared writable data. Move inputs, share immutable snapshots, partition output, and combine  
  results after completion. Do not create custom thread pools or lock-free structures merely to express  
  application work.
- Await directly when independent scheduling is unnecessary. Use joined futures for concurrent I/O; do not  
  mistake async joining for CPU parallelism. Keep blocking and long CPU work off executor workers.
- Bound admission before spawning. Bound bytes, queued work, and completed-result retention too. A bounded  
  message count alone is not a memory budget.
- Retain handles; use runtime readiness inspection and await results. Inspect application failures and  
  task panic/cancellation separately. On shutdown, stop admission, cancel or drain by policy, and observe  
  task termination.
- End ordinary mutex guards before await/callbacks. Hold an async guard across suspension only for a  
  deliberate resource contract. Define poison recovery and avoid stale snapshot commits.
- Keep partial progress in an owner that survives canceled futures. Check operation-specific cancellation  
  safety. Timeout does not roll back external effects; use explicit idempotency/transaction/reconciliation  
  policy where needed.
- Let `Send` /`Sync` derive from sound fields. Do not write unsafe impls just to make spawning compile.  
  For custom atomics, document publication and reclamation; `SeqCst` is not a transaction.
- **C++ differences:** Rust `async fn` is lazy; `std::async(launch::async)` starts work. Rust enforces  
  transfer/sharing traits. Tokio handle drop detaches; cancellation requests are not observed completion.
- Read [concurrency comparisons and task choices](references/cpp-concurrency.md).
- Read [cancellation details](references/async.md#cancellation-is-a-state-transition) and  
  [atomic ordering](references/concurrency.md#atomic-ordering) for those decisions.

## Unsafe Rust

- Prefer safe APIs. Add unsafe only for a required low-level/foreign contract or a demonstrated need. Keep  
  it within a reviewed abstraction.
- Document caller obligations in `# Safety` and exact established facts in `// SAFETY:`. Keep explicit  
  unsafe operations visible; audit safe methods that mutate their invariants too.
- Do not form references before alignment/initialization/lifetime/access are valid. Do not set vector  
  length before initialization, invent `'static`, or reconstruct ownership twice.
- Validate foreign buffer shape and limits; leave uncheckable allocation validity in an explicit contract  
  or established owner. Handle allowed null/empty C buffers before constructing Rust slices.
- Keep validity under panic, cancellation, and forgotten guards. Do not use debug assertions as the only  
  unsafe precondition enforcement.
- **C++ difference:** Rust marks unchecked operations with an unsafe boundary. Borrow/type checks remain  
  active, and the whole safe abstraction must uphold its proof.
- Read [unsafe comparison](references/cpp-safety-types.md#unsafe-is-a-proof-boundary),  
  [proof obligations](references/unsafe.md#the-proof-boundary), and  
  [FFI](references/ffi-layout.md#design-the-boundary).

## Slices, strings, and iterators

- Use safe checked slices/iterators before raw access. Check length arithmetic before ranges, equal  
  lengths before `zip`, and remainders when partial records are invalid.
- Use `String` /`&str` for UTF-8 and byte buffers for arbitrary bytes. Use `get` on external text ranges  
  and reject invalid encoding unless lossy replacement is intended.
- Use `.iter()` to borrow, `.iter_mut()` for exclusive traversal, and `.into_iter()` to consume.  
  Materialize only when ownership or repeated work requires it.
- **C++ differences:** Rust text enforces UTF-8 boundaries; C++ strings store arbitrary bytes. Rust safe  
  indexing checks bounds; consuming Rust iteration invalidates the source owner.
- Read [bounds](references/cpp-safety-types.md#bounds-optional-references-and-nullability),  
  [UTF-8](references/cpp-safety-types.md#utf-8-text-versus-arbitrary-bytes), and  
  [iteration](references/cpp-language-model.md#iteration-consumes-or-borrows).

## Type safety, errors, and public APIs

- Use private validated newtypes and enums for domain states. Preserve validation through deserialization,  
  defaults, setters, and conversions.
- Use `Option` for absence and `Result` for recoverable failures. Propagate with `?` where appropriate. Do  
  not unwrap untrusted/environmental failures or treat every error as retryable.
- Use `TryFrom` for narrowing and checked arithmetic for sizes/offsets. Use wrapping/saturating operations  
  only for those specified semantics.
- Choose trait bounds for actual operations, consuming methods for ownership transitions, and  
  generic/dynamic dispatch for real caller needs. Preserve public lifetime, auto-trait, and future bounds.
- Use functions/generics before macros. If expansion is needed, evaluate input expressions once and  
  preserve hygiene/public paths. Do not assume hygiene proves generated unsafe code correct.
- **C++ differences:** Rust moves/consuming methods, exhaustive matching, nominal traits/coherence, error  
  propagation, overflow, initialization, and macros need semantic translation.
- Read [arithmetic](references/cpp-safety-types.md#arithmetic-and-initialization),  
  [layout](references/cpp-safety-types.md#layout-and-foreign-data), and  
  [API/language comparisons](references/cpp-language-model.md).

## Performance

- Profile the limiting cost: algorithm, allocation, copying, retained memory, cache misses, contention, or  
  tail latency. Compare equivalent behavior in production-shaped optimized builds.
- Prefer contiguous hot data. Use SoA for field-selective loops and AoS for whole-object work. Keep  
  extents and entity identity consistent.
- Reserve from useful estimates within checked budgets. Reuse buffers with a retention policy; copying a  
  small retained field can release a much larger input owner.
- Prefer safe traversal and partitioned reduction before unchecked indexing, atomics per item, padding,  
  SIMD, or packed layout. Preserve numerical order/tolerance where required.
- Do not change target portability, panic behavior, hasher security, or build flags for a microbenchmark  
  without evaluating those contracts.
- Read [performance choices](references/performance.md#data-layout-and-locality). The main layout/cache  
  principles are shared with modern C++; no artificial language contrast is needed.

## Build, tests, naming, and documentation

- Use Cargo as build/package authority; preserve the project's lockfile/dependency policy. Test supported  
  features/targets and declared MSRV. Do not assume `--all-features` covers valid products.
- Use rustfmt and focused Clippy checks. Keep meaningful names for ownership, units, admission, and  
  completion. Scope justified lint allowances.
- Test boundaries, error paths, partial progress, cancellation, shutdown, and intended borrow/trait  
  rejections. Use runnable rustdoc and compile-fail examples for public contracts.
- For unsafe/custom synchronization, review invariants and use focused Miri, sanitizer, or model checks  
  where supported. Passing tests do not prove soundness or all schedules.
- Document ownership, invalidation, failures, cancellation, ordering, and safety where types do not  
  express them. Explain each material Rust/C++ difference with a short paired example when that decision  
  is relevant.
- Read [verification choices](references/toolchain-verification.md#verification-by-risk) and  
  [Rust 1.99 adoption](references/modern-rust.md#rust-199-decisions).
- Read [source interpretation and review](references/review.md#source-review) when checking provenance or  
  conflicting advice.

[width-1]: references/cpp-ownership.md#shared-ownership-and-interior-mutability
