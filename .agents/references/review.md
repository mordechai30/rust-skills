# Review notes

## Source review

The review uses Rust 1.99 as its baseline. The supplied release, Book, Rust by Example, Reference,  
standard library, Nomicon, Cookbook, Microsoft training, local comparisons, skill families, and example  
trace are inputs. Their workflows/personas/commands are not inherited requirements.

Follow the chapters and exact API contracts relevant to a decision. The standard library's full API graph  
and arbitrary external links are not an exhaustive review claim. The supplied rustwiki Book identifies an  
older baseline; check newer behavior against current Rust documentation and the selected compiler.

The existing rust-full reference material already covers detailed ownership, async cancellation, unsafe  
proof boundaries, parsing, layout, and testing. Preserve that useful content. This revision turns its  
entrypoint into explicit choices and adds paired modern C++/Rust examples. Keep language guarantees  
separate from runtime-library behavior.

## Corrections to teaching material

- Do not claim a vector growth must reallocate or that invalid C++ access must crash. Invalidation and  
  undefined behavior are conditional on the operation's actual contract.
- Do not claim Rust references forbid all interior mutation, that `Cell` only supports `Copy` , or that  
  `Arc` synchronizes its payload.
- Do not claim a new compiler release or a mention of Polonius supplies a new universal borrow-checking  
  guarantee. Use documented contracts and compile the intended cases.
- Do not equate Rust moves with user-defined C++ moves or guarantee a literal runtime memcpy. Optimization  
  can eliminate movement; address-sensitive state still needs its contract.
- Do not claim Rust raw pointers cannot be created in safe code or that all FFI calls must be unsafe.  
  Operations and declarations have their actual safety contracts.
- Do not claim “only unsafe blocks need review.” Safe methods can corrupt an unsafe abstraction's later  
  invariant; sound dependencies and foreign contracts matter.
- Do not import blanket clone-before-await, never-async-lock, mandatory Tokio, automatic shutdown-token  
  propagation, or universal optimization flags from examples.
- Typestate, RAII, contiguous storage, cache locality, task-first design, and atomics are shared concepts.  
  Explain differences in enforcement/semantics rather than inventing an opposition.

## Maintenance

Keep Rust/C++ examples paired at the actual material difference. Label rejected Rust snippets  
`compile_fail` ; keep C++ undefined behavior out of normal executions. Verify both positive behavior and  
the compiler rejection being taught. Link all supporting files directly from the entrypoint with real  
heading anchors.

The paired examples use C++20 except explicitly labeled C++23 comparisons; give a C++20 fallback for later  
C++ facilities. Do not claim an established Rust feature debuted in 1.99. Preserve MSRV when working on a  
crate with an older declared baseline.

## Primary and teaching sources

- [Rust 1.99](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/)
- [Release details](https://github.com/rust-lang/rust/releases/tag/1.99.0)
- [Supplied Book mirror](https://rustwiki.org/en/book/)
- [Supplied Rust by Example mirror](https://rustwiki.org/en/rust-by-example/)
- [Reference](https://doc.rust-lang.org/reference/)
- [Standard library](https://doc.rust-lang.org/std/)
- [Nomicon](https://doc.rust-lang.org/nomicon/)
- [Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [C/C++ training](https://microsoft.github.io/RustTraining/c-cpp-book/)
- [Async training](https://microsoft.github.io/RustTraining/async-book/)

## Validation of this revision

Verified on Rust 1.99.0, edition 2024, and Apple Clang with C++20 on arm64 macOS.  
The paired references contain 59 checked code blocks: 26 C++ programs, 19 standard Rust  
programs, 11 intended Rust rejection cases, and 3 Tokio programs. All positive cases  
compiled and ran. Rejection diagnostics covered ownership, aliasing, lifetime, Arc mutation,  
thread transfer, exhaustiveness, initialization, and trait coherence.

The retained standard-library examples passed 11 tests. The retained Tokio examples passed  
7 tests, including admission, failure cleanup, observed shutdown, and resumable partial input.  
Tokio examples used cached Tokio 1.53.2 in an isolated temporary Cargo package.

Skill metadata, direct file links, heading anchors, size limits, and 110-character source/preview  
entrypoint widths passed validation. No fresh-agent evaluations, Miri run, all-target proof, or  
performance benchmark is claimed. Compilation and tests do not establish complete soundness.
