<!-- Copyright (c) Microsoft Corporation. Licensed under the MIT license. -->

# Checklist

- **Universal**
  - [ ] Follow the upstream guidelines ([M-UPSTREAM-GUIDELINES])
  - [ ] Use static verification ([M-STATIC-VERIFICATION])
  - [ ] Lint overrides should use `#[expect]` ([M-LINT-OVERRIDE-EXPECT])
  - [ ] Public types are Debug ([M-PUBLIC-DEBUG])
  - [ ] Public types meant to be read are Display ([M-PUBLIC-DISPLAY])
  - [ ] If in doubt, split the crate ([M-SMALLER-CRATES])
  - [ ] Names are free of weasel words ([M-WEASEL-WORDS])
  - [ ] Names of items are short ([M-SHORT-NAMES])
  - [ ] Prefer regular over associated functions ([M-REGULAR-FN])
  - [ ] Magic values are documented ([M-DOCUMENTED-MAGIC])
  - [ ] Use structured logging with message templates ([M-LOG-STRUCTURED])
- **Library / Interoperability**
  - [ ] Types are Send ([M-TYPES-SEND])
  - [ ] Native escape hatches ([M-ESCAPE-HATCHES])
  - [ ] Don't leak external types ([M-DONT-LEAK-TYPES])
  - [ ] Items come from their original crate ([M-FOREIGN-REEXPORTS])
  - [ ] Accept `impl AsRef<>` where feasible ([M-IMPL-ASREF])
  - [ ] Accept `impl RangeBounds<>` where feasible ([M-IMPL-RANGEBOUNDS])
  - [ ] Accept `impl 'IO'` where feasible ('sans IO') ([M-IMPL-IO])
- **Library / UX**
  - [ ] Abstractions don't visibly nest ([M-SIMPLE-ABSTRACTIONS])
  - [ ] Avoid smart pointers and wrappers in APIs ([M-AVOID-WRAPPERS])
  - [ ] Prefer types over generics, generics over dyn traits ([M-DI-HIERARCHY])
  - [ ] Errors are canonical structs ([M-ERRORS-CANONICAL-STRUCTS])
  - [ ] Canonical error conversion uses `From`, not `map_err` ([M-FROM-ERROR])
  - [ ] Complex type construction has builders ([M-INIT-BUILDER])
  - [ ] Complex type initialization hierarchies are cascaded ([M-INIT-CASCADED])
  - [ ] Services are Clone ([M-SERVICES-CLONE])
  - [ ] Essential functionality should be inherent ([M-ESSENTIAL-FN-INHERENT])
  - [ ] Modules are balanced in size and scope ([M-BALANCED-MODULES])
  - [ ] Don't define preludes ([M-NO-PRELUDE])
  - [ ] Parameter ordering is consistent ([M-PARAMETER-CONSISTENCY])
  - [ ] Collections implement the appropriate iter traits ([M-COLLECTION-TRAITS])
  - [ ] Functions are `async` over returning a Future ([M-ASYNC-FN])
- **Library / Resilience**
  - [ ] I/O and system calls are mockable ([M-MOCKABLE-SYSCALLS])
  - [ ] Test utilities are feature gated ([M-TEST-UTIL])
  - [ ] Integration tests live under `tests/` ([M-INTEGRATION-TESTS])
  - [ ] Use the proper type family ([M-STRONG-TYPES])
  - [ ] Newtypes guard their invariants ([M-STRONG-TYPES-GUARD])
  - [ ] Builders validate in final `.build()` ([M-BUILD-RESULT])
  - [ ] Don't glob re-export items ([M-NO-GLOB-REEXPORTS])
  - [ ] Avoid statics ([M-AVOID-STATICS])
  - [ ] Production code uses telemetry, not println ([M-LOG-NOT-PRINT])
- **Library / Building**
  - [ ] Libraries work out of the box ([M-OOBE])
  - [ ] Native `-sys` crates compile without dependencies ([M-SYS-CRATES])
  - [ ] Features are additive ([M-FEATURES-ADDITIVE])
- **Macros**
  - [ ] Macros are a last resort ([M-MACRO-LAST-RESORT])
  - [ ] Prefer 'macros by example' over proc macros ([M-EXAMPLE-OVER-PROC])
  - [ ] Macros don't lie about signatures ([M-MACROS-DONT-LIE])
  - [ ] Macros assume main crate ([M-MACRO-MAIN-CRATE])
  - [ ] Pin supporting proc macro crates ([M-MACRO-VERSION-PIN])
  - [ ] Third party items come from hidden `_private` module ([M-MACRO-HELPERS])
  - [ ] Proc macros should have separate impl crate incl. tests ([M-PROC-IMPL])
  - [ ] Proc macros don't produce implied or hidden items ([M-PROC-IMPLIED-ITEMS])
