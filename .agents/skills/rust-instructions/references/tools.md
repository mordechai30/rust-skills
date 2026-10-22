# Tool and crate use

## Setup

- Run the environment checker before setup; use its exact executable and package names.
- Install only tools needed for the project workflow; add libraries only when the code requires them.
- Choose versions compatible with MSRV, targets, feature policy, and the existing dependency graph.
- Use `cargo install --locked PACKAGE` for Cargo-installed tools; preserve a package-manager installation
  method.
- Review changes before updating manifests or lockfiles; do not copy source examples as required dependencies.

## cargo-audit

- Install the `cargo-audit` executable, not a project dependency; run `cargo audit` at the lockfile root.
- Allow its normal advisory database refresh for a current audit; report stale/offline evidence explicitly.
- Review advisory IDs, affected versions, platform applicability, and fixed versions.
- Update the affected dependency within project constraints and repeat the audit and tests.
- Scope justified advisory exceptions with reasons and expiry/review conditions; do not suppress failures
  globally.
- Keep audit in CI or the established security hook; do not treat a clean database check as proof of
  soundness.
- Read the [official cargo-audit documentation](https://docs.rs/cargo-audit/latest/cargo_audit/).

## Bacon

- Install `bacon`; run `bacon` from the project for live background checking.
- Use `bacon clippy`, `bacon test`, or `bacon nextest` when those jobs exist in the installed configuration.
- Use `bacon --init` only when adding project settings is authorized; review the generated `bacon.toml`.
- Match jobs to the actual workspace, features, targets, and test policy.
- Keep final command-line and CI checks; do not report a running watcher as a completed verification.
- Read [Bacon usage](https://dystroy.org/bacon/).

## cargo-nextest

- Install the `cargo-nextest` executable; run `cargo nextest run --locked` for compiled tests.
- Run `cargo test --doc --locked` separately; do not assume nextest runs rustdoc tests.
- Configure timeouts, heavy-test concurrency, and resource isolation in `.config/nextest.toml` when needed.
- Account for process-per-test behavior, shared fixtures, external services, and cross-test resource
  contention.
- Use retries only under an explicit flaky-test policy; report first-attempt failures instead of hiding them.
- Read [nextest usage](https://nexte.st/) and the [crate documentation][nextest-docs].

## rustfmt

- Install rustfmt for the selected toolchain with `rustup component add rustfmt`.
- Run `cargo fmt --all` while editing and `cargo fmt --all -- --check` in hooks and CI.
- Use `cargo fmt` to infer language/style editions from manifests; preserve explicit project configuration.
- Check formatting details rustfmt cannot enforce, especially comments, naming, and macro-specific layout.
- Do not use formatter skips to disguise unclear code or migrate the style edition without a project need.
- Read [rustfmt configuration](https://github.com/rust-lang/rustfmt) and the [Style Guide][style].

## Clippy

- Install Clippy for the selected toolchain with `rustup component add clippy`.
- Run `cargo clippy --workspace --all-targets --locked -- -D warnings` for a supported default configuration.
- Run separate supported feature/target configurations; avoid invalid feature unions.
- Inspect correctness and performance suggestions; prefer clarity when a shorter style suggestion adds no
  benefit.
- Keep allowances local with a concrete reason; select pedantic/restriction lints individually.
- Do not enable the entire restriction group or automatically accept every lint rewrite.
- Read [Clippy categories and usage](https://doc.rust-lang.org/clippy/).

## rustfix

- Use rustfix through Cargo's `cargo fix`; do not install the `rustfix` library as a standalone executable.
- Use `cargo fix --locked` for compiler suggestions or `cargo clippy --fix` for applicable Clippy suggestions.
- Start from a reviewed working tree; avoid `--allow-dirty` and `--allow-staged` unless those changes are
  deliberate.
- Select supported features/targets, inspect the diff, format, and rerun relevant checks and tests.
- Do not run automatic fixes inside a check-only pre-commit hook or migrate editions as routine lint cleanup.
- Read [cargo fix behavior](https://doc.rust-lang.org/cargo/commands/cargo-fix.html).

## Hyperfine

- Install `hyperfine`; build baseline and candidate binaries before measuring execution.
- Run `hyperfine --warmup 3 --export-json timings.json './baseline input' './candidate input'` for a
  warm-cache
  comparison when both binaries and the input represent the real workload.
- Check output equivalence first; use repeated runs and inspect variation and outliers.
- Declare warm/cold cache policy, setup cost, environment, hardware, and selected metrics.
- Inspect the installed version's `--help` before using shell or metric options; do not assume GitHub main
  matches it.
- Use explicit shell selection for pipelines, redirection, or expansion when supported; quote arguments
  correctly.
- Avoid privileged machine-wide cache resets unless they are required and authorized.
- Read the [official Hyperfine repository](https://github.com/sharkdp/hyperfine).

## flamegraph

- Install the `flamegraph` package; use its `cargo flamegraph` command to sample CPU stack costs.
- Run `cargo flamegraph --bin actual-binary -- actual-arguments` against representative optimized work.
- Enable release source debug information, for example `debug = "line-tables-only"`, in a profiling
  configuration.
- Keep symbols and verify the installed backend: Linux perf; current macOS xctrace; older versions may use
  DTrace.
- Inspect backend permissions and required platform tools before capture; do not infer usability from a binary
  alone.
- Interpret width as the selected sampled cost, not a chronological timeline or automatic allocation measure.
- Reprofile after a change and benchmark separately; account for sampling and instrumentation effects.
- Read [current flamegraph setup](https://github.com/flamegraph-rs/flamegraph) before using the old [0.1.6
  page][fg-old].

## DHAT

- Add `dhat` as an optional project dependency for executable heap profiling, or as a dev dependency for heap
  tests.
- Define a profiling-only feature; gate both the global allocator and profiler setup behind it.
- Enable `dhat::Alloc` as the one global allocator for that profiling build; resolve existing allocator
  conflicts.
- Hold `dhat::Profiler::new_heap()` across the work and drop it normally to write `dhat-heap.json`.
- Run the actual binary with `--release --features dhat-heap` when that is the chosen feature name.
- View the JSON in a local DHAT-compatible viewer; distinguish total allocation volume, peak live heap, and
  end heap.
- Treat allocator counts as heap evidence, not process RSS or copy profiling; account for work outside the
  guard.
- Remeasure timing without DHAT enabled. Serialize heap tests because allocator/profiler state is
  process-global.
- Read [DHAT setup and limits](https://docs.rs/dhat/latest/dhat/).

## tracing and tracing-subscriber

- Add `tracing` where structured spans/events are needed; install a subscriber only in the application
  entrypoint.
- Use `tracing-subscriber` layers and filters to route diagnostics; do not set a global subscriber from a
  library.
- Record request IDs, queue/lock waits, work sizes, errors, and cancellation with explicit fields.
- Use `#[instrument(skip(...))]` to exclude sensitive or expensive arguments, or select fields explicitly.
- Use `#[instrument]` or `.instrument(span)` for futures; do not retain `Span::enter()` guards across
  `.await`.
- Filter diagnostic volume and avoid building expensive field values before checking whether they are enabled.
- Do not interpret span elapsed time as CPU time; include waiting and scheduling in the explanation.
- Read [tracing](https://docs.rs/tracing/latest/tracing/) and [Tokio tracing setup][tokio-tracing].

## tracing-chrome

- Add `tracing-chrome` to a diagnostic build when a cross-thread or async timeline will clarify a bottleneck.
- Compose its layer with the existing subscriber; retain its flush guard until traced work and tasks finish.
- Open the generated JSON in a compatible trace viewer such as Perfetto; correlate spans and events.
- Limit trace volume and measure overhead; do not use an instrumented timeline as a normal-build benchmark.
- Read [tracing-chrome usage](https://docs.rs/tracing-chrome/latest/tracing_chrome/).

## Tokio Console

- Install the `tokio-console` client and add `console-subscriber` to a diagnostic application configuration.
- Enable the selected Tokio version's required tracing support and `--cfg tokio_unstable` for task
  diagnostics.
- Preserve existing Rust flags when adding that cfg; do not replace target or project flags blindly.
- Use `console_subscriber::init()` only when it owns subscriber setup; otherwise compose its layer.
- Keep `tokio` and `runtime` targets at TRACE; avoid compile-time filtering that removes those events.
- Run the client against the instrumented application; keep the diagnostic endpoint local or
  access-controlled.
- Inspect task poll/busy time, wakes, resource waits, and long polls; use a CPU profiler for inner CPU
  hotspots.
- Remove diagnostic configuration from normal performance comparisons; do not require nightly solely for the
  cfg.
- Read [Tokio Console setup](https://github.com/tokio-rs/console).

## oha

- Install `oha` for an HTTP workload that needs load testing; target only an authorized service/environment.
- Run `oha -z 30s -c 20 http://127.0.0.1:8080/actual-path` only after selecting suitable duration and
  concurrency.
- Match routes, payloads, authentication, connection reuse, and request distribution to the actual workload.
- Compare tail latency, status/error counts, throughput, and server resource use; check load-generator
  saturation.
- Collect server tracing and task diagnostics during a separate diagnostic run when needed.
- Read [oha usage](https://github.com/hatoo/oha); treat the transcript's tool-name spelling as transcription
  noise.

## Rayon

- Add `rayon` as a library dependency for substantial CPU data parallelism.
- Use `rayon::prelude::*`, `par_iter`, `par_iter_mut`, or `into_par_iter` with clear ownership.
- Prefer disjoint outputs or worker-local `fold` followed by `reduce`; avoid shared atomics/maps on every
  item.
- Compare sequential and parallel execution across real input sizes and skew; keep small work sequential.
- Use the existing pool or a deliberately sized `ThreadPoolBuilder`; bound async admission before dispatch.
- Do not run synchronous Rayon work directly on async executor workers when it would block them.
- Preserve ordering and numeric contracts; use deterministic partition/merge rules when reproducibility
  matters.
- Read [Rayon](https://docs.rs/rayon/latest/rayon/) and [parallel iterator contracts][rayon-iter].

## DashMap

- Add `dashmap` only when shared concurrent key/value access is required and measured against simpler choices.
- Prefer a `HashMap` for sequential work or local maps with a final merge for independent parallel
  aggregation.
- Keep entry and reference guards in short lexical scopes; drop them before another potentially locking map
  call.
- Do not hold guards across `.await`, reentrant callbacks, or blocking work; follow each method's deadlock
  notes.
- Use entry operations for a single-key update when suitable; do not treat multiple keys as one transaction.
- Measure key skew, shard contention, memory, and tail latency; use `try_get` only with an explicit retry
  policy.
- Enable optional Rayon integration only when actually used; do not wrap a DashMap in an extra mutex by
  default.
- Read [DashMap](https://docs.rs/dashmap/latest/dashmap/) and [method locking contracts][dashmap-api].

[nextest-docs]: https://docs.rs/cargo-nextest/latest/cargo_nextest/
[style]: https://doc.rust-lang.org/style-guide/
[fg-old]: https://docs.rs/crate/flamegraph/0.1.6
[tokio-tracing]: https://tokio.rs/tokio/topics/tracing
[rayon-iter]: https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html
[dashmap-api]: https://docs.rs/dashmap/latest/dashmap/struct.DashMap.html
