# Performance

## Contents

- [Start with a cost model](#start-with-a-cost-model)  
- [Allocation and collection choice](#allocation-and-collection-choice)  
- [Data layout and locality](#data-layout-and-locality)  
- [Loops, dispatch, and vectorization](#loops-dispatch-and-vectorization)  
- [Parallel and async costs](#parallel-and-async-costs)  
- [Build tuning](#build-tuning)  
- [Benchmark evidence](#benchmark-evidence)  
- [Sources](#sources)  

## Start with a cost model

Tie an optimization to a workload and a measurable limiting resource. A useful  
hypothesis is “this parser copies the entire payload per message; returning a  
borrowed view will reduce copied bytes while the input owner remains short  
lived.” “Iterators are faster” is not a workload-specific hypothesis.  

Choose the algorithm and representation before adjusting individual  
instructions. Removing an intermediate allocation, reducing repeated work,  
batching I/O, or making access sequential usually has a clearer causal model  
than adding `inline(always)` or unchecked indexing. Measure the actual effect;  
there is no universal minimum percentage improvement.  

Observe wall time, CPU time, allocations, peak live memory, retained capacity,  
cache behavior, contention, and tail latency as appropriate. A faster request  
that retains a large shared input can lower throughput under sustained load.  
Test limits and failure behavior as well as the common path.  

## Allocation and collection choice

- **Requirement:** Dense sequential data  
  **Useful starting point:** `Vec<T>` or slice  
  **Check before choosing:** Growth, element size, cache locality  

- **Requirement:** Fixed number of inline elements  
  **Useful starting point:** `[T; N]`  
  **Check before choosing:** Owner/future/stack size and move cost  

- **Requirement:** Fixed owned sequence after building  
  **Useful starting point:** `Box<[T]>`  
  **Check before choosing:** Conversion cost and loss of spare capacity  

- **Requirement:** FIFO or efficient end operations  
  **Useful starting point:** `VecDeque<T>`  
  **Check before choosing:** Split storage when requiring one slice  

- **Requirement:** Lookup without order requirement  
  **Useful starting point:** `HashMap` /`HashSet`  
  **Check before choosing:** Hash cost, adversarial input, iteration needs  

- **Requirement:** Ordered traversal/range queries  
  **Useful starting point:** `BTreeMap` /`BTreeSet`  
  **Check before choosing:** Node allocation and key comparison cost  

- **Requirement:** Small collections  
  **Useful starting point:** Linear vector or measured specialized container  
  **Check before choosing:** Actual size distribution and extra dependency  


Reserve from a credible bound or size hint, with checked size arithmetic.  
Over-reserving every request can increase memory more than it saves allocator  
calls. Reuse buffers with a maximum retained capacity or eviction policy when  
rare large inputs would otherwise pin memory indefinitely.  

Avoid collect-transform-collect pipelines when one traversal suffices. Borrow  
for inspection and move owned values into outputs when the producer is done.  
Clone deliberately: a `String` clone copies its data, an `Arc` clone updates a  
reference count, and a custom clone can perform arbitrary work. Borrowing can  
also retain an expensive owner, so “never clone” is an equally poor rule.  

Streaming bounds memory only when every stage is bounded. An iterator over a  
gigabyte input is lazy, but the owned gigabyte is still live. An async stream  
that buffers all results before delivery has the same retention problem.  

Hasher selection affects security as well as time. Do not replace randomized  
hashing for externally supplied keys just because a microbenchmark prefers a  
faster deterministic hasher. Document the input trust and collision threat.  

## Data layout and locality

Prefer contiguous storage for sequential processing. `Vec<Box<T>>` adds pointer  
chasing and per-object allocations; it can still fit stable addresses, varied  
sizes, or sparse ownership requirements. An arena trades allocation overhead  
for bulk lifetime and retention. An index into owned storage often avoids  
self-referential pointers without losing efficient access.  

Array-of-structs works well when processing all fields of each object. Separate  
field arrays can improve bandwidth when loops use only a subset, but complicate  
length invariants and mutation. Validate equal lengths once at the boundary and  
maintain that relationship in the owning type.  

Default Rust layout permits field reordering. Inspect `size_of` , `align_of` , and  
profiled cache behavior; do not assume declaration order is the memory order.  
For ABI-stable layout, use the documented representation and accept the  
compatibility commitment. `repr(packed)` can create unaligned accesses and is  
rarely an appropriate general optimization.  

False sharing occurs when independent writes contend on the same cache line.  
First partition work or aggregate per-worker results. Padding hot counters can  
help when measurements show the effect, but cache-line size and workload vary.  
An atomic shared counter on every item can erase the gain from parallelism.  

## Loops, dispatch, and vectorization

Iterator adapters can compile to the same tight loops as indexing. Give the  
optimizer clear bounds with slices, exact chunks, and zipped iteration after  
checking equal lengths. `zip` truncates to the shorter input; performance does  
not excuse silently dropping data.  

```rust
/// Applies an explicit modular addition to equal-length inputs.
/// Iterator bounds remain safe and all elements are processed exactly once.
fn add_in_place(left: &mut [u32], right: &[u32]) -> Result<(), &'static str> {
    if left.len() != right.len() {
        return Err("length mismatch");
    }
    for (left, right) in left.iter_mut().zip(right) {
        *left = left.wrapping_add(*right);
    }
    Ok(())
}
```

Only consider unchecked access after profiling or generated-code inspection  
shows a meaningful remaining check and a reviewable invariant proves each index.  
Do not put the only bound proof in a debug assertion: release builds can remove  
it. Safe iteration may expose the bounds more effectively than raw pointers.  

Generics allow specialization by concrete type through monomorphization, which  
can enable inlining but increase binary size and compile time. Trait objects  
permit runtime heterogeneity with indirect calls; they can reduce code size and  
still fit coarse operations well. Choose dispatch granularity from the hot  
path rather than banning `dyn` or boxing globally.  

Start SIMD work with straightforward contiguous loops and inspect vectorization  
before using target intrinsics. `std::arch` intrinsics require the documented  
target feature and memory conditions. Runtime dispatch must guard CPU features  
and provide a supported fallback. A binary built with `target-cpu=native` can  
use instructions unavailable on a deployment machine. Check portable SIMD's  
stability against the actual toolchain; do not label a nightly interface stable.  

## Parallel and async costs

Parallelism costs scheduling, synchronization, extra memory, and changed  
reduction order. Batch enough useful work per task and reuse a pool when work  
repeats. Scoped threads solve borrow lifetimes; they do not automatically pool  
threads. Balance partition size against input skew and CPU/memory bandwidth.  

For reductions, floating-point order can change the result, and checked integer  
intermediate overflow can depend on grouping. Define numerical semantics before  
changing execution order. Use deterministic partition/merge rules when required.  

Async helps multiplex waiting work but does not reduce CPU work automatically.  
Minimize unnecessary task creation, wakeups, tiny messages, and lock acquisitions  
when those costs dominate. Bound work before spawning and batch with a latency  
limit so throughput gains do not cause unbounded response delay.  

## Build tuning

Use a production-representative optimized profile. Cargo's development profile  
is valuable for iteration but not a basis for production performance claims.  
Compare profile settings in controlled builds:  

- Thin or fat LTO may improve cross-crate optimization while increasing link  
  cost. Measure binary size and runtime with the actual dependency graph.  
- Fewer codegen units can improve optimization while slowing builds; do not  
  apply `codegen-units = 1` as a universal requirement.  
- Size optimization can improve instruction-cache behavior or reduce throughput.  
  `opt-level = "z"` is not guaranteed to produce the smallest or fastest result.  
- Panic abort changes recovery and cleanup behavior. Treat it as a product  
  decision, not a free performance switch.  
- Keep enough debug information for profiling and crash diagnosis. Stripping  
  symbols can make evidence harder to collect.  
- PGO needs representative training data, compatible toolchains, and validation  
  against independent workloads. Avoid tuning to a single benchmark fixture.  

Apply flags at the correct Cargo/profile/target scope and check that build  
scripts, dependencies, and tests receive the intended settings. Do not silently  
change portability or error semantics to improve a benchmark.  

## Benchmark evidence

Use realistic size distributions and both common and worst supported shapes.  
Separate setup from the timed operation when production also amortizes setup;  
include it when production pays it per request. Use `std::hint::black_box` where  
needed to prevent irrelevant optimization, while recognizing it is a best-effort  
barrier rather than a model of production input.  

Record toolchain, target, profile, features, input, and hardware. Check output  
equivalence before comparing timing. Repeat enough to distinguish noise and  
consider warm caches, allocator state, CPU frequency, and competing work. Avoid  
reporting a microbenchmark speedup as a whole-application improvement.  

For a memory optimization, measure high-water and retained memory under repeated  
small inputs after a large input. For concurrency, include saturation and  
shutdown under load. Report what changed, why the measurement supports it, and  
the limits of that evidence.  

## Sources

[Cargo profiles](https://doc.rust-lang.org/stable/cargo/reference/profiles.html) ,  
[rustc code-generation options](https://doc.rust-lang.org/stable/rustc/codegen-options/index.html) ,  
[rustc PGO](https://doc.rust-lang.org/stable/rustc/profile-guided-optimization.html) ,  
[standard library](https://doc.rust-lang.org/stable/std/index.html) , and  
[Microsoft C/C++ training](https://microsoft.github.io/RustTraining/c-cpp-book/)  
support the cost and build model. Specific collection and intrinsic APIs remain  
subject to their own contracts.
