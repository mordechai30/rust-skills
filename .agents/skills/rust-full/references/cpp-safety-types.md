# Safety and data contracts: Rust versus C++

Examples use Rust 1.99, edition 2024, and C++20 unless marked otherwise. Do not execute intentionally  
invalid C++ accesses as normal tests.

## Unsafe is a proof boundary

**Use:** safe operations first. Add unsafe only for a required foreign/low-level contract or a measured  
need that safe code cannot meet. Document caller obligations in `# Safety` and established facts at each  
operation in `// SAFETY:`. Review every safe method that can change the invariant.

**Difference:** ordinary C++ operations can carry unchecked preconditions without a language-level  
`unsafe` marker. Rust requires an unsafe boundary for operations such as unchecked slice access and  
raw-pointer dereference. This does not disable Rust ownership/type checking. A safe Rust API must uphold  
its internal unsafe operations for every allowed safe caller; sound dependencies and FFI are part of that  
guarantee.

```cpp
#include <cassert>
#include <span>
#include <vector>
int first(std::span<const int> values) {
    return values[0]; // C++20: caller must ensure nonempty input.
}
int main() { std::vector<int> values{7}; assert(first(values) == 7); }
```

```rust
/// Returns the first element under an explicit nonempty-input contract.
/// # Safety
/// values must contain at least one element.
unsafe fn first_unchecked(values: &[i32]) -> i32 {
    // SAFETY: The caller guarantees index zero is in bounds.
    unsafe { *values.get_unchecked(0) }
}
fn main() {
    let values = [7];
    // SAFETY: values has one initialized element.
    assert_eq!(unsafe { first_unchecked(&values) }, 7);
    // Prefer values.first().copied() for a production optional result.
}
```

Do not form references to uninitialized or misaligned memory, invent `'static`, or reconstruct raw  
ownership twice. Null/size/alignment checks cannot prove foreign allocation liveness. Normalize an allowed  
null/zero-length foreign buffer before creating a Rust slice; raw slice constructors require non-null  
alignment even when empty. Prefer `Vec`, `Box`, and established wrappers over manual allocation. Rust  
1.99 raw-parts/layout APIs expand low-level capability without proving validity.

`MaybeUninit<T>` is storage, not a valid `T`. Publish vector length only after initializing the prefix.  
Maintain validity on panic and leaked guards. Tests and Miri find defects on covered paths; they do not  
prove the whole safe abstraction sound.

## Bounds, optional references, and nullability

**Use:** `Option<&T>` for optional borrowed identity, `Option<Box<T>>` for optional exclusive ownership,  
and `get` for externally derived indices. Do not convert a missing value into an unchecked access or an  
unexplained panic.

**Difference:** Rust safe indexing checks bounds and panics on failure; `get` returns `None`. C++20  
`span::operator[]` has an unchecked precondition; C++20 `vector::at` checks and throws. C++26 adds  
`span::at`; the C++20 fallback is explicit bounds validation. Rust references and `Box` cannot be null;  
`Option` makes absence explicit. Rust also permits raw null pointer values in safe code, but dereferencing  
them requires unsafe.

```cpp
#include <cassert>
#include <functional>
#include <optional>
#include <vector>
std::optional<std::reference_wrapper<const int>> find(const std::vector<int>& xs, unsigned i) {
    if (i >= xs.size()) return std::nullopt;
    return std::cref(xs[i]);
}
int main() { std::vector<int> xs{7}; assert(!find(xs, 1)); }
```

```rust
fn find(xs: &[i32], i: usize) -> Option<&i32> { xs.get(i) }
fn main() {
    assert_eq!(find(&[7], 1), None);
    assert_eq!(find(&[7], 0), Some(&7));
}
```

The lifetime of Rust's returned reference remains tied to `xs`; the C++ caller still owns that  
obligation. `Option<&T>` has documented niche guarantees; do not generalize them to arbitrary enums, DST  
handles, or foreign ABI layouts.

## UTF-8 text versus arbitrary bytes

**Use:** `String` /`&str` for validated UTF-8, `Vec<u8>` /`&[u8]` for arbitrary bytes, and `CString`  
/`CStr` at NUL-terminated C interfaces. Use `get` on external byte ranges and validate conversion with  
`from_utf8`. Choose lossy replacement only when the product permits it.

**Difference:** C++ `string` and `string_view` can hold arbitrary bytes and permit slicing inside a  
multibyte character. Rust `str` promises valid UTF-8; ranges must lie on character boundaries. Both  
lengths count bytes here. Rust `char` is a Unicode scalar value, not a C++ narrow byte `char`; neither  
scalar iteration nor byte iteration counts grapheme clusters.

```cpp
#include <cassert>
#include <string_view>
int main() {
    std::string_view text = "\xC3\xA9x";
    auto partial = text.substr(0, 1); // Valid byte view, incomplete UTF-8 character.
    assert(partial.size() == 1);
}
```

