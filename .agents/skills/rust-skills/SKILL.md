---
name: rust-skills
description: >
  Comprehensive Rust coding guidelines with 265 rules across 26 categories.
  Use when writing, reviewing, or refactoring Rust code. Covers ownership,
  error handling, async patterns, concurrency, unsafe code, API design, memory
  optimization, performance, numeric safety, conversions, serde, pattern
  matching, macros, closures, observability, testing, and common anti-patterns.
license: MIT
metadata:
  author: leonardomso
  version: "1.5.1"
  sources:
    - Rust API Guidelines
    - Rust Performance Book
    - Rust 2024 Edition Guide
    - The Rustonomicon
    - ripgrep, tokio, serde, polars, axum, cargo codebases
---

# Rust Coding Guidance

Use the relevant rules for the task. This skill targets Rust 2024; check the crate's edition and MSRV before using version-specific features. Use the project's existing runtime, error strategy, and style.

## Read by task

Read [the rule index](references/rule-index.md) only when you need to choose a rule. Then open the matching file in `rules/`:

- Ownership and errors: `own-`, `err-`.
- Unsafe code and numeric correctness: `unsafe-`, `num-`, `type-`.
- Public APIs and generics: `api-`, `trait-`, `conv-`.
- Async and concurrency: `async-`, `conc-`.
- Performance and memory: `perf-`, `opt-`, `mem-` after measurement.
- Testing, docs, and linting: `test-`, `doc-`, `lint-`.

Apply the safety requirements of the actual code. Use the [safety-critical Rust guidelines](https://github.com/Safety-Critical-Rust-Consortium/safety-critical-rust-coding-guidelines) for safety-critical work and [Linux kernel rules](https://docs.kernel.org/rust/coding-guidelines.html) for kernel code. Treat other rules as recommendations that may have exceptions.
