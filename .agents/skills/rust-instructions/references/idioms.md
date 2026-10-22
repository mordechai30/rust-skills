# Idiom choices

## Choose by clarity

- Prefer a familiar idiom when it makes ownership, data flow, and failure behavior easier to understand.
- Keep explicit statements, names, matches, and loops when they are clearer without a material performance
  cost; do not shorten code solely to look idiomatic.
- Preserve Style Guide formatting; distinguish an expression choice from formatting rules.
- Inspect Clippy suggestions; use a narrow explained allowance when a style rewrite reduces clarity.
- Measure a disputed hot path before claiming that expressions, combinators, or iterators are faster.

## Ownership and APIs

- Accept borrowed views for immediate inspection; require an owned container only for actual container
  operations or ownership transfer.
- Use `new` for a clear constructor; implement `Default` only when its value meets the domain contract.
- Keep fields private when construction must enforce invariants; expose validated fallible constructors.
- Use `take` or `replace` only when the temporary replacement preserves validity on every exit.
- Return a consumed input in the error when recovery needs it and the error contract permits that return.
- Keep closure capture preparation close to the closure; use distinct names instead of shadowing when they
  clarify cloning, borrowing, or ownership transfer.
- Use temporary mutability when it makes the transition to read-only use clear.
- Use `Deref` for pointer-like semantics, not to imitate inheritance or hide unrelated conversions.
- Use `non_exhaustive` only for a required evolution contract; define useful unknown-variant handling.

## Expressions, text, and dispatch

- Use `?`, `match`, or combinators according to the clearest required error and absence behavior.
- Preserve fallback evaluation and side effects; use a lazy fallback when eager construction is material.
- Use `if let` for a clear optional branch; use option iteration when it fits an existing iterator pipeline.
- Use `format!` for clear owned text construction; reuse a buffer when measurements justify it.
- Prefer generics when concrete types satisfy the requirement clearly; use `dyn Trait` for required runtime
  heterogeneity or measured build-size benefits.
- Check MSRV and temporary lifetimes before adopting version-specific borrowed dispatch examples.

## Documentation and exceptional boundaries

- Run documentation assertions that claim behavior; label compile-only examples explicitly.
- Do not report an uncalled helper's assertions as executed tests.
- Apply the existing safety policy to FFI examples; require exact foreign ownership and lifetime contracts.
- Retain sound necessary FFI; correct source defects without treating every unsafe example as a conflict.
- Show source conflicts to the user and obtain resolution before adopting disputed examples.

## Sources

- Read [Rust idioms](https://rust-unofficial.github.io/patterns/idioms/) as conditional design guidance.
- Read [Ian Bull's article](https://ianbull.com/posts/idiomatic-rust/) without treating brevity as a goal.
- Check [Option fallback behavior](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)
  before replacing an explicit branch with a combinator.