```rust
fn main() {
    let text = "éx";
    assert_eq!(text.len(), 3);
    assert_eq!(text.get(..1), None);
    assert_eq!(text.get(..2), Some("é"));
    assert!(std::str::from_utf8(&[0xff]).is_err());
}
```

Rust `String` /`str` have no NUL-termination promise. Do not pass their data as a C string without the  
proper adapter. `String::from_utf8_lossy_owned` is Rust 1.99; earlier supported Rust can use  
`String::from_utf8_lossy(&bytes).into_owned()` when replacement semantics fit.

## Arithmetic and initialization

**Use:** `TryFrom` for potentially narrowing conversions; checked arithmetic for sizes/offsets; explicit  
wrapping or saturating arithmetic only for those domain semantics. Initialize values before use. Do not  
rely on debug overflow checks or debug assertions as release preconditions.

**Difference:** C++ signed overflow is undefined behavior; Rust ordinary integer overflow is  
checked/panicking or wrapping according to settings, not signed-overflow UB. Explicit checked arithmetic  
avoids profile-dependent behavior. Rust has no implicit numeric widening conversion and rejects reads of  
possibly uninitialized locals. `as` still permits truncation, so explicit syntax alone does not validate a  
conversion.

```cpp
#include <cassert>
#include <cstdint>
#include <limits>
#include <optional>
std::optional<std::int32_t> add(std::int32_t a, std::int32_t b) {
    auto sum = std::int64_t{a} + b;
    if (sum < std::numeric_limits<std::int32_t>::min() ||
        sum > std::numeric_limits<std::int32_t>::max()) return std::nullopt;
    return static_cast<std::int32_t>(sum);
}
int main() { assert(!add(std::numeric_limits<std::int32_t>::max(), 1)); }
```

```rust
fn main() {
    assert_eq!(i32::MAX.checked_add(1), None);
    assert_eq!(i32::MAX.wrapping_add(1), i32::MIN);
    assert!(u8::try_from(256_u16).is_err());
}
```

```cpp
#include <cassert>
unsigned count(bool ready) {
    unsigned value;
    if (ready) value = 7;
    return value; // C++20 accepts this; false would read an uninitialized value.
}
int main() { assert(count(true) == 7); } // Never execute the invalid false path.
```

```rust,compile_fail
fn main() {
    let count: usize;
    println!("{count}"); // E0381: possibly uninitialized.
}
```

Fallible reservation handles failure of that reservation, not all future allocations. Default allocation  
failure can abort; it is not C++ `bad_alloc` recovery. Error and cleanup behavior must match the selected  
allocation and panic policy.

## Layout and foreign data

**Use:** default Rust layout for internal types; `repr(C)` for a defined C-compatible boundary; field-wise  
encoding for wire formats. Validate lengths, tags, and encoding before constructing Rust values. Do not  
transmute serialized bytes into structs or Rust ownership handles.

**Difference:** default Rust layout may reorder fields, whereas ordinary C++ standard-layout members have  
ordering rules. Rust `repr(C)` defines layout, not valid discriminants, ownership, endian order, or  
recursive compatibility of every field. Rust zero-sized marker values can occupy zero bytes; complete C++  
empty objects have nonzero size. `[[no_unique_address]]` can reduce a C++ member's storage but does not  
turn every empty complete object into a zero-sized type.

```cpp
#include <cassert>
struct Marker {};
struct Record { unsigned tag; unsigned count; };
int main() { static_assert(sizeof(Marker) >= 1); }
```

```rust
struct Marker;
#[repr(C)]
struct Record { tag: u32, count: u32 }
fn main() {
    assert_eq!(std::mem::size_of::<Marker>(), 0);
    assert_eq!(std::mem::size_of::<Record>(), 8);
}
```

Use opaque foreign handles with matching destruction; do not reinterpret a foreign `shared_ptr` as `Arc`.  
`extern "C"` supplies an ABI, not a safety proof. Rust 2024 requires unsafe external blocks and relevant  
unsafe attributes. Unwinding needs its documented boundary policy; catch-and-translate does not repair  
invalid state or catch aborts.

## Sources

- [Unsafe operations](https://doc.rust-lang.org/reference/unsafety.html)
- [Undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
- [Raw slice safety](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html#safety)
- [Option representation](https://doc.rust-lang.org/std/option/index.html#representation)
- [String](https://doc.rust-lang.org/std/string/struct.String.html)
- [str](https://doc.rust-lang.org/std/primitive.str.html)
- [Overflow and conversions](https://doc.rust-lang.org/reference/expressions/operator-expr.html)
- [Type layout](https://doc.rust-lang.org/reference/type-layout.html)
- [MaybeUninit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html)
