---
name: rust-instructions
description: >-
  Implement, review, and optimize Rust with clarity-first style, safe ownership,
  measured performance, and toolchain checks. Use for Rust code and project tooling.
---

# Rust Instructions

- Always prefer clarity over brevity unless it materially harms performance.
- Use explicit names, intermediate values, and control flow when they improve understanding, even over a
  shorter idiom. Preserve Rust Style Guide formatting and ownership correctness.
- Try to avoid unsafe Rust as much as possible unless it materially complicates code or dependencies.
- Preserve the project's edition, MSRV, toolchain pin, targets, runtime, and public contracts.
- Ask for clarification when requirements or supplied references are materially unclear or unavailable.
- Give the user's safety and clarity requirements precedence; retain only compatible source advice.
- Show every source conflict found, link the exact advice, explain its effect, and ask the user to resolve it.
- Pause the affected decision until resolution; reuse an explicit prior resolution for the same conflict.
- Read only the supporting reference needed for the current decision. Do not inherit source examples as
  policy.

## Memory and ownership

- Borrow for inspection, move for ownership transfer, and clone for deliberate independent ownership.
- Use values, arrays, `Vec`, `String`, and `Box` before raw allocation or pointer ownership.
- Accept slices and `&str` for immediate views; accept growable containers when resizing is required.
- Use `Cow` for results that can stay borrowed until transformation or mutation requires ownership.
- Read [performance guidance](references/performance.md) for allocation, retention, dispatch, and inlining.

## Exclusive access

- Shorten borrows and split independent fields or slices through safe APIs before adding interior mutability.
- Use checked disjoint access supported by the MSRV. Do not clone or add unsafe merely to bypass borrowing.

## Reference lifetime enforcement

- Tie returned or stored references to their owners; return ownership when a result must escape that lifetime.
- Do not manufacture `'static`. Prefer IDs or offsets with validated identity over self-referential buffers.
- Copy a small retained field when it releases a much larger allocation.

## Shared ownership and interior mutability

- Use `Rc` for one-thread shared ownership, `Arc` for cross-thread ownership, and `Weak` for back-references.
- Use `Cell` for replacement, `RefCell` for checked single-thread borrows, and `Mutex` for synchronized
  access.
- Select `RwLock` from measurements. Do not treat `Arc::clone` as a snapshot or payload synchronization.
- Use `Arc::make_mut` or `Rc::make_mut` only when shared copy-on-write matches the data contract.

## Concurrency

- Use the existing async runtime for nonblocking I/O and a bounded CPU pool for sustained computation.
- Await directly when independent scheduling is unnecessary; keep blocking work off executor workers.
- Partition output and reduce local results before introducing shared mutation, per-item atomics, or DashMap.
- Bound admission before spawning; bound queued bytes and retained outputs as well as task counts.
- Retain handles, inspect failures, stop admission on shutdown, and observe cancellation completion.
- End ordinary lock and DashMap guard scopes before await or callbacks.
- Retain partial progress outside canceled futures; define retry and side-effect recovery explicitly.
- Read [crate and tool usage](references/tools.md) before introducing Rayon or DashMap.

## Unsafe Rust

- Try to avoid unsafe Rust as much as possible unless it materially complicates code or dependencies.
- Use necessary, sound unsafe at FFI boundaries under this exception; do not treat FFI itself as a conflict.
- Prefer a suitable safe wrapper; document pointer, ownership, lifetime, aliasing, and buffer contracts.
- Compare safe alternatives before accepting an exception; record the concrete reason for that exception.
- Keep justified unsafe inside a narrow reviewed abstraction; document `# Safety` and each `// SAFETY:` proof.
- Do not reject an established dependency solely because its sound implementation uses unsafe internally.
- Read [safety guidance](references/safety.md) when implementing unsafe, FFI, pinning, or custom
  synchronization.

## Types, text, and API contracts

- Read [idiom choices](references/idioms.md) when shaping APIs, expressions, or documentation examples.
- Use validated private newtypes and enums, `Option` for absence, and `Result` for recoverable errors.
- Use checked size arithmetic and `TryFrom` for narrowing; validate UTF-8 and external ranges.
- Prefer generics or `impl Trait` over dynamic dispatch when static types meet the requirement clearly.
- Use `dyn Trait` for required runtime heterogeneity or when measured code size and compile costs justify it.
- Use functions and generics before macros; evaluate macro inputs once and preserve public API bounds.

## Performance, tooling, and documentation

- Follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/) and the selected style edition.
- Measure, isolate, optimize, test equivalent behavior, and remeasure with representative optimized builds.
- Inline small critical functions only when evidence supports it; start with `#[inline]`, not blanket forcing.
- Preserve dependency policy and test supported features, targets, MSRV, failures, cancellation, and shutdown.
- Install or integrate applicable pre-commit checks; run rustfmt and Clippy without silently fixing staged
  code.
- Scope justified Clippy allowances when a suggestion reduces clarity without a material performance benefit.
- Document ownership, invalidation, errors, cancellation, and ordering where types do not express them.

## Worked examples and sources

- Use the commands and focused examples in [tool usage](references/tools.md) for the current task.
- Use [performance guidance](references/performance.md) for safe optimization choices and evidence limits.
- Use [safety guidance](references/safety.md) for exceptional low-level boundaries.
- Use [reviewed sources](references/sources.md) to check provenance and resolve conflicting advice.
- Read its source-conflict sections before adopting disputed advice; do not apply unresolved examples.
- Do not report examples, benchmarks, or checks as passed unless they actually ran.

## Tool and dependency checks

- Run `bash scripts/check-rust-environment.sh --project /absolute/project/path` for local inventory.
- Add `--online` to check current stable releases; use `--json` for structured output.
- Use `--project` with the intended Cargo project; omit it for machine-only checks.
- Report missing, broken, outdated, newer-available, and unknown results separately.
- Offer the reported install or update commands. Execute them only when installation or update is authorized.
- Preserve pins, MSRV, features, and lockfile policy. Do not equate a newer release with a compatible update.
- Do not install library crates globally. Add only needed libraries to the correct project dependency section.
- Read [checker operation](references/checks.md) for arguments, exit codes, limitations, and hook integration.
- Do not read script implementations during normal use; read them only when asked to inspect or modify them.

## Use the listed tools and crates

- Use `cargo audit` for RustSec lockfile checks, Bacon for live feedback, and nextest for compiled tests.
- Run `cargo test --doc` separately from nextest; keep CI checks even when Bacon is running.
- Use rustfmt for formatting, Clippy for inspected lint guidance, and `cargo fix` for reviewed rustfix edits.
- Use Hyperfine for end-to-end timing, flamegraph for sampled CPU stacks, and DHAT for heap allocation
  evidence.
- Use tracing with an application-owned subscriber; use tracing-chrome for timelines and Tokio Console for
  tasks.
- Use `oha` for authorized HTTP load tests; measure tail latency and errors as well as throughput.
- Use Rayon for substantial CPU data parallelism and DashMap only for required shared concurrent lookup.
- Read [tool usage](references/tools.md) before setup, profiling, or crate integration.
