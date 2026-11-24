# The Rust Performance Book Index

| Chapter | Summary |
| --- | --- |
| [Title Page](src/title-page.md) | Title page and authorship metadata for *The Rust Performance Book* by Nicholas Nethercote and contributors. |
| [Introduction](src/introduction.md) | Overview of performance characteristics covered by the book, including runtime speed, memory usage, and compile times. |
| [Benchmarking](src/benchmarking.md) | Methodology and tools for comparing performance across program versions and avoiding benchmarking pitfalls. |
| [Build Configuration](src/build-configuration.md) | Compiler and Cargo profile settings (`opt-level`, LTO, `codegen-units`, `target-cpu`, PGO, allocators) that improve runtime speed without code changes. |
| [Linting](src/linting.md) | Using Clippy performance lints to detect and fix common suboptimal Rust patterns. |
| [Profiling](src/profiling.md) | Profiling tools and techniques to identify hot code paths, allocations, and cache misses worth optimizing. |
| [Inlining](src/inlining.md) | Reducing function call overhead and enabling cross-function optimizations with `#[inline]` attributes. |
| [Hashing](src/hashing.md) | Speeding up `HashMap` and `HashSet` using alternative hashers and avoiding unnecessary hashing overhead. |
| [Heap Allocations](src/heap-allocations.md) | Minimizing costly heap allocations and deallocations through reuse, stack allocation, `Cow`, `SmallVec`, and capacity pre-allocation. |
| [Type Sizes](src/type-sizes.md) | Shrinking frequently instantiated structs and enums to reduce memory usage and improve cache locality. |
| [Standard Library Types](src/standard-library-types.md) | Performance-oriented APIs and usage patterns for `Vec`, `Option`, `Result`, `Rc`/`Arc`, and other standard library types. |
| [Iterators](src/iterators.md) | Avoiding unnecessary `collect` allocations and writing iterator chains that optimize cleanly. |
| [Bounds Checks](src/bounds-checks.md) | Understanding when slice/vector bounds checks affect hot loops and how to help the compiler elide them safely. |
| [I/O](src/io.md) | Improving I/O throughput by locking stdout/stderr, buffering reads and writes, and minimizing syscalls. |
| [Logging and Debugging](src/logging-and-debugging.md) | Ensuring disabled logging or debug instrumentation does not perform expensive work at runtime. |
| [Wrapper Types](src/wrapper-types.md) | Reducing synchronization and borrow-checking overhead when using wrapper types like `RefCell` and `Mutex`. |
| [Machine Code](src/machine-code.md) | Inspecting generated assembly with Compiler Explorer or `cargo-show-asm` to spot hidden inefficiencies. |
| [Parallelism](src/parallelism.md) | Introducing safe parallelism (e.g., data parallelism with Rayon) to scale compute-bound workloads. |
| [General Tips](src/general-tips.md) | High-level optimization principles: algorithmic complexity, doing less work, and avoiding premature optimization. |
| [Compile Times](src/compile-times.md) | Practical techniques and Cargo/rustc settings for speeding up Rust compilation. |
| [Summary](src/SUMMARY.md) | `mdBook` table of contents for *The Rust Performance Book*. |
