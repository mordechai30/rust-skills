# Slices, strings, and parsing

## Contents

- [Views and ownership](#views-and-ownership)  
- [Bounds and disjoint mutation](#bounds-and-disjoint-mutation)  
- [Byte parsing without alignment assumptions](#byte-parsing-without-alignment-assumptions)  
- [UTF-8 and string boundaries](#utf-8-and-string-boundaries)  
- [Mutable and uninitialized buffers](#mutable-and-uninitialized-buffers)  
- [Raw slice obligations](#raw-slice-obligations)  

## Views and ownership

Rely on a slice's documented  
operations, not a manually transmuted two-word representation. Its lifetime  
keeps access within the owner's validity. Vector growth requires mutable access  
and is rejected while an overlapping borrowed view is still used, even if spare  
capacity makes a particular growth appear unlikely to reallocate.  

A borrowed parser can retain a large input allocation through a tiny returned  
view. Choose ownership at the consumer boundary using retention as well as  
copy cost; passing a view cheaply does not make keeping its owner alive cheap.  

## Bounds and disjoint mutation

Length arithmetic needs its own checks before a range is constructed. A safe  
`get(start..start + size)` cannot protect the addition from overflow. Use  
`checked_add` , validate the range, and preserve a useful parse error.  

`zip` stops at the shorter iterator. If a numerical kernel requires equal input  
lengths, check equality before zipping; silent truncation is a logic bug.  
`chunks_exact` exposes a remainder that must be handled or rejected. `windows`  
overlap; arbitrary simultaneous mutable overlapping windows would violate  
exclusive access, so ordinary `windows_mut` is not an available escape hatch.  

### Example: validate equal lengths before a zipped update

```rust
/// Adds elementwise using explicit wrapping arithmetic.
/// A mismatched length returns false without modifying the destination.
fn add_wrapping(dst: &mut [u32], src: &[u32]) -> bool {
    if dst.len() != src.len() {
        return false;
    }
    for (out, input) in dst.iter_mut().zip(src) {
        *out = out.wrapping_add(*input);
    }
    true
}
```

For two index-selected elements, prefer `get_disjoint_mut` when the MSRV supports  
it. It checks both bounds and overlap. For regions, split once and operate on  
the resulting sub-slices. These designs often let the optimizer remove repeated  
bounds checks without any unchecked access.  

```rust
/// Returns exclusive access to two checked, distinct elements.
/// Duplicate indices and out-of-range indices return None.
fn two_mut<T>(items: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    let [left, right] = items.get_disjoint_mut([a, b]).ok()?;
    Some((left, right))
}
```

An unchecked slice operation needs the same proof as its checked counterpart,  
including valid ranges and non-overlap. It merely removes enforcement. Use it  
only for a demonstrated bottleneck where the invariant remains clear after  
future maintenance.  

## Byte parsing without alignment assumptions

Network or file bytes do not inherit Rust struct alignment, native byte order,  
valid enum discriminants, or padding guarantees. Decode fields from checked  
byte ranges with `from_le_bytes` or `from_be_bytes` . A fixed-size array conversion  
checks the size; it does not allocate a heap buffer.  

### Example: read a little-endian length prefix

```rust
/// Borrows one length-prefixed payload and the remaining input.
/// Invalid or truncated prefixes return None; no heap allocation is needed.
fn frame(input: &[u8]) -> Option<(&[u8], &[u8])> {
    let (header, rest) = input.split_first_chunk::<4>()?;
    let length = usize::try_from(u32::from_le_bytes(*header)).ok()?;
    rest.split_at_checked(length)
}
```

A production parser should usually return errors that distinguish truncation,  
invalid length, unsupported format, and a resource-limit violation. This small  
example intentionally uses `Option` to show the borrowing relationship.  

Reject oversized frames before allocating or waiting to accumulate their whole  
contents. Define whether incomplete input means "need more bytes" or invalid EOF.  
Keep incremental parser state outside a cancelable future when it must survive  
an interrupted read. Document whether the returned remainder permits more frames  
or whether trailing bytes are an error.  

Casting a byte pointer to `*const Header` and reading it can fail alignment and  
validity requirements. `read_unaligned` addresses alignment only; it does not  
validate discriminants, pointers, endian order, lifetimes, or uninitialized  
padding. `repr(C)` controls layout, not a wire format.  

## UTF-8 and string boundaries

String length is a byte count. `chars` yields Unicode scalar values, not display  
characters or grapheme clusters. ASCII parsing can use bytes when the format is  
explicitly ASCII. User-visible cursor movement or truncation can require a  
grapheme-aware library instead of either bytes or `chars` .  

Use `str::get` for externally derived byte ranges: it returns `None` for invalid  
UTF-8 boundaries. Use `from_utf8` to validate a borrowed byte buffer. A lossy  
conversion deliberately replaces invalid data and should fit the product's  
semantics. Do not use unchecked UTF-8 simply because an upstream service  
"normally" produces text.  

### Example: borrow an ASCII tag from validated text

```rust
/// Returns the tag before the first colon without copying it.
/// A tag must be nonempty ASCII so callers do not confuse it with free text.
fn tag(input: &str) -> Option<&str> {
    let (head, _) = input.split_once(':')?;
    if head.is_empty() || !head.is_ascii() {
        return None;
    }
    Some(head)
}
```

String slicing borrows; `to_owned` , `to_string` , and many formatting operations  
allocate owned output. Prefer writing into a caller-owned reusable buffer when  
the surrounding pipeline already owns output storage. `String::from_utf8`  
can reuse a `Vec<u8>` allocation while checking validity.  

## Mutable and uninitialized buffers

`&mut [u8]` refers to initialized bytes. It is not an output view over arbitrary  
uninitialized capacity. For a `Vec` , `len` marks initialized elements; `capacity`  
marks available storage. Never extend `len` before an API has initialized the  
new elements.  

Use initialized buffers with safe I/O APIs unless measurement justifies a  
specialized uninitialized path. For manual initialization, use  
`spare_capacity_mut` to obtain `&mut [MaybeUninit<T>]` , write values, then commit  
exactly the initialized prefix. Partial writes, panics, and zero-sized types  
need explicit treatment.  

Borrowed slice APIs can be used for interior processing without returning the  
whole original buffer. Keeping a range or offset plus an owner can simplify a  
long-lived result, but the owner's mutation must still preserve the meaning of  
the range. An offset into a reused scratch buffer is not a durable result.  

## Raw slice obligations

For `from_raw_parts` , prove all of the following, not merely non-nullness:  

- The range is in one live allocation, valid for the required reads, and uses  
  initialized, valid `T` values.  
- The pointer is non-null and aligned even for an empty slice or zero-sized `T` .  
- The byte size is at most `isize::MAX` , and address arithmetic does not wrap.  
- The owner remains alive for the entire returned lifetime.  
- Mutation does not violate the shared reference's contract.  

For `from_raw_parts_mut` , additionally prove exclusive access for its returned  
lifetime. Neither other reads nor writes through independently derived pointers  
may violate that exclusivity. Passing a null pointer with zero length from C  
requires normalization to a valid empty Rust view rather than directly calling  
the raw constructor.  

Two allocations can happen to be adjacent. Address equality at their boundary  
does not permit constructing one slice over both. If ownership cannot be proved,  
return separate slices or copy into one owning allocation.  

Sources: [slice methods](https://doc.rust-lang.org/stable/std/primitive.slice.html) ,  
[str methods](https://doc.rust-lang.org/stable/std/primitive.str.html) ,  
[shared raw slice safety](https://doc.rust-lang.org/stable/std/slice/fn.from_raw_parts.html#safety) ,  
[mutable raw slice safety][link-1] ,  
[borrow splitting](https://doc.rust-lang.org/stable/nomicon/borrow-splitting.html) ,  
[Microsoft checked indexing][link-2] .  

[link-1]: https://doc.rust-lang.org/stable/std/slice/fn.from_raw_parts_mut.html#safety
[link-2]: https://microsoft.github.io/RustTraining/c-cpp-book/ch17-2-avoiding-unchecked-indexing.html
