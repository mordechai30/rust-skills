---
name: rust-full
description: >-
  Guide Rust implementation, review, and C++ migration with practical ownership, borrowing,
  concurrency, unsafe, and performance choices. Explain material Rust/C++ differences with examples.
---

# Rust Full

Use stable Rust 1.99 as the reviewed baseline. Preserve the project's edition, MSRV, supported targets,  
runtime, and public contracts. Do not migrate editions or raise MSRV merely to use a newer spelling. Read  
supporting detail only when it governs the decision.

## Memory and ownership

- Borrow for inspection, move for ownership transfer, and clone only for deliberate independent ownership.  
  Pass cheap `Copy` values by value; `Copy` does not guarantee a cheap large array.
- Use values, `[T; N]`, `Vec<T>`, and `String` as owners. Use `Box<T>` for exclusive heap ownership; do  
  not default to raw allocation or raw-pointer ownership.
- Use `&[T]`, `&mut [T]`, and `&str` for immediate borrowed inputs. Use `&mut Vec<T>` when growth is  
  part of the operation. Move owned data into retained tasks.
- Use owning fields and `Drop` for synchronous cleanup. Move from borrowed fields with `take` /`replace`  
  only when the replacement preserves domain validity. Finish fallible/async close explicitly.
- **C++ difference:** Rust moves invalidate non-`Copy` source bindings; they do not run custom move  
  constructors. Borrow lifetimes and exclusivity are enforced in safe Rust.
