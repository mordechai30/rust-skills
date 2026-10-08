---
name: rust-full
description: >-
  Develops, reviews, and optimizes modern Rust code with explicit ownership, memory, slice, concurrency,
  async, and unsafe contracts. Use for Rust implementation, API design, C/C++ migration, performance work, or
  soundness review, including FFI and custom low-level abstractions.
---

# Rust Full

Preserve the project's edition, minimum supported Rust  
version (MSRV), target support, runtime, and public compatibility requirements.  
Use newer stable features when they improve the actual design and fit those  
requirements. A newer compiler does not require an edition migration.  

## Working method

Before choosing a representation, identify who owns each resource, who can  
borrow or mutate it, how long it must remain valid, and where work can suspend  
or run concurrently. Treat bounds, allocation limits, cancellation, and panic  
paths as part of the design.  

For performance work, connect each change to a measured cost: allocation,  
retained memory, data movement, cache misses, synchronization, blocking, or  
instruction count. Prefer a better algorithm or safe representation before  
removing checks or introducing raw pointers.  

For unsafe work, distinguish the caller's safety obligations from the  
implementation's proof. A safe API must uphold its unsafe operations for every  
permitted safe use, including panicking callbacks and leaked guards. Small  
unsafe blocks do not make the surrounding safe mutation irrelevant to soundness.  

Read the references selected below in full when they govern the change. Load  
additional topics only when the implementation crosses their boundaries.  

## Memory and ownership

Read [memory and ownership](references/memory-ownership.md) when choosing  
borrowing, ownership transfer, shared ownership, interior mutability, allocation,  
or pinning. Its decision tables explain the costs and obligations.  

- [Ownership at API boundaries](references/memory-ownership.md#ownership-at-api-boundaries)  
- [Borrow lifetimes and disjoint access][link-1]  
- [Shared ownership and interior mutability][link-2]  
- [Allocation and retained memory](references/memory-ownership.md#allocation-and-retained-memory)  
- [Drop and pinning](references/memory-ownership.md#drop-and-pinning)  

Read [slices, strings, and parsing](references/slices-strings.md) for borrowed  
views, mutation of multiple elements, byte parsing, UTF-8 boundaries, or buffer  
reuse. A pointer and a length are not a proof of a valid slice.  

## Concurrency

Read [concurrency](references/concurrency.md) for threads, parallel algorithms,  
shared state, channels, atomic publication, or manual `Send` /`Sync` decisions.  

- [Select the execution model](references/concurrency.md#select-the-execution-model)  
- [Send and Sync contracts](references/concurrency.md#send-and-sync-contracts)  
- [Locks and state transitions](references/concurrency.md#locks-and-state-transitions)  
- [Bounded work and shutdown](references/concurrency.md#bounded-work-and-shutdown)  
- [Atomic ordering](references/concurrency.md#atomic-ordering)  

Read [async execution and cancellation](references/async.md) when any future,  
executor, `.await` , stream, timeout, or spawned task enters the design. Separate  
memory safety from progress, cancellation safety, and application consistency.  

## Unsafe Rust

Read [unsafe Rust](references/unsafe.md) before adding or changing unsafe  
operations, raw-pointer ownership, uninitialized storage, unsafe trait impls,  
packed access, or a safe abstraction around them.  

- [The proof boundary](references/unsafe.md#the-proof-boundary)  
- [Pointer and reference validity](references/unsafe.md#pointer-and-reference-validity)  
- [Initialization and panic safety](references/unsafe.md#initialization-and-panic-safety)  
- [Raw ownership and reclamation](references/unsafe.md#raw-ownership-and-reclamation)  
- [Verification limits](references/unsafe.md#verification-limits)  

Read [FFI and layout](references/ffi-layout.md) for foreign buffers, callbacks,  
exported symbols, C variadics, ABI layouts, or unwinding across language boundaries.  
Do not turn a foreign promise into a safe Rust API without enforcing or owning it.  

## Performance and public design

Read [performance](references/performance.md) for profiling, collection choice,  
data layout, SIMD, dispatch, build tuning, and benchmarks. It gives decision  
criteria rather than universal optimization switches.  

Read [API design and correctness](references/api-correctness.md) for validated  
types, numeric behavior, error contracts, generics, serialization, macros,  
and documentation. Public trait bounds and representation changes are design  
commitments, even when they have no immediate runtime cost.  

Read [toolchain and verification](references/toolchain-verification.md) for  
MSRV, Cargo feature matrices, rustc options, style, doctests, and focused checks.  
Run configurations that represent supported products; `--all-features` is not  
a substitute for a valid feature matrix.  

## Current guidance and source checks

Read [modern Rust and source interpretation](references/modern-rust.md) when  
using release-specific APIs or resolving conflicting advice. The baseline  
review used Rust 1.99.0. Check API stability against the project's actual MSRV;  
`stable` documentation changes over time.  

Read [sources and synthesis notes](references/sources.md) to find authoritative  
manuals, training chapters, local inspiration, and corrections made during  
synthesis. Issue discussions identify disputed guidance; they do not create  
language guarantees.  

Primary source entry points are linked here directly:  

- [Rust 1.99 announcement](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/) and  
  [full release](https://github.com/rust-lang/rust/releases/tag/1.99.0) .  
- [Standard library](https://doc.rust-lang.org/stable/std/index.html) ,  
  [Reference](https://doc.rust-lang.org/stable/reference/index.html) ,  
  [Nomicon](https://doc.rust-lang.org/stable/nomicon/index.html) , and  
  [Nomicon issues](https://github.com/rust-lang/nomicon/issues) .  
- [Cargo](https://doc.rust-lang.org/stable/cargo/index.html) ,  
  [rustc](https://doc.rust-lang.org/stable/rustc/index.html) ,  
  [Style Guide](https://doc.rust-lang.org/stable/style-guide/index.html) , and  
  [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html) .  
- [Microsoft C/C++ training](https://microsoft.github.io/RustTraining/c-cpp-book/)  
  and [async training](https://microsoft.github.io/RustTraining/async-book/) .  

For provenance audits, read the [local-source inventory](references/local-source-inventory.tsv)  
and [web-source inventory](references/web-source-inventory.tsv) . These are  
evidence records, not required implementation workflows.  

## Worked examples and evaluation

The conceptual references contain small examples beside their explanations.  
For complete, executable examples, read these files directly:  

- [Standard-library examples and tests](references/examples-std.rs) : borrowed  
  parsing, conditional allocation, disjoint mutation, scoped workers, atomic  
  publication, raw-buffer validation, and initialization.  
- [Tokio examples and tests](references/examples-async.rs) : bounded admission,  
  tracked shutdown, and cancellation-safe incremental input. These require Tokio  
  with `rt` , `macros` , `sync` , `time` , and `io-util` ; their tests also need `test-util` .  
- [Behavioral evaluation cases](references/evaluations.md) : realistic ownership,  
  concurrency, and unsafe review tasks with observable acceptance criteria.  

When reporting work, distinguish compilation and tests from a soundness proof  
or measured performance result. State the ownership and concurrency choices,  
the checks actually run, and any unverified contract that affects correctness.  

[link-1]: references/memory-ownership.md#borrow-lifetimes-and-disjoint-access
[link-2]: references/memory-ownership.md#shared-ownership-and-interior-mutability
