# unsafe-minimize-scope

> Keep `unsafe` blocks as small as possible — mark only the operation that requires unsafety, not the surrounding safe code.

## Why It Matters

When an entire function is marked `unsafe fn`, every line inside appears equally suspect to an auditor. Shrinking unsafe blocks to the minimum isolates exactly which operation violates Rust's safety invariants, making reviews tractable and bugs easier to find. Rust 2024 warns by default through `unsafe_op_in_unsafe_fn` when an unsafe operation inside an `unsafe fn` lacks an explicit `unsafe {}` block.

## Bad

```rust
// Entire function body marked unsafe — safe arithmetic, bounds checks,
// and the single unsafe dereference are all equally "dangerous" to a reader.
unsafe fn sum_at(ptr: *const i32, len: usize, index: usize) -> i32 {
    let adjusted_len = len.saturating_sub(1); // safe — but looks unsafe
    assert!(index <= adjusted_len);           // safe — but looks unsafe
    let value = *ptr.add(index);              // the only actually unsafe op
    value + 1                                 // safe — but looks unsafe
}
```

```rust
// Huge unsafe block wrapping safe logic inside an unsafe fn (2024 edition
// now requires unsafe {} here anyway, but large blocks are still bad style).
pub unsafe fn process(ptr: *const u8, len: usize) -> Vec<u8> {
    unsafe {
        let mut result = Vec::with_capacity(len); // safe
        for i in 0..len {                         // safe
            result.push(*ptr.add(i));             // unsafe — buried in noise
        }
        result
    }
}
```

## Good

```rust
// A slice carries the validity and length contract for a safe caller.
fn sum_at(values: &[i32], index: usize) -> i32 {
    values[index] + 1
}
```

```rust
// In a genuinely unsafe fn, 2024 edition still requires unsafe {} per op.
/// # Safety
///
/// `ptr` must be valid for reads for `len` bytes and properly aligned.
pub unsafe fn process(ptr: *const u8, len: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(len); // safe — outside any unsafe block
    for i in 0..len {
        // SAFETY: caller guarantees ptr is valid for len bytes; i < len.
        let byte = unsafe { *ptr.add(i) };
        result.push(byte);
    }
    result
}
```

## Key Points

- **2024 edition `unsafe_op_in_unsafe_fn`**: even inside an `unsafe fn`, each unsafe operation now needs its own `unsafe {}`. This warns by default in Rust 2024; projects can deny the lint.
- Expose a safe wrapper only when its signature and checks uphold every unsafe precondition for all safe callers. Otherwise keep an `unsafe fn` contract.
- Each small unsafe block needs its own `// SAFETY:` comment (see `unsafe-safety-comment`).
- If multiple consecutive lines are all unsafe for the *same* invariant reason, a single block covering only those lines is acceptable.

## When a Larger Block Is Acceptable

If two unsafe operations share the *exact same precondition* and separating them would require re-stating the identical justification, a single block covering both is fine — but it should still be the minimum necessary scope.

## See Also

- [unsafe-safety-comment](unsafe-safety-comment.md) - write `// SAFETY:` above every unsafe block
- [unsafe-send-sync-manual](unsafe-send-sync-manual.md) - document invariants when manually implementing Send/Sync
