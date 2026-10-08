# API design and correctness

## Contents

- [Represent the invariant](#represent-the-invariant)  
- [Ownership in public signatures](#ownership-in-public-signatures)  
- [Numeric and indexing contracts](#numeric-and-indexing-contracts)  
- [Error and panic boundaries](#error-and-panic-boundaries)  
- [Trait and generic commitments](#trait-and-generic-commitments)  
- [Serialization and macros](#serialization-and-macros)  
- [Sources](#sources)  

## Represent the invariant

Put repeated validation into a constructor and preserve it through private  
fields. A validated length, index, or ID is useful only if it remains valid for  
the object against which it was checked. An index validated against one vector  
does not authorize indexing another vector or the same vector after removal.  
Use a borrow-bound view, a version, or a generational ID when identity changes.  

A newtype should narrow the legal state space rather than merely rename an  
integer. Check all construction paths, including `From` , deserialization,  
default values, setters, and test helpers. Do not derive `Deserialize` directly  
onto an invariant-bearing representation if that bypasses its constructor.  

```rust
/// Stores a nonzero batch size subject to an application-specific upper bound.
/// Private storage prevents callers from bypassing admission validation.
struct BatchSize(std::num::NonZeroUsize);

impl BatchSize {
    /// Validates a batch size against the caller's configured budget.
    /// The bound is supplied by policy rather than embedded as a magic constant.
    fn new(value: usize, maximum: std::num::NonZeroUsize) -> Option<Self> {
        let value = std::num::NonZeroUsize::new(value)?;
        (value <= maximum).then_some(Self(value))
    }

    fn get(&self) -> usize {
        self.0.get()
    }
}
```

Typestate fits a small number of irreversible transitions when compile-time  
tracking removes real misuse. For dynamic workflows with retries, concurrent  
actors, or persistence, an explicit runtime state machine may be clearer and  
more complete. Neither design validates facts that can change outside the  
program, such as a remote session's liveness.  

## Ownership in public signatures

An `impl Into<String>` parameter offers ownership conversion at the boundary,  
but passing `&str` still allocates when the result must be retained. Generic  
`AsRef` APIs can improve flexibility while multiplying monomorphized code.  
Choose the abstraction for the operation rather than applying it mechanically.  

Return borrowing relationships that are meaningful. A result derived from one  
input can borrow that input. A result assembled from independent inputs may  
need ownership or separate lifetimes rather than one artificially broad  
lifetime that forces all inputs to remain borrowed together.  

Annotate builders, guards, and returned futures with `must_use` when discarding  
them predictably violates their intended use. Do not make memory safety depend  
on a lint: callers can ignore warnings or intentionally forget a value.  

## Numeric and indexing contracts

Specify arithmetic semantics where overflow is a valid boundary case. Use  
checked operations for sizes, offsets, counters with finite limits, and protocol  
conversions. Use wrapping operations for defined modular arithmetic and  
saturating operations only when saturation is the actual domain rule.  

Integer overflow is not the C/C++ signed-overflow model. Runtime checks depend  
on compiler/profile settings. A debug-only panic does not establish a  
release-build precondition. Division and shifts have additional exceptional  
cases; check the operation's contract rather than treating overflow as the  
only numeric hazard.  

An `as` cast can truncate or change signedness. Use `TryFrom` for a header length  
that must fit the target's `usize` . Check multiplication and addition before  
allocation or slicing. A 64-bit-host test cannot establish 32-bit correctness.  

For floating-point input, define NaN, infinity, signed zero, and ordering rules.  
`total_cmp` provides a total order with specific bit-pattern semantics; it does  
not validate that a measurement is finite. A tolerance comparison must reject  
or explicitly handle NaN rather than accidentally treating it as success.  

Iterators eliminate many indexing mistakes but do not enforce equal lengths.  
`zip` silently stops at the shorter iterator. `chunks_exact` leaves a remainder;  
validate it when a partial record is forbidden. Range checks must include  
overflow in `start + count` , not only the final comparison with `len` .  

## Error and panic boundaries

Separate expected environmental failures from broken internal invariants.  
Expose structured errors where callers must decide whether to retry, reject,  
or reconcile. Attach context at the operation boundary without discarding the  
source error. Avoid classifying every error as retryable: malformed input,  
authentication failure, and resource saturation often need different policies.  

A panic across a public safe API is not automatically unsound, but hidden panic  
conditions can make the API unsuitable for a service or FFI boundary. Document  
intentional panic conditions and maintain memory safety on unwind. Callbacks,  
comparisons, hashing, formatting, cloning, and destructors are user code and can  
panic even when the container method looks simple.  

Do not use `unwrap` on untrusted input to claim a validated invariant. An  
internal `expect` can explain a fact established by an earlier check, but  
consider whether later refactoring can separate those locations. Prefer making  
the valid state explicit when it would otherwise require repeated assumptions.  

Mutex poisoning is an advisory signal, not a guarantee that all invariant  
violations are detected. Recover with `into_inner` only after choosing how to  
validate or repair protected state. Blindly ignoring poisoning can expose a  
partially committed application operation even when memory remains safe.  

## Trait and generic commitments

Public `Send` , `Sync` , lifetime, and returned-future bounds affect what callers  
can build. Adding a bound later can break implementations; omitting a needed  
future `Send` promise can prevent generic spawning. State the required contract  
at the boundary rather than hoping implementations happen to satisfy it.  

Use `?Sized` where borrowing unsized values is useful. Use associated types when  
one implementation chooses a related type; use generic parameters when multiple  
implementations for different inputs are intended. Higher-ranked bounds can  
express callbacks that work for any borrow lifetime without requiring `'static`  
data. Avoid extra abstractions that have no actual caller.  

Default method additions, blanket impls, auto-trait changes, and new enum variants  
can have compatibility effects beyond changed function signatures. Preserve  
exhaustiveness promises or use `non_exhaustive` deliberately at API design time.  
Trait-object suitability is a separate question from whether a trait compiles.  

Implement `Eq` , `Hash` , and `Ord` consistently. Mutating a map key's comparison  
or hash behavior while it is stored can violate the collection's documented  
logic requirements, even if interior mutability makes the mutation compile.  
Distinguish contained logic errors from undefined behavior obligations.  

## Serialization and macros

Define wire versions, unknown-field/tag policy, maximum sizes, duplicate-key  
policy, and semantic validation. Rust layout and serde derives alone do not  
define a durable protocol. Borrowed deserialization can reduce copies while  
tying output lifetime to the entire input; measure retention before using it  
for caches or long-lived tasks.  

For parser and serializer symmetry, test that accepted values round-trip under  
the chosen normalization rules, and that malformed lengths and tags fail before  
allocation. Do not promise byte-identical round trips when the format allows  
equivalent encodings and the implementation normalizes them.  

Rust macro hygiene does not prevent repeated evaluation of an expression  
metavariable. Bind an expression once when duplicating its expansion would  
duplicate side effects. Use `$crate` for paths into an exported macro's defining  
crate. Test macro use from an external crate, renamed dependencies, and relevant  
editions; an in-crate expansion can hide path and visibility defects.  

Prefer a function, generic, or `const fn` when syntax generation adds no useful  
capability. For a macro that generates unsafe code, the expansion's safety proof  
must hold for every accepted token pattern and user-supplied expression.  

## Sources

[Rust Reference](https://doc.rust-lang.org/stable/reference/index.html) ,  
[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html) ,  
[standard library](https://doc.rust-lang.org/stable/std/index.html) , and  
[Microsoft C/C++ training](https://microsoft.github.io/RustTraining/c-cpp-book/)  
support these boundary decisions. Exact conversion, collection, and trait  
contracts should be checked in their API documentation when they govern a change.