- **Applications**
  - [ ] Use mimalloc for apps ([M-MIMALLOC-APPS])
  - [ ] Applications may use Anyhow or derivatives ([M-APP-ERROR])
  - [ ] Applications target highest viable target-cpu ([M-TARGET-CPU])
- **FFI**
  - [ ] Isolate DLL state between FFI libraries ([M-ISOLATE-DLL-STATE])
  - [ ] Business logic belongs in core crates, FFI only translates ([M-FFI-TRANSLATES])
  - [ ] FFI crates follow established naming conventions ([M-FFI-NAMING])
- **Correctness**
  - [ ] Unsafe needs reason, should be avoided ([M-UNSAFE])
  - [ ] Unsafe implies undefined behavior ([M-UNSAFE-IMPLIES-UB])
  - [ ] All code must be sound ([M-UNSOUND])
  - [ ] Panic means 'stop the program' ([M-PANIC-IS-STOP])
  - [ ] Detected programming bugs are panics, not errors ([M-PANIC-ON-BUG])
  - [ ] Panic continuation is last resort ([M-PANIC-CONTINUATION])
  - [ ] Custom panics have a helpful message ([M-PANIC-MESSAGE])
- **Performance**
  - [ ] Optimize for throughput, avoid empty cycles ([M-THROUGHPUT])
  - [ ] Identify, profile, optimize the hot path early ([M-HOTPATH])
  - [ ] Long-running tasks should have yield points ([M-YIELD-POINTS])
  - [ ] Reuse allocations where possible ([M-MEM-REUSE])
  - [ ] Library telemetry does not tank performance ([M-LOG-OVERHEAD])
  - [ ] Nested type hierarchies should avoid needless indirection ([M-AVOID-INDIRECTION])
  - [ ] Use boxed slices and strings for immutable owned sequences ([M-BOX-DST])
  - [ ] Shrink collections to fit after building ([M-SHRINK-TO-FIT])
  - [ ] Use a fast hasher where possible ([M-FAST-HASHER])
  - [ ] Collections are created with sufficient initial capacity ([M-INITIAL-CAPACITY])
  - [ ] Hot `async` functions reduce stack size ([M-ASYNC-STACK-SIZE])
- **Project**
  - [ ] Common settings come from the workspace Cargo.toml ([M-CARGO-WORKSPACE])
  - [ ] The workspace lists and versions all crates ([M-CRATES-IN-WORKSPACE])
  - [ ] All crates are siblings in one folder ([M-CRATES-FLAT-FOLDER])
  - [ ] New crates target latest edition ([M-LATEST-EDITION])
  - [ ] MSRV is conservatively updated ([M-MSRV])
- **Documentation**
  - [ ] First sentence is one line; approx. 15 words ([M-FIRST-DOC-SENTENCE])
  - [ ] Has comprehensive module documentation ([M-MODULE-DOCS])
  - [ ] Documentation has canonical sections ([M-CANONICAL-DOCS])
  - [ ] Mark `pub use` items with `#[doc(inline)]` ([M-DOC-INLINE])
- **AI**
  - [ ] Design with AI use in mind ([M-DESIGN-FOR-AI])
  - [ ] Items are only visible through one path ([M-SINGLE-ITEM-PATH])
  - [ ] Avoid meta design documentation ([M-NO-META-DESIGN-DOCUMENTATION])
  - [ ] Tests do not assert ground truth ([M-TAUTOLOGICAL-TESTS])
  - [ ] Rust code solves Rust problems ([M-RUST-SHAPED])

<!-- Universal  -->
[M-UPSTREAM-GUIDELINES]: src/guidelines/universal/M-UPSTREAM-GUIDELINES.md
[M-STATIC-VERIFICATION]: src/guidelines/universal/M-STATIC-VERIFICATION.md
[M-LINT-OVERRIDE-EXPECT]: src/guidelines/universal/M-LINT-OVERRIDE-EXPECT.md
[M-PUBLIC-DEBUG]: src/guidelines/universal/M-PUBLIC-DEBUG.md
[M-PUBLIC-DISPLAY]: src/guidelines/universal/M-PUBLIC-DISPLAY.md
[M-SMALLER-CRATES]: src/guidelines/universal/M-SMALLER-CRATES.md
[M-WEASEL-WORDS]: src/guidelines/universal/M-WEASEL-WORDS.md
[M-SHORT-NAMES]: src/guidelines/universal/M-SHORT-NAMES.md
[M-REGULAR-FN]: src/guidelines/universal/M-REGULAR-FN.md
[M-DOCUMENTED-MAGIC]: src/guidelines/universal/M-DOCUMENTED-MAGIC.md
[M-LOG-STRUCTURED]: src/guidelines/universal/M-LOG-STRUCTURED.md
[M-LOG-NOT-PRINT]: src/guidelines/libs/resilience/M-LOG-NOT-PRINT.md

