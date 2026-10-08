# FFI and layout

## Contents

- [Design the boundary](#design-the-boundary)  
- [Buffers and ownership](#buffers-and-ownership)  
- [Layout is a contract](#layout-is-a-contract)  
- [Callbacks and threads](#callbacks-and-threads)  
- [Unwinding and errors](#unwinding-and-errors)  
- [Variadic functions](#variadic-functions)  
- [Sources](#sources)  

## Design the boundary

An FFI boundary joins two different type and lifetime systems. Use a narrow  
foreign ABI and convert its data into validated Rust types. A raw integer tag  
plus checked conversion is often safer than directly accepting a Rust enum.  
Keep Rust ownership types behind opaque handles unless the foreign interface  
explicitly defines a compatible representation and destruction protocol.  

Use the documented calling convention, target-dependent C types from  
`std::ffi` , and generated bindings when the foreign header is the authority.  
`extern "C"` describes an ABI, not the safety of the call. In edition 2024,  
external blocks are marked `unsafe` ; mark an individual foreign function safe  
only if its complete contract is enforceable for every safe call.  

Exported symbol attributes such as `no_mangle` have unsafe obligations,  
including avoiding symbol collisions. Use the edition-appropriate  
`#[unsafe(no_mangle)]` syntax when required. A stable Rust symbol name does not  
create a stable Rust ABI.  

Translate C/C++ concepts by contract, not syntax. A C++ span resembles a  
borrowed slice, but its lifetime is not checked by Rust when it crosses FFI.  
A foreign shared pointer has its own allocator and reference-count protocol;  
it is not an `Arc<T>` . C++ move-from rules are type-specific and differ from  
Rust's compile-time move tracking.  

## Buffers and ownership

Specify the buffer's element type, byte order, range, owner, validity duration,  
and concurrency rules. Decide explicitly whether null with length zero is  
allowed. A foreign ABI may allow it, but Rust slice constructors require a  
non-null aligned pointer even for empty slices. Return an empty Rust slice  
without constructing one from null.  

Lengths from foreign code or file headers need checked conversion to `usize`  
and checked multiplication by element size. Reject sizes above the protocol  
limit before allocation. A size check does not prove allocation validity.  

Prefer returning owned copies when a foreign borrow's lifetime cannot be  
enforced. For zero-copy access, a guard must keep the foreign allocation alive  
and prevent invalidating mutation for the full Rust borrow. Do not manufacture  
`'static` from a raw pointer because the C API provides no lifetime parameter.  

```rust
/// Represents the foreign ABI's read-only byte range.
/// The receiver must apply its unsafe allocation and access contract.
#[repr(C)]
struct ForeignBytes {
    /// Address supplied by the foreign owner; null is permitted only at len zero.
    data: *const u8,
    /// Number of initialized bytes available during the documented call scope.
    len: usize,
}
```

Do not release foreign allocations using Rust's global allocator unless that  
is the interface's documented allocator pairing. Export a matching destructor  
for Rust-owned opaque handles. Ensure the foreign caller invokes it exactly  
once, after all callbacks and borrows have ended. If arbitrary foreign callers  
can pass invented or repeated handles, pointer shape checks cannot make the  
operation safe; a registry with validated IDs is an alternative design.  

For C strings, distinguish a NUL-terminated borrowed pointer from an owned  
`CString` . The caller must guarantee a readable terminator within a live  
allocation. Do not scan arbitrary memory hoping to find one. Preserve the  
matching `into_raw` /`from_raw` contract and do not allow foreign code to change  
the effective length when the reclamation API forbids that.  

## Layout is a contract

Default Rust layout is not a C ABI or wire format. `#[repr(C)]` controls a  
type's specified C-style layout but does not recursively convert its fields to  
C-compatible layouts. Use it on all boundary components that require it.  
`#[repr(transparent)]` forwards the relevant layout/ABI of its eligible field;  
check its field restrictions and document the public representation promise.  

Layout equality alone does not imply function-call ABI compatibility, valid  
bit patterns, or legal ownership conversion. A `repr(C)` Rust enum still  
requires a valid discriminant. A C enum or integer bitmask may contain other  
values, so accept an integer and validate it before constructing the Rust enum.  

Padding is not a reliable serialized value. Reading a whole struct as bytes  
can expose uninitialized padding, platform byte order, and unstable details.  
Encode fields explicitly using a defined byte order; decode with checked  
byte operations such as `u32::from_le_bytes` rather than aligned casts.  

Packed fields can be misaligned. Even creating an intermediate `&field` can be  
invalid. Use a raw borrow followed by unaligned access for plain copied fields:  

```rust
/// Models a packed record to demonstrate unaligned field access.
/// It is not used as a portable wire-format decoder.
#[repr(C, packed)]
struct Packed {
    /// Small tag preceding the potentially misaligned value.
    tag: u8,
    /// Native-endian integer whose address need not have u32 alignment.
    value: u32,
}

/// Reads a packed field without creating a misaligned reference.
/// The field contains a valid u32, and read_unaligned copies it by value.
fn packed_value(record: &Packed) -> u32 {
    let pointer = &raw const record.value;
    // SAFETY: pointer addresses an initialized u32 in the live record.
    // read_unaligned does not require the field address to be u32-aligned.
    unsafe { pointer.read_unaligned() }
}
```

Unaligned reads of an owning non-`Copy` value need an ownership proof to avoid  
double drop. Packed storage is not a general remedy for cache or memory costs;  
it can force slower accesses and complicate borrowing.  

## Callbacks and threads

For callback registration, define who owns the context pointer, which threads  
can invoke it, whether callbacks are concurrent or reentrant, and how  
unregistration waits for in-flight callbacks. Freeing the context immediately  
after unregistering is unsafe if the foreign system can still call it.  

An `Arc` cloned into a callback context helps only if the registration and  
reclamation protocol keeps that clone alive. Passing `Arc::as_ptr` without  
retaining ownership does not transfer a strong reference.  

Do not hold a non-reentrant lock while invoking foreign code that can call back  
into the same state. Snapshot data before the callback or use a state machine  
that records and validates reentrancy. A thread-affine handle must preserve  
that affinity for construction, use, and destruction; a mutex alone cannot do  
so. Auto-trait markers should reflect the foreign contract.  

## Unwinding and errors

Specify the unwind ABI and failure policy. A Rust panic reaching a non-unwind  
ABI boundary causes termination under the documented rules; foreign unwinding  
into such a Rust boundary can be undefined behavior. Use `"C-unwind"` only  
when cross-language unwinding is intended and the participating stack and  
destructors satisfy its requirements.  

When the foreign interface requires error codes, contain Rust unwinding within  
Rust and translate failures. `catch_unwind` catches unwinding Rust panics; it  
does not catch aborts or make partially mutated state valid. Handling foreign  
exceptions with Rust panic-catching facilities has separate limitations.  
Keep the callback's invariant valid before exposing any failure result.  

Never return a borrowed pointer to an error message stored in a temporary  
`String` . Use caller-provided storage, a stable static message, or an owned  
error handle with a matching release operation. Define success/failure output  
initialization so callers do not read unset fields.  

## Variadic functions

Rust 1.99 stabilizes definitions of C-ABI variadic functions and the associated  
`VaList` argument access. This expands interoperation, not type checking of a  
foreign argument list. Restrict variadics to boundaries that require them; use  
typed slices or iterators for ordinary Rust APIs.  

The caller must provide the promised count and exact promoted C argument types.  
For example, small integer arguments follow C integer promotions and float  
arguments are passed as double. Retrieving a different type, reading beyond the  
list, or relying on an unsupported ABI violates the contract. `VaArgSafe`  
restricts supported retrieval types but cannot validate the actual argument  
list. Keep variadic manipulation within its permitted lifetime and copying  
rules. Test the supported target ABIs, not only the host compiler.  

## Sources

[Nomicon: FFI](https://doc.rust-lang.org/stable/nomicon/ffi.html) ,  
[Reference: external blocks](https://doc.rust-lang.org/stable/reference/items/external-blocks.html) ,  
[Reference: type layout](https://doc.rust-lang.org/stable/reference/type-layout.html) ,  
[Rust 1.99 release](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/) , and  
[Microsoft C/C++ training](https://microsoft.github.io/RustTraining/c-cpp-book/)  
provide the boundary and migration context.
