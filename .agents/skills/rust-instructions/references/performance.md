# Performance decisions

## Measure, isolate, optimize

- Specify the required workload and performance target before changing architecture.
- Measure baseline wall time, throughput, tail latency, CPU, heap allocation, or memory according to the
  problem.
- Use representative optimized builds; record compiler, features, target, hardware, input, and cache policy.
- Isolate the limiting cost with the appropriate profiler; do not optimize merely because a pattern looks
  expensive.
- Change one material factor, verify equivalent outputs and failures, then repeat the same measurement.
- Keep the clearer code when differences are within noise or immaterial to the requirement.
- Report workload-specific evidence and limitations; do not reuse example speedup percentages as expectations.

## Algorithms, allocation, and I/O

- Choose an appropriate algorithm and collection before tuning individual instructions.
- Parse directly from iterators or borrowed slices when intermediate collections add no required capability.
- Use `Vec` or slices for contiguous sequential work; use ordered or hashed maps according to actual access
  needs.
- Use entry APIs for insert-or-update and lazy error construction when repeated lookup or eager work is
  material.
- Reserve from credible checked estimates; limit untrusted sizes before allocation.
- Reuse buffers when useful, with a policy to release unusually large retained capacity.
- Use buffered I/O for many small operations; reuse line buffers when each line need not escape the loop.
- Flush explicitly when callers must observe errors; do not rely on Drop to report a failed flush.
- Measure total allocated bytes, peak live heap, retained heap, and process RSS as distinct quantities.
- Do not replace randomized hashing for untrusted keys without evaluating collision resistance.

## Clarity and dispatch

- Prefer meaningful names, named intermediate values, explicit validation, and straightforward control flow.
- Use an iterator chain only while its data flow remains clear; use a loop for complex state or early exits.
- Prefer generics or `impl Trait` over `dyn Trait` when types are known and specialization fits the contract.
- Keep bounds minimal and signatures readable; move non-generic work out of large generic bodies when helpful.
- Use dynamic dispatch for required runtime selection, heterogeneity, or measured code-size/build-time
  benefits.
- Compare runtime, binary size, and compilation cost before multiplying monomorphized hot-path variants.

## Inlining

- Let optimization choose inlining first; inspect a measured critical path for missed small-function inlining.
- Use `#[inline]` on suitable small critical functions, especially across crate boundaries, and remeasure.
- Use `#[inline(always)]` only with evidence; check code size, instruction-cache cost, and build time.
- Do not annotate every function or claim inline attributes guarantee a particular generated result.
- Outline infrequent error work with `#[cold]` only when it improves a demonstrated hot path.
- Keep public documentation before attributes and preserve ordinary Rust formatting.

## Copy-on-write

- Use `Cow::Borrowed` for unchanged borrowed data and `Cow::Owned` for transformed or independently owned
  data.
- Use `Cow::to_mut` to promote borrowed data only when mutation occurs; account for the clone cost.
- Use `Cow::into_owned` at a retention boundary when independent lifetime is required.
- Prefer an ordinary borrow or owned value when both variants add unnecessary complexity.
- Use `Arc::make_mut` or `Rc::make_mut` for shared snapshots only when uniqueness and weak-reference behavior
  fit.
- Measure retained owners as well as copies; a small owned field can release a much larger input allocation.

## Layout and parallelism

- Start with contiguous data and safe iteration; prefer partitioned outputs and local reductions.
- Separate field arrays only for a measured field-selective bottleneck; maintain equal-length and identity
  invariants.
- Inspect type sizes on the supported target before boxing rare large variants or narrowing stored indices.
- Do not use packed layout as a general optimization; decode serialized fields with safe byte operations.
- Benchmark parallel granularity and skew; do not assume Rayon plus a shared map is faster than local
  aggregation.
- Preserve floating-point tolerance/order and checked-integer behavior when changing reduction grouping.
- Bound task admission, queued bytes, and retained results; prevent CPU work from blocking async workers.

## Build tuning

- Preserve MSRV, deployment CPU support, panic behavior, security, and reproducibility.
- Keep symbol information for diagnostic builds and use the actual production profile for final comparisons.
- Compare LTO and codegen-unit changes before adoption; include build-time and size costs.
- Use `target-cpu=native` only for an explicitly compatible deployment, not a portable distributed binary.
- Use PGO only with representative training and independent validation workloads.
- Treat panic abort and alternate allocators as behavior/deployment decisions requiring relevant validation.
- Verify feature stability before using nightly options, intrinsics, or experimental APIs.
- Do not use `unreachable_unchecked` as a branch-likelihood hint or debug assertions as safety enforcement.

## Sources

- Check [benchmarking](https://nnethercote.github.io/perf-book/benchmarking.html),
  [inlining](https://nnethercote.github.io/perf-book/inlining.html), and
  [heap allocation](https://nnethercote.github.io/perf-book/heap-allocations.html) for relevant options.
- Check [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) before changing build
  behavior.
- Check [Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html) and
  [inline attributes](https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute)
  for exact API and compiler contracts.