<!-- Libs -->
[M-TYPES-SEND]: src/guidelines/libs/interop/M-TYPES-SEND.md
[M-DONT-LEAK-TYPES]: src/guidelines/libs/interop/M-DONT-LEAK-TYPES.md
[M-FOREIGN-REEXPORTS]: src/guidelines/libs/interop/M-FOREIGN-REEXPORTS.md
[M-ESCAPE-HATCHES]: src/guidelines/libs/interop/M-ESCAPE-HATCHES.md
[M-STRONG-TYPES]: src/guidelines/libs/resilience/M-STRONG-TYPES.md
[M-STRONG-TYPES-GUARD]: src/guidelines/libs/resilience/M-STRONG-TYPES-GUARD.md
[M-NO-GLOB-REEXPORTS]: src/guidelines/libs/resilience/M-NO-GLOB-REEXPORTS.md
[M-ESSENTIAL-FN-INHERENT]: src/guidelines/libs/ux/M-ESSENTIAL-FN-INHERENT.md
[M-MOCKABLE-SYSCALLS]: src/guidelines/libs/resilience/M-MOCKABLE-SYSCALLS.md
[M-SIMPLE-ABSTRACTIONS]: src/guidelines/libs/ux/M-SIMPLE-ABSTRACTIONS.md
[M-AVOID-WRAPPERS]: src/guidelines/libs/ux/M-AVOID-WRAPPERS.md
[M-DI-HIERARCHY]: src/guidelines/libs/ux/M-DI-HIERARCHY.md
[M-ERRORS-CANONICAL-STRUCTS]: src/guidelines/libs/ux/M-ERRORS-CANONICAL-STRUCTS.md
[M-FROM-ERROR]: src/guidelines/libs/ux/M-FROM-ERROR.md
[M-INIT-BUILDER]: src/guidelines/libs/ux/M-INIT-BUILDER.md
[M-BUILD-RESULT]: src/guidelines/libs/resilience/M-BUILD-RESULT.md
[M-INIT-CASCADED]: src/guidelines/libs/ux/M-INIT-CASCADED.md
[M-SERVICES-CLONE]: src/guidelines/libs/ux/M-SERVICES-CLONE.md
[M-IMPL-ASREF]: src/guidelines/libs/interop/M-IMPL-ASREF.md
[M-IMPL-RANGEBOUNDS]: src/guidelines/libs/interop/M-IMPL-RANGEBOUNDS.md
[M-IMPL-IO]: src/guidelines/libs/interop/M-IMPL-IO.md
[M-BALANCED-MODULES]: src/guidelines/libs/ux/M-BALANCED-MODULES.md
[M-NO-PRELUDE]: src/guidelines/libs/ux/M-NO-PRELUDE.md
[M-PARAMETER-CONSISTENCY]: src/guidelines/libs/ux/M-PARAMETER-CONSISTENCY.md
[M-COLLECTION-TRAITS]: src/guidelines/libs/ux/M-COLLECTION-TRAITS.md
[M-ASYNC-FN]: src/guidelines/libs/ux/M-ASYNC-FN.md
[M-TEST-UTIL]: src/guidelines/libs/resilience/M-TEST-UTIL.md
[M-INTEGRATION-TESTS]: src/guidelines/libs/resilience/M-INTEGRATION-TESTS.md
[M-AVOID-STATICS]: src/guidelines/libs/resilience/M-AVOID-STATICS.md
[M-OOBE]: src/guidelines/libs/building/M-OOBE.md
[M-SYS-CRATES]: src/guidelines/libs/building/M-SYS-CRATES.md
[M-FEATURES-ADDITIVE]: src/guidelines/libs/building/M-FEATURES-ADDITIVE.md

<!-- Apps -->
[M-APP-ERROR]: src/guidelines/apps/M-APP-ERROR.md
[M-MIMALLOC-APPS]: src/guidelines/apps/M-MIMALLOC-APPS.md
[M-TARGET-CPU]: src/guidelines/apps/M-TARGET-CPU.md

