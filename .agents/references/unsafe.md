# Unsafe Rust

## Contents

- [The proof boundary](#the-proof-boundary)  
- [Pointer and reference validity](#pointer-and-reference-validity)  
- [Aliasing and interior mutability](#aliasing-and-interior-mutability)  
- [Initialization and panic safety](#initialization-and-panic-safety)  
- [Raw ownership and reclamation](#raw-ownership-and-reclamation)  
- [Variance, markers, and pinning](#variance-markers-and-pinning)  
- [Verification limits](#verification-limits)  
- [Sources](#sources)  

## The proof boundary

An unsafe function assigns documented  
obligations to its caller; its body still needs explicit proof at each unsafe  
operation. Use `unsafe_op_in_unsafe_fn` to keep those locations visible.  

Make the safe wrapper own or validate every condition it can. A check for null,  
length, and alignment cannot determine whether arbitrary foreign memory is  
allocated, initialized, or still live. Such promises require an unsafe caller  
contract or an owner whose construction already establishes them.  

For each abstraction, write the invariant before the code:  

- **Question:** What memory is valid?  
  **Required evidence:** Allocation origin, extent, alignment, initialization  

- **Question:** Who can access it?  
  **Required evidence:** Shared and exclusive access rules across the full lifetime  

- **Question:** What owns it?  
  **Required evidence:** Unique reclamation responsibility and matching allocator  

- **Question:** What can change?  
  **Required evidence:** Reallocation, mutation, callbacks, and concurrency  

- **Question:** What interrupts it?  
  **Required evidence:** Panic, early return, cancellation, and forgotten guards  

- **Question:** What is generic?  
  **Required evidence:** All allowed types, zero-sized types, destructors, and traits  


A `// SAFETY:` comment should connect the exact operation to those established  
facts. “The pointer is valid” merely repeats the claim. Explain which owner  
keeps it live, how the range was bounded, and why competing access is excluded.  

Audit safe methods that update pointer, length, capacity, or state flags. An  
incorrect safe setter can invalidate a later unsafe operation. Prefer private  
fields and small, invariant-preserving transitions. Unsafe code in a private  
module is still an obligation of every safe exported operation.  

## Pointer and reference validity

A raw pointer can exist without being dereferenceable. A reference generally  
cannot: forming `&T` or `&mut T` asserts alignment, non-nullness, valid referent  
and lifetime/access conditions at that moment. Do not construct a reference to  
uninitialized, freed, packed-misaligned, or foreign-invalid storage and promise  
to fix it later. Use raw borrow syntax when only a raw address is justified.  

A usable pointer has more than a numerical address. Provenance and allocation  
identity govern which memory it can access. Preserve pointers through pointer  
operations; use strict-provenance APIs when manipulating addresses is necessary.  
Do not implement an allocator or tagged pointer by assuming that converting an  
integer back into a pointer restores the required authority to access memory.  

`NonNull<T>` proves non-nullness, not alignment, initialization, lifetime,  
ownership, or exclusivity. `NonNull::dangling()` is useful as an aligned  
non-null sentinel when an API permits it, including empty/ZST buffers. It is  
not an allocation to dereference for a nonzero-size value.  

For `slice::from_raw_parts` , establish all API conditions: non-null aligned  
pointer even at length zero, one allocation covering the range, initialized  
valid elements, no disallowed mutation during the returned borrow, byte extent  
at most `isize::MAX` , and no address wrap. Mutable slices additionally require  
exclusive access for the entire borrow. Adjacent allocations cannot be merged  
into one slice just because their addresses happen to touch.  

Tie a returned borrow to a real owner, not an unconstrained generic lifetime.  
When the foreign owner cannot express that lifetime, copy into owned Rust memory  
under an explicit unsafe contract or expose a guard that keeps the owner alive.  

```rust
/// Copies a foreign byte range into Rust-owned memory.
/// Empty foreign buffers may use null; allocation validity cannot be checked here.
///
/// # Safety
/// For nonzero len, ptr must designate len initialized readable bytes within one
/// live allocation, with no mutation during this call and no address wrap.
/// The range must not exceed isize::MAX bytes.
unsafe fn copy_foreign(ptr: *const u8, len: usize) -> Result<Vec<u8>, &'static str> {
    if len == 0 {
        return Ok(Vec::new());
    }
    if ptr.is_null() || len > isize::MAX as usize {
        return Err("invalid foreign buffer shape");
    }
    // SAFETY: The caller guarantees allocation, initialization, and access.
    // u8 has alignment 1; checks above establish non-nullness and size bound.
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    Ok(bytes.to_vec())
}
```

Checks reject visibly invalid input but do not make this function safe to call  
with arbitrary addresses. Allocation can fail according to the allocation API's  
policy. Add a protocol size limit and fallible reservation when required.  

Use `ptr::add` /`offset` only with their allocation and overflow obligations met.  
Wrapping pointer arithmetic permits more intermediate addresses but does not  
make their dereference legal. Pointer differences likewise need an allocation  
relationship; do not subtract unrelated object pointers to determine lengths.  

## Aliasing and interior mutability

Shared references normally forbid mutation of their referent except through  
`UnsafeCell` regions. Mutable references represent exclusive access under Rust's  
rules; “there are no simultaneous CPU instructions” is insufficient to justify  
aliasing them. Borrow scopes, reborrowing, and accesses through derived pointers  
must remain consistent with the reference contract.  

`UnsafeCell<T>` permits interior mutation through shared access. It does not  
permit data races, multiple conflicting mutable references, or invalid `T`  
values. Its layout also does not automatically preserve niche optimizations in  
containing types. Obtain raw access through its documented interface and  
establish synchronization separately.  

Convert to references only for the range and time that are valid. Keep raw  
pointer operations raw where temporary references would claim too much. Prefer  
safe `split_at_mut` or `get_disjoint_mut` for disjoint elements; a pointer-based  
replacement must prove the same separation including empty ranges and ZSTs.  

Memory-mapped devices and foreign concurrent mutation need specialized access  
rules. Volatile reads/writes concern observable memory operations, not atomic  
synchronization. Atomic operations need suitable alignment, valid objects, and  
consistent access discipline; mixing ordinary and atomic racing accesses is  
not repaired by a fence.  

## Initialization and panic safety

Use `MaybeUninit<T>` for storage that does not yet contain a valid `T` . A zero  
bit pattern is not valid for every type: references, many enums, and nonzero  
integers are examples. `MaybeUninit::zeroed().assume_init()` needs a type-specific  
validity proof. Padding need not be initialized, so do not summarize all  
uninitialized bytes as forbidden in all contexts.  

`MaybeUninit` does not automatically drop initialized values. Track the  
initialized prefix of a partially built array and clean it up if construction  
can panic. `assume_init_ref` and `assume_init_mut` require initialization already  
to be complete; they are not interfaces for initializing an arbitrary `T` .  

For a vector, `len` means the prefix contains valid elements that may be read  
and dropped. Never set `len` before completing initialization. Reserve before  
taking raw pointers because growth can invalidate them. Prefer safe extension  
unless measurements or an external filling API justify spare-capacity access.  

```rust
/// Appends a byte slice through uninitialized spare capacity.
/// This illustrates the initialization proof; extend_from_slice is usually simpler.
fn append_bytes(out: &mut Vec<u8>, input: &[u8]) {
    let old_len = out.len();
    let new_len = old_len.checked_add(input.len()).expect("length overflow");
    out.reserve(input.len());
    for (slot, byte) in out.spare_capacity_mut().iter_mut().zip(input) {
        slot.write(*byte);
    }
    // SAFETY: reserve made at least input.len() spare slots available.
    // The non-panicking u8 writes initialized exactly old_len..new_len.
    unsafe { out.set_len(new_len) };
}
```

This narrow byte example avoids panicking user code. A generic constructor with  
callbacks or destructors needs a cleanup guard or incremental length tracking.  
No safety-critical invariant may depend on a user's destructor definitely  
running: safe code can use `mem::forget` , create cycles, or exit the process.  
Leaking resources may be acceptable for memory safety; exposing dangling  
references after a forgotten guard is not.  

Separate basic safety from strong transactional behavior. On panic, a container  
must remain valid and avoid double drop; it need not always restore the previous  
application state. Never use `catch_unwind` as a substitute for maintaining  
validity during unwinding, or assume every panic strategy unwinds.  

## Raw ownership and reclamation

`ptr::read` copies bits while producing an owned value. For non-`Copy` types,  
arrange that the source is no longer read or dropped as the same owner.  
`ptr::write` does not drop a previous initialized value; replacing one without  
handling it can leak resources. `copy_nonoverlapping` requires disjoint byte  
ranges; `copy` allows overlap but still needs valid source/destination ranges  
and a correct ownership state after copying.  

Use paired ownership APIs such as `Box::into_raw` and `Box::from_raw` rather  
than leaking a box and later inventing reclamation. Reconstruct once, with the  
correct pointer, type, allocator, layout, and initialized value. A pointer into  
the middle of a box is not a pointer to that box's allocation.  

```rust
/// Temporarily exposes a box pointer and restores its unique ownership.
/// No foreign code is involved and no other owner is constructed.
fn box_round_trip(value: Box<u64>) -> Box<u64> {
    let raw = Box::into_raw(value);
    // SAFETY: raw came from this Box, still points to its live initialized
    // allocation, and has not been reclaimed or used to create another owner.
    unsafe { Box::from_raw(raw) }
}
```

For vectors, retain the original allocator, element type, capacity, and length.  
Do not reconstruct `Vec<u8>` from a `Vec<u32>` allocation solely because byte  
counts match: deallocation alignment and layout still matter. Raw parts are an  
ownership transfer, not a serializable buffer description.  

`ManuallyDrop<T>` suppresses automatic destruction but retains `T` 's validity  
requirements. It is not interchangeable with `MaybeUninit<T>` . Unsafe manual  
drop creates extra state obligations; derived trait methods or public fields  
can expose a value that has already been dropped. Prefer ordinary owners where  
they can express the destruction sequence.  

Lock-free reclamation needs proof beyond atomic pointer publication. An acquire  
load can observe a pointer whose allocation another thread later frees. Use a  
proven reclamation mechanism, such as a suitable ownership protocol or established  
epoch/hazard-pointer library, and verify its reader lifetime rules.  

## Variance, markers, and pinning

Raw pointer wrappers need intentional lifetime, variance, ownership, and auto  
trait behavior. `NonNull<T>` is covariant; a wrapper that permits writing `T`  
through shared access may need invariance. `PhantomData` affects variance,  
drop checking, and auto traits. Select its type for the actual contract rather  
than copying a marker from a superficially similar container.  

Manual `Send` and `Sync` impls must account for all operations, destructors,  
allocator behavior, callbacks, and thread-affine foreign resources. A mutex  
around a handle does not make moving or destroying that handle on another  
thread legal. Stable marker fields can opt out of auto traits; unstable negative  
impl syntax in teaching material is not a stable implementation recipe.  

Pinning guarantees depend on the pointee and projection contract, not on the  
address of the `Pin` wrapper. Do not move a structurally pinned field through  
`get_unchecked_mut` , `mem::replace` , or a destructor. An unsafe projection must  
preserve pinning through drop and every safe accessor. Use established projection  
tools or avoid self-referential storage when an index or owned external buffer  
can express the relationship.  

## Verification limits

The Rust Reference does not claim a complete finalized memory model. Use the  
documented API contracts and conservative established rules. Treat experimental  
aliasing models as tools for finding defects, not permission to ignore a  
documented obligation.  

Run Miri on focused valid-client tests where supported. Exercise empty and ZST  
buffers, boundary lengths, panicking constructors, repeated ownership transfer,  
and safe API misuse attempts. Sanitizers can find target-specific memory defects  
in native executions; concurrency model checking can explore bounded schedules.  
These tools detect classes of bugs, not all bugs or all foreign behavior.  

Do not deliberately execute UB as a normal regression test. Use compile-fail  
tests for rejected borrow patterns, safe negative-input tests for validation,  
and separate diagnostic experiments only when a suitable tool can identify the  
invalid operation. Review the invariant and safe surface even when every test  
passes.  

## Sources

[Reference: undefined behavior][link-1] ,  
[Reference: memory model](https://doc.rust-lang.org/stable/reference/memory-model.html) ,  
[Nomicon](https://doc.rust-lang.org/stable/nomicon/index.html) ,  
[MaybeUninit](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html) , and  
[raw slice construction](https://doc.rust-lang.org/stable/std/slice/fn.from_raw_parts.html)  
state the core obligations. The  
[Nomicon issue tracker](https://github.com/rust-lang/nomicon/issues) helps identify  
examples and explanations that need correction or a stability check.  

[link-1]: https://doc.rust-lang.org/stable/reference/behavior-considered-undefined.html