- Details: [ownership choices](references/memory-ownership.md#ownership-at-api-boundaries).
- Paired examples: [moves](references/cpp-ownership.md#moves-and-copies),  
  [RAII](references/cpp-ownership.md#taking-a-field-and-enforcing-raii),  
  [pinning](references/cpp-ownership.md#pinning-versus-deleting-moves).

## Exclusive access

- Use `&mut T` for exclusive access. Shorten overlapping borrows; split fields/slices with safe APIs  
  before adding interior mutability. Do not use unsafe or cloning as a default borrow-error fix.
- Use `split_at_mut` /checked splits for regions and `get_disjoint_mut` for checked distinct indices. End  
  the borrow before growth or replacement.
- **C++ difference:** two C++ `T&` arguments may alias; two conflicting Rust exclusive borrows are  
  rejected. `&T` is shared access, with explicit interior-mutability exceptions.
- [Exclusive access comparison](references/cpp-ownership.md#exclusive-access).

## Reference lifetime enforcement

- Tie returned/stored borrows to their real owners. Return ownership when the result must escape that  
  lifetime. Do not invent `'static` or build a self-referential growable container by default.
- Use indices/ranges or owned external buffers where simpler; maintain identity and invalidation rules.  
  Copy a small retained field if it would otherwise retain a large allocation.
- **C++ difference:** `span` /`string_view` do not enforce owner lifetime; Rust borrowed slices/text do.  
  Lifetime annotations express relationships, not longer storage duration.
- [Lifetime comparison](references/cpp-ownership.md#reference-lifetime-enforcement).
- [Slices and text](references/slices-strings.md#bounds-and-disjoint-mutation).

## Shared ownership and interior mutability

- Use `Rc<T>` for single-thread sharing and `Arc<T>` for cross-thread shared ownership. Prefer a borrow  
  when sharing lifetime responsibility is unnecessary. Break back-reference cycles with `Weak`.
- Use `Cell` for replacement, `RefCell` for single-thread checked borrowing, and `Mutex` for synchronized  
  access. Use `try_borrow` when conflict is recoverable. Choose `RwLock` from measured behavior.
- Do not treat `Arc` as payload synchronization or an independent snapshot. Do not use `Arc<RefCell<_>>`  
  to bypass `Sync`.
- **C++ difference:** shared C++ ownership can expose writable aliases; Rust shared ownership needs an  
  explicit mutation mechanism. `RefCell` checks outstanding guards at runtime.
- [Sharing comparison](references/cpp-ownership.md#shared-ownership-and-interior-mutability).
- [Detailed choices](references/memory-ownership.md#shared-ownership-and-interior-mutability).

## Concurrency

- **Use task/result APIs over managing a thread and synchronization for each operation.** Use the  
  established async runtime for nonblocking I/O and bounded CPU pool for sustained parallel CPU work.  
  Await directly when independent scheduling is unnecessary.
- Minimize shared writable data: move inputs, share immutable snapshots, partition outputs, and combine  
  results. Do not block executor workers or spawn unbounded work.
- Bound admission before spawning; bound bytes and retained outputs too. Keep handles; inspect readiness  
  with the runtime API and collect results/errors by awaiting. Stop admission on shutdown and observe  
  completion after cancellation.
- End ordinary lock guards before await/callbacks. Use an async guard across suspension only for a  
  deliberate resource contract. Do not invent lock-free structures or manual `Send` /`Sync` impls to make  
  application code compile.
- **C++ difference:** Rust `async fn` returns a lazy future; `std::async(launch::async)` starts work. Rust  
  `Send` /`Sync` enforce transfer/sharing; Tokio handle drop detaches rather than joining.
- [Paired concurrency examples](references/cpp-concurrency.md).
- [Execution choices](references/concurrency.md#select-the-execution-model) and  
  [atomic contracts](references/concurrency.md#atomic-ordering).
- [Async cancellation and supervision](references/async.md#cancellation-is-a-state-transition).

## Unsafe Rust

- Prefer safe APIs. Add unsafe only for a required boundary or demonstrated need. Keep raw ownership  
  inside a reviewed abstraction with matching reclamation.
- Write caller obligations in `# Safety` and exact established facts in `// SAFETY:`. Review safe  
  setters, callbacks, panic paths, and forgotten guards that can affect the proof.
- Do not form invalid references, set vector length before initialization, manufacture lifetimes, or  
  mistake null/size checks for allocation validity.
- **C++ difference:** unchecked Rust operations require an explicit unsafe boundary; it does not disable  
  borrowing/type checks or make surrounding safe mutation irrelevant.
- [Unsafe comparison](references/cpp-safety-types.md#unsafe-is-a-proof-boundary).
- [Proof details](references/unsafe.md#the-proof-boundary) and  
  [FFI](references/ffi-layout.md#design-the-boundary).

## Types, text, and API contracts

- Use validated newtypes/enums for domain states, `Option` for absence, and `Result` for recoverable  
  failures. Propagate with `?`; do not unwrap untrusted failures.
- Use checked size arithmetic and `TryFrom` for narrowing; choose wrapping/saturating behavior explicitly.  
  Use `get` for external ranges and validate UTF-8 before treating bytes as text.
- Use trait bounds for actual operations; choose generic or dynamic dispatch for real callers. Use  
  consuming methods for ownership transitions and the least restrictive needed closure trait.
- **C++ differences:** Rust UTF-8, indexing, overflow, exhaustiveness, consuming methods, nominal traits,  
  and iterator ownership require semantic translation, not syntax substitution.
- [Safety/data comparisons](references/cpp-safety-types.md) and  
  [API/language comparisons](references/cpp-language-model.md).
- [API detail](references/api-correctness.md#trait-and-generic-commitments).

## Performance, tooling, and documentation

- Profile allocation, retention, cache misses, contention, and tail latency. Prefer contiguous data;  
  choose SoA for field-selective hot loops and AoS for whole-object work. Preserve numerical and ownership  
  semantics.
- Prefer safe traversal and buffer reuse before unchecked access. Do not adopt packed layout, SIMD,  
  `target-cpu=native`, or global optimization flags without workload/target evidence.
- Keep the project's Cargo dependency/feature policy. Test supported configurations; `--all-features` is  
  not a valid matrix by itself. Use rustfmt and scoped, justified Clippy allowances.
- Test boundaries, failures, cancellation, shutdown, and promised borrow rejections. Use focused  
  Miri/model/sanitizer checks where supported; passing tools do not prove soundness.
- Document ownership, failures, cancellation, and safety contracts with runnable rustdoc examples. Explain  
  material Rust/C++ differences with paired examples whenever adopting or changing those contracts.
- [Performance decisions](references/performance.md#data-layout-and-locality).
- [Toolchain and verification](references/toolchain-verification.md#verification-by-risk).
- [Rust 1.99 decisions](references/modern-rust.md#rust-199-decisions): check actual API stability against  
  MSRV. New raw APIs are capabilities, not default implementation choices.

## Worked examples and sources

- [Standard-library tests](references/examples-std.rs): parsing, disjoint access, publication, and raw  
  boundaries.
- [Tokio tests](references/examples-async.rs): bounded admission, observed shutdown, and retained partial  
  input.
- [Behavioral evaluation cases](references/evaluations.md): acceptance criteria; do not report unrun  
  agent evaluations as passed.
- [Sources and corrections](references/sources.md): authority, provenance, and source-review limits.
- [Local inventory](references/local-source-inventory.tsv) and  
  [web inventory](references/web-source-inventory.tsv): provenance records, not execution instructions.