<!-- FFI -->
[M-ISOLATE-DLL-STATE]: src/guidelines/ffi/M-ISOLATE-DLL-STATE.md
[M-FFI-TRANSLATES]: src/guidelines/ffi/M-FFI-TRANSLATES.md
[M-FFI-NAMING]: src/guidelines/ffi/M-FFI-NAMING.md

<!-- Correctness -->
[M-UNSAFE]: src/guidelines/correctness/M-UNSAFE.md
[M-UNSAFE-IMPLIES-UB]: src/guidelines/correctness/M-UNSAFE-IMPLIES-UB.md
[M-UNSOUND]: src/guidelines/correctness/M-UNSOUND.md
[M-PANIC-IS-STOP]: src/guidelines/correctness/M-PANIC-IS-STOP.md
[M-PANIC-ON-BUG]: src/guidelines/correctness/M-PANIC-ON-BUG.md
[M-PANIC-CONTINUATION]: src/guidelines/correctness/M-PANIC-CONTINUATION.md
[M-PANIC-MESSAGE]: src/guidelines/correctness/M-PANIC-MESSAGE.md

<!-- Performance -->
[M-HOTPATH]: src/guidelines/performance/M-HOTPATH.md
[M-THROUGHPUT]: src/guidelines/performance/M-THROUGHPUT.md
[M-YIELD-POINTS]: src/guidelines/performance/M-YIELD-POINTS.md
[M-MEM-REUSE]: src/guidelines/performance/M-MEM-REUSE.md
[M-LOG-OVERHEAD]: src/guidelines/performance/M-LOG-OVERHEAD.md
[M-AVOID-INDIRECTION]: src/guidelines/performance/M-AVOID-INDIRECTION.md
[M-BOX-DST]: src/guidelines/performance/M-BOX-DST.md
[M-SHRINK-TO-FIT]: src/guidelines/performance/M-SHRINK-TO-FIT.md
[M-FAST-HASHER]: src/guidelines/performance/M-FAST-HASHER.md
[M-INITIAL-CAPACITY]: src/guidelines/performance/M-INITIAL-CAPACITY.md
[M-ASYNC-STACK-SIZE]: src/guidelines/performance/M-ASYNC-STACK-SIZE.md

<!-- Project -->
[M-CARGO-WORKSPACE]: src/guidelines/project/M-CARGO-WORKSPACE.md
[M-CRATES-IN-WORKSPACE]: src/guidelines/project/M-CRATES-IN-WORKSPACE.md
[M-CRATES-FLAT-FOLDER]: src/guidelines/project/M-CRATES-FLAT-FOLDER.md
[M-LATEST-EDITION]: src/guidelines/project/M-LATEST-EDITION.md
[M-MSRV]: src/guidelines/project/M-MSRV.md

<!-- Docs -->
[M-FIRST-DOC-SENTENCE]: src/guidelines/docs/M-FIRST-DOC-SENTENCE.md
[M-MODULE-DOCS]: src/guidelines/docs/M-MODULE-DOCS.md
[M-CANONICAL-DOCS]: src/guidelines/docs/M-CANONICAL-DOCS.md
[M-DOC-INLINE]: src/guidelines/docs/M-DOC-INLINE.md

<!-- Macros -->
[M-MACRO-LAST-RESORT]: src/guidelines/macros/M-MACRO-LAST-RESORT.md
[M-EXAMPLE-OVER-PROC]: src/guidelines/macros/M-EXAMPLE-OVER-PROC.md
[M-MACROS-DONT-LIE]: src/guidelines/macros/M-MACROS-DONT-LIE.md
[M-MACRO-MAIN-CRATE]: src/guidelines/macros/M-MACRO-MAIN-CRATE.md
[M-MACRO-VERSION-PIN]: src/guidelines/macros/M-MACRO-VERSION-PIN.md
[M-MACRO-HELPERS]: src/guidelines/macros/M-MACRO-HELPERS.md
[M-PROC-IMPL]: src/guidelines/macros/M-PROC-IMPL.md
[M-PROC-IMPLIED-ITEMS]: src/guidelines/macros/M-PROC-IMPLIED-ITEMS.md

<!-- AI -->
[M-DESIGN-FOR-AI]: src/guidelines/ai/M-DESIGN-FOR-AI.md
[M-SINGLE-ITEM-PATH]: src/guidelines/ai/M-SINGLE-ITEM-PATH.md
[M-NO-META-DESIGN-DOCUMENTATION]: src/guidelines/ai/M-NO-META-DESIGN-DOCUMENTATION.md
[M-TAUTOLOGICAL-TESTS]: src/guidelines/ai/M-TAUTOLOGICAL-TESTS.md
[M-RUST-SHAPED]: src/guidelines/ai/M-RUST-SHAPED.md
