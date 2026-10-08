# Toolchain and verification

## Contents

- [Version and product matrix](#version-and-product-matrix)  
- [Cargo features and dependencies](#cargo-features-and-dependencies)  
- [Style with semantic clarity](#style-with-semantic-clarity)  
- [Verification by risk](#verification-by-risk)  
- [Commands as conditional examples](#commands-as-conditional-examples)  
- [Embedded and no_std boundaries](#embedded-and-no_std-boundaries)  
- [Sources](#sources)  

## Version and product matrix

Distinguish the compiler used today, the package's declared `rust-version` , the  
edition, and the supported target set. Edition changes affect language rules;  
upgrading a compiler does not migrate an edition. New standard-library methods  
can raise MSRV even if the syntax remains valid in an older edition.  

Verify the oldest supported compiler against the intended dependency resolution,  
not only against source syntax. Lockfile format, target-specific dependencies,  
build scripts, proc macros, and enabled features can all affect compatibility.  
Use the repository's lockfile policy and preserve reproducible product builds.  

Document the configurations that represent products: default features, a minimal  
build, each mutually exclusive backend, supported targets, and the release  
profile when it changes behavior. A check on the host does not establish ABI,  
atomic support, alignment, or pointer-width correctness on other targets.  

## Cargo features and dependencies

Cargo feature resolution can unify features requested through dependencies.  
Design additive capabilities where practical and examine which package actually  
enables an optional dependency. Disabling default features in one dependency  
declaration does not necessarily disable defaults enabled elsewhere.  

`--all-features` can enable invalid backend combinations or miss behavior of  
minimal configurations. Use an explicit supported matrix. Resolver versions  
change feature/MSRV resolution behavior; inspect the workspace setting rather  
than assuming the newest package edition controls every dependency.  

Separate normal, build, and development dependencies. A proc macro executes on  
the build host, while its generated code must fit the target and MSRV. A  
`no_std` library can accidentally regain `std` through a dependency's default  
features. Check the resolved feature graph when that boundary matters.  

For an added crate, consider its soundness surface, maintenance, target support,  
license, feature defaults, and runtime coupling in proportion to the task.  
Prefer an established abstraction over a custom unsafe one when it meets the  
requirements. Do not invent a dependency change solely to satisfy a generic  
style preference.  

## Style with semantic clarity

Use the Rust Style Guide and the project's formatter configuration for layout.  
Keep naming tied to units and roles where ambiguity causes defects: distinguish  
byte lengths from element counts, capacity from initialized length, and an  
admission permit from a completion signal. Formatting does not expose these  
invariants by itself.  

Place unsafe justifications beside the operation and public safety obligations  
in the function's `# Safety` documentation. Document meaningful panic,  
cancellation, ownership, and ordering behavior. Avoid comments that merely  
translate syntax or duplicate a method's name.  

For code examples, distinguish intentionally rejected code, pseudocode, and  
complete executable code. A snippet with missing application types is not  
evidence that an API compiles. Use doctests for public examples where feasible,  
including `compile_fail` when the rejected pattern is the point.  

## Verification by risk

- **Change or risk:** Public borrowing/trait API  
  **Useful evidence:** Compile external use and intended rejection cases  
  **Limit:** Compilation alone does not test runtime semantics  

- **Change or risk:** Parser/size handling  
  **Useful evidence:** Truncation, maximum, overflow, invalid encoding, fuzz/property tests  
  **Limit:** Coverage and generator quality matter  

- **Change or risk:** Unsafe abstraction  
  **Useful evidence:** Invariant review, safe-client adversarial tests, Miri where supported  
  **Limit:** No complete soundness proof from a passing tool  

- **Change or risk:** Concurrent state  
  **Useful evidence:** Controlled scheduling, bounded model checking, shutdown under load  
  **Limit:** Unexplored schedules and foreign behavior remain  

- **Change or risk:** Async cancellation  
  **Useful evidence:** Drop at suspension points; resume retained state; inspect task completion  
  **Limit:** Tests must exercise actual partial progress  

- **Change or risk:** Optimization  
  **Useful evidence:** Equivalent outputs and production-shaped benchmark/profile  
  **Limit:** A microbenchmark is not end-to-end performance  

- **Change or risk:** FFI/layout  
  **Useful evidence:** Foreign integration tests on supported ABIs, sizes/offsets where contracted  
  **Limit:** Host-only success misses target differences  


Use tests that probe a contract, not tests that recite the implementation.  
For a disjoint-mutation helper, duplicate and invalid indices matter more than  
asserting which method it calls. For bounded tasks, observe peak active work  
and completed destruction after failure. For buffer initialization, test empty  
and large growth cases without constructing invalid values.  

Treat lint suggestions as evidence to inspect. An allocation lint does not know  
the application's retention requirements. An unused result may be a real lost  
error; a broad allow can hide future defects. Keep justified allowances scoped.  

## Commands as conditional examples

Use the project's existing verification workflow. The examples below concern  
configuration coverage that a successful default build cannot establish:  

```sh
cargo check --locked --no-default-features
cargo test --locked --no-default-features --features selected-backend
cargo check --locked --target supported-target-triple
cargo +declared-msrv check --locked
```

Replace placeholders with actual project values and use only supported  
combinations. `--locked` intentionally fails if the resolution must change;  
dependency maintenance may require a deliberate lockfile update. A library  
compatibility check may also need resolution without relying on its own lockfile  
if consumers resolve dependencies independently.  

When already available and suitable, focused Miri and native sanitizer runs add  
evidence for unsafe code. Do not install or migrate a toolchain silently to make  
a verification command work. State an unavailable check and its practical limit.  
Rustc flags and nightly `-Z` options need their own stability and target checks.  

## Embedded and no_std boundaries

`no_std` removes the standard-library dependency; it does not automatically ban  
allocation, remove panics, or establish real-time behavior. `alloc` requires a  
suitable allocator. Heapless capacity still needs an explicit exhaustion policy.  
Use `core` APIs and examine default features throughout the dependency graph.  

Check target atomic availability before choosing a synchronization design.  
Interrupt masking, critical sections, DMA, and memory-mapped peripherals have  
platform-specific contracts that ordinary thread mutex advice does not supply.  
Volatile access preserves relevant I/O operations but does not establish  
inter-thread synchronization or cache coherence with DMA.  

For hard latency bounds, account for allocation, lock contention, interrupts,  
destructors, and input-dependent algorithms. Rust's memory-safety guarantees do  
not imply a real-time execution bound. Keep platform facts in the target's  
documentation and validate them on the hardware that owns the contract.  

## Sources

[Cargo book](https://doc.rust-lang.org/stable/cargo/index.html) ,  
[features](https://doc.rust-lang.org/stable/cargo/reference/features.html) ,  
[resolver](https://doc.rust-lang.org/stable/cargo/reference/resolver.html) ,  
[rust-version](https://doc.rust-lang.org/stable/cargo/reference/rust-version.html) ,  
[rustc book](https://doc.rust-lang.org/stable/rustc/index.html) , and  
[Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/index.html)  
are the primary build and style authorities.
