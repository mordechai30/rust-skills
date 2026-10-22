# Source review instructions

- Use the supplied Rust sources and local instruction corpus as the basis for these instructions.
- Preserve every original entrypoint section; remove language comparisons from the new artifacts.
- Use exact Rust and crate contracts for semantics; use the Style Guide for formatting and presentation.
- Resolve simplified performance advice against required safety, clarity, portability, and measured behavior.
- Check version-specific documentation before adopting API or command changes.
- Give the user's safety and clarity requirements precedence; retain only compatible source advice.
- Show every conflict found with a source link, the conflicting rule, and its practical consequence.
- Ask the user to resolve each new conflict before adopting affected advice; reuse explicit prior resolutions.
- Distinguish factual defects, incomplete contracts, and conditional tradeoffs; do not label every unsafe
  example or unfamiliar idiom a contradiction.
- Apply the existing FFI exception to sound necessary unsafe; do not request approval merely for its presence.
- Do not infer that a recorded source inventory requires every historical topic or dependency in a new
  project.

## Local inputs

- Treat the original agent instructions and rust-full skill corpus as creation provenance, not runtime inputs.
- Preserve their entrypoint sections and compatible ownership, API, concurrency, safety, and tooling guidance.
- Apply the supplied progressive-disclosure guide to packaging, relative operational links, and width limits.
- Treat the supplied video transcript as provenance for the profiling workflow and named tools.
- Use the bundled operational references during normal skill use; do not reload the original input corpus.

## Style and performance sources

- Check the [Style Guide](https://doc.rust-lang.org/style-guide/) and its complete print edition.
- Check the [Performance Book introduction](https://nnethercote.github.io/perf-book/introduction.html) and
  [Performance Book](https://nnethercote.github.io/perf-book/), including build, profiling, inlining,
  allocation, layout, iterators, bounds, I/O, parallelism, and compile-time chapters.
- Use the [Reintech article][reintech] as supporting optimization ideas; reject its unsafe branch-hint
  example.
- Read [Rust idioms](https://rust-unofficial.github.io/patterns/idioms/) for readable ownership and API
  choices.
- Read [Ian Bull's article](https://ianbull.com/posts/idiomatic-rust/) for compatible idiomatic guidance.
- Select expressions, loops, combinators, and explicit intermediate values by clarity, not line count.
- Use the [video](https://www.youtube.com/watch?v=q3VOsGzkM-M) through the supplied transcript, not video
  playback.
- Do not adopt universal inlining, allocator, CPU, panic, or layout settings from an example.
- Do not claim all tool backends work on every platform, Rust compile times are universally fast, or total
  allocated bytes equal peak resident memory.

## Tool sources

- Check [cargo-audit](https://docs.rs/cargo-audit/latest/cargo_audit/), [Bacon](https://dystroy.org/bacon/),
  [nextest](https://nexte.st/), and [nextest rustdoc](https://docs.rs/cargo-nextest/latest/cargo_nextest/).
- Check [rustfmt](https://github.com/rust-lang/rustfmt), [Clippy](https://doc.rust-lang.org/clippy/), and
  [cargo fix](https://doc.rust-lang.org/cargo/commands/cargo-fix.html) for formatting, linting, and rustfix
  usage.
- Check the [supplied flamegraph release](https://docs.rs/crate/flamegraph/0.1.6) and
  [current upstream](https://github.com/flamegraph-rs/flamegraph) for backend differences.
- Use the [official Hyperfine repository](https://github.com/sharkdp/hyperfine), as clarified by the user,
  instead of relying on the inaccessible supplied crates.io version page.
- Check [DHAT](https://docs.rs/dhat/latest/dhat/), [tracing](https://docs.rs/tracing/latest/tracing/),
  [Tokio tracing](https://tokio.rs/tokio/topics/tracing), and [Console](https://github.com/tokio-rs/console).
- Check [tracing-chrome](https://docs.rs/tracing-chrome/latest/tracing_chrome/) and
  [oha](https://github.com/hatoo/oha) for the additional tools named in the transcript.
- Check [Rayon](https://docs.rs/rayon/latest/rayon/) and [DashMap](https://docs.rs/dashmap/latest/dashmap/)
  plus the exact method contracts used by the implementation.

## Review limits

- Treat the source review as task-focused review of supplied files, manuals, and relevant linked contracts.
- Do not claim exhaustive review of every standard-library item, historical issue, or arbitrary external link.
- Do not reuse prior validation claims as checks of this skill; record only checks actually executed here.
- Treat inventories as provenance; do not inherit an old compiler baseline as a requirement for new projects.

## Source conflicts: resolved by the user

- Apply the user's safety-first and clarity-first resolution to the Reintech and Ian Bull findings below.
- Reject Reintech's use of `unreachable_unchecked` for a merely unlikely branch; a reachable call is undefined
  behavior. Follow the [actual contract][unreachable] rather than treating the function as a likelihood hint.
- Treat Reintech's forced inlining, prefetching, layout, and release-profile examples as conditional options;
  do not introduce unsafe or hidden behavior changes solely because an example uses them.
- Treat Ian Bull's expression, combinator, iterator, and Clippy recommendations as readability options;
  retain explicit control flow when clearer without a material performance cost.
- Do not label the Rust API contract itself a conflict; distinguish a documented unsafe operation from an
  incorrect use of that operation in another source.

## Source conflicts: awaiting specific resolution

- Use this section for new safety or clarity conflicts that need user resolution; none remain for the
  reviewed findings below under the clarified policy.
- Show each new conflict and ask for resolution before adopting its advice; retain prior explicit decisions.

## Reviewed FFI and documentation findings

- Apply the clarified exception: keep sound necessary FFI, correct defective examples, and preserve useful
  advice; do not treat the presence of unsafe as a reason to reject a source.
- Review [FFI error examples][ffi-errors]: the safe exported function dereferences a supplied pointer based
  on an assumed lifetime. Require caller validity/aliasing obligations and a correctly marked boundary.
- Review [FFI string output examples][ffi-passing]: signed return counts are converted without checked error
  handling, and NUL-containing output is passed to `CString::new(...).unwrap()`.
- Verify foreign status, bounds, and termination before decoding; distinguish missing contracts and compile
  defects from demonstrated runtime memory errors.
- Correct the same page's lifetime claim: a temporary `CString` in the call argument lives through that
  call; it cannot support foreign retention after the statement. Check [temporary scopes][temporaries].
- Review [documentation initialization][doc-init]: assertions inside an uncalled helper are compile-checked
  but never executed; do not present this as behavioral verification.
- Retain that pattern for explicitly compile-only examples; the source itself documents this limitation.
- Keep the deliberately defective counterexample in [accepting strings][ffi-accepting] labeled as a warning,
  not an implementation. Its recommended example already documents caller obligations.
- Treat the destructor idiom's absolute exit wording in light of its own caveats; do not require destructor
  execution for memory safety. Verify abort behavior against the Rust contract before repeating it.

[reintech]: https://reintech.io/blog/optimizing-rust-code-for-performance-guide
[unreachable]: https://doc.rust-lang.org/std/hint/fn.unreachable_unchecked.html
[ffi-errors]: https://rust-unofficial.github.io/patterns/idioms/ffi/errors.html
[ffi-passing]: https://rust-unofficial.github.io/patterns/idioms/ffi/passing-strings.html
[ffi-accepting]: https://rust-unofficial.github.io/patterns/idioms/ffi/accepting-strings.html
[doc-init]: https://rust-unofficial.github.io/patterns/idioms/rustdoc-init.html
[temporaries]: https://doc.rust-lang.org/reference/destructors.html#temporary-scopes
