# Sources and synthesis notes

## Contents

- [Primary sources](#primary-sources)  
- [Contract-specific sources](#contract-specific-sources)  
- [Training coverage](#training-coverage)  
- [Local inspiration](#local-inspiration)  
- [Corrections and exclusions](#corrections-and-exclusions)  
- [Review limits](#review-limits)  

## Primary sources

The user-supplied web materials are the technical authority for this skill.  
The review baseline is Rust 1.99.0; stable documentation was retrieved on  
2026-10-07 through the requested Jina Reader. Prefer the relevant API contract  
over a generalized teaching statement when they differ.  

- **Source:** [Rust 1.99 announcement](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/)  
  **Contribution to the skill:** Variadic definitions, raw metadata, ownership-transfer guidance, stabilized  
  APIs  

- **Source:** [Rust 1.99 full release](https://github.com/rust-lang/rust/releases/tag/1.99.0)  
  **Contribution to the skill:** Detailed language/library changes and compatibility context  

- **Source:** [Standard library](https://doc.rust-lang.org/stable/std/index.html)  
  **Contribution to the skill:** Per-operation safety conditions, ownership, collections, sync, and memory  
  APIs  

- **Source:** [Style Guide](https://doc.rust-lang.org/stable/style-guide/index.html)  
  **Contribution to the skill:** Formatting and presentation conventions; semantic comments remain  
  task-specific  

- **Source:** [rustc book](https://doc.rust-lang.org/stable/rustc/index.html)  
  **Contribution to the skill:** Codegen settings, lint interpretation, target features, PGO, and  
  profile-sensitive behavior  

- **Source:** [Cargo book](https://doc.rust-lang.org/stable/cargo/index.html)  
  **Contribution to the skill:** Features, resolver, profiles, MSRV, target/build/dependency configuration  

- **Source:** [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)  
  **Contribution to the skill:** Borrowing, lifetime, generic, trait, error, and unsafe examples checked  
  against stronger contracts  

- **Source:** [Rust Reference](https://doc.rust-lang.org/stable/reference/index.html)  
  **Contribution to the skill:** Undefined behavior, pointers, destructors, ABI, type layout, and language  
  rules  

- **Source:** [Microsoft C/C++ book](https://microsoft.github.io/RustTraining/c-cpp-book/)  
  **Contribution to the skill:** Migration tradeoffs, ownership, smart pointers, errors, concurrency, FFI,  
  embedded, and macros  

- **Source:** [Microsoft async book](https://microsoft.github.io/RustTraining/async-book/)  
  **Contribution to the skill:** Future state, pinning, executors, streams, async traits, pitfalls, and  
  production supervision  

- **Source:** [Nomicon](https://doc.rust-lang.org/stable/nomicon/index.html)  
  **Contribution to the skill:** Aliasing, initialization, variance, auto traits, panic safety, leaks, FFI,  
  and raw container reasoning  

- **Source:** [Nomicon issues](https://github.com/rust-lang/nomicon/issues)  
  **Contribution to the skill:** Questions and corrections that need current primary-contract verification  


## Contract-specific sources

The most consequential claims were checked against focused documentation:  

- [Undefined behavior](https://doc.rust-lang.org/stable/reference/behavior-considered-undefined.html)  
  and [memory-model status](https://doc.rust-lang.org/stable/reference/memory-model.html) :  
  unsafe preconditions and the limits of finalized language rules.  
- [Type layout](https://doc.rust-lang.org/stable/reference/type-layout.html) and  
  [destructors](https://doc.rust-lang.org/stable/reference/destructors.html) :  
  field representation, validity, drop behavior, and ABI assumptions.  
- [Raw pointers](https://doc.rust-lang.org/stable/std/ptr/index.html) ,  
  [raw immutable slices](https://doc.rust-lang.org/stable/std/slice/fn.from_raw_parts.html) ,  
  and [raw mutable slices](https://doc.rust-lang.org/stable/std/slice/fn.from_raw_parts_mut.html) :  
  provenance, one-allocation ranges, alignment, extent, and access permissions.  
- [MaybeUninit](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html) ,  
  [UnsafeCell](https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html) , and  
  [Pin](https://doc.rust-lang.org/stable/std/pin/index.html) :  
  initialization, interior mutability, and structural pinning obligations.  
- [Atomic ordering](https://doc.rust-lang.org/stable/std/sync/atomic/enum.Ordering.html) ,  
  [Arc](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html) ,  
  [Mutex](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html) ,  
  [RwLock](https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html) , and  
  [thread scope](https://doc.rust-lang.org/stable/std/thread/fn.scope.html) :  
  publication, trait bounds, poisoning, and borrow-bound parallel work.  
- [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html) ,  
  [resolver](https://doc.rust-lang.org/stable/cargo/reference/resolver.html) ,  
  [rust-version](https://doc.rust-lang.org/stable/cargo/reference/rust-version.html) ,  
  and [profiles](https://doc.rust-lang.org/stable/cargo/reference/profiles.html) :  
  supported configuration and compatibility coverage.  
- [rustc codegen options](https://doc.rust-lang.org/stable/rustc/codegen-options/index.html)  
  and [PGO](https://doc.rust-lang.org/stable/rustc/profile-guided-optimization.html) :  
  build costs, overflow checks, panic strategy, target portability, and evidence.  

The async examples add an explicit Tokio dependency rather than implying these  
contracts are universal executor behavior. The reviewed Tokio docs were 1.53.2  
and Tokio-util 0.7.19:  

- [spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)  
- [select](https://docs.rs/tokio/latest/tokio/macro.select.html)  
- [JoinSet](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html)  
- [spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)  
- [async mutex](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)  
- [TaskTracker](https://docs.rs/tokio-util/latest/tokio_util/task/struct.TaskTracker.html)  

These pages move at `/latest/` ; recheck the installed crate version before  
depending on a changed runtime contract.  

## Training coverage

The Microsoft C/C++ chapters supplied migration ideas across data modeling,  
lifetimes and borrowing, smart pointers and interior mutability, errors,  
generics, iterators, concurrency, unsafe/FFI, `no_std` , case studies, cloning,  
checked indexing, logging, semantic differences, and macros. The synthesis  
retains decisions that need care, not an introductory syntax course.  

The async chapters supplied the state-machine view, pinning, custom future  
polling, executor choice, Tokio operations, alternatives, traits, streams,  
pitfalls, bounded production patterns, architecture, and capstone examples.  
Teaching implementations were examined as explanations; they were not copied  
as a production runtime or supervision framework.  

The Nomicon material supplied deeper proofs for aliasing, borrowing and  
splitting, unbounded lifetimes, variance and markers, drop checking, initialization,  
leaks, exception safety, Send/Sync, atomics, FFI, and vector construction.  
Its raw-vector exercises illustrate invariants, not a reason to replace `Vec` .  

## Local inspiration

The supplied local comparison documents, `rust-cpp.md` and  
`rust_vs_c_comparison.md` , motivated the ownership, slice invalidation,  
concurrency, and unsafe-boundary emphasis. Strong claims were corrected against  
the primary sources: safe APIs depend on sound unsafe internals, invalid C/C++  
access is not a guaranteed crash, and safe mutation can affect a later unsafe  
operation's invariant.  

All skill families in the two requested directories were inventoried. Their  
topic coverage informed this package as follows:  

- **Existing material:** rust_pack general debug  
  **Ideas retained:** Reproduction, evidence, failure-path reasoning  
  **Material not imposed:** Mandatory planning/approval workflows  

- **Existing material:** rust_pack general security  
  **Ideas retained:** Input limits, error handling, unsafe boundary review  
  **Material not imposed:** Unrelated security checklists or architecture  

- **Existing material:** rust_pack general syntax  
  **Ideas retained:** Readable Rust and meaningful contracts  
  **Material not imposed:** Introductory syntax explanations  

- **Existing material:** rust_pack lint-hunter  
  **Ideas retained:** Treat lints as prompts to inspect costs and correctness  
  **Material not imposed:** Blind lint-driven rewrites  

- **Existing material:** rust_pack pest specialist  
  **Ideas retained:** Grammar validation, input shape and bounded parsing  
  **Material not imposed:** Mandatory parser dependency  

- **Existing material:** rust_pack RON specialist  
  **Ideas retained:** Schema, validation, and serialization boundaries  
  **Material not imposed:** Mandatory serialization format  

- **Existing material:** rust_pack router and rust-core  
  **Ideas retained:** Topic selection and relevant supporting resources  
  **Material not imposed:** Router personas or inherited commands  

- **Existing material:** rust-async-patterns  
  **Ideas retained:** Runtime distinctions, Send state, cancellation, bounded work  
  **Material not imposed:** A required runtime or architecture  

- **Existing material:** rust-best-practices and its chapters  
  **Ideas retained:** Ownership APIs, testing, errors, traits, docs, and measured performance  
  **Material not imposed:** Blanket “never clone”, mutex, or optimization rules  

- **Existing material:** rust-skills and its rule/reference resources  
  **Ideas retained:** 265 categorized ideas across ownership, memory, errors, APIs, iterators, async,  
  concurrency, unsafe, macros, tooling, and performance  
  **Material not imposed:** Rule rankings, percentages, scripts, and example code as authority  


Local scripts were inspected as advisory examples, not run as prescribed  
workflows. The full file and external-link inventory is separately available  
from SKILL.md for provenance; it does not become an execution dependency.  

## Corrections and exclusions

The synthesis checks especially addressed `&mut T` Send bounds, `Cell` operation  
bounds, SeqCst limits, empty raw slices, padding and uninitialized storage,  
leaked-guard safety, raw ownership reclamation, native async trait Send promises,  
task detachment, TaskTracker close semantics, and admission before spawning.  

The user approved excluding two unavailable indirect links found in existing  
skills: a docs.rs placeholder (`https://docs.rs/crate`) and the removed Roc file  
at `crates/compiler/solve/src/to_var.rs` . They supply no technical claim in the  
skill. Transient Jina size-budget failures for large manuals were resolved by  
using the bundled Reader script's supported budget option.  

## Review limits

The review used the supplied entry pages, relevant chapters and API contracts,  
the local skill corpus and resources, and selected Nomicon issue discussions.  
It is a task-focused synthesis, not a claim to have read every API item linked  
from the entire standard library or every historical issue. A downloaded page  
is not by itself proof that every claim in that page was adopted or verified.  

Examples were written for the chosen contracts rather than copied wholesale.  
Compilation and behavior tests provide example-level evidence; soundness still  
requires the documented invariant, and performance claims require workload  
measurements. Fresh-agent/model evaluations are specified separately and must  
not be reported as completed unless actually run.
