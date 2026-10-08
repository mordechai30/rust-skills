//! Complete contract-focused examples, reviewed on Rust 1.99.0, edition 2024.
//! Run this file with `rustc --edition=2024 --test examples-std.rs`, then the binary.
//! Tests exercise valid safe clients and rejected input shapes; they do not prove soundness.

#![deny(unsafe_op_in_unsafe_fn)]

use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Borrows unchanged text and owns transformed text.
/// Retaining the borrowed result also retains the lifetime of its input owner.
pub fn display_name(input: &str) -> Cow<'_, str> {
    if input.contains('_') {
        Cow::Owned(input.replace('_', " "))
    } else {
        Cow::Borrowed(input)
    }
}

/// Describes validation failures before a borrowed frame is returned.
/// Distinct cases let the caller separate incomplete input from forbidden size.
#[derive(Debug, PartialEq, Eq)]
pub enum FrameError {
    /// The four-byte length header is incomplete.
    Header,
    /// The declared length cannot fit the target address space.
    Length,
    /// The declared length exceeds the caller's protocol budget.
    Limit,
    /// The declared payload is not fully available.
    Payload,
}

/// Decodes one little-endian length-prefixed frame and returns borrowed views.
/// It validates size before slicing and leaves trailing data for the next frame.
pub fn frame(input: &[u8], maximum: usize) -> Result<(&[u8], &[u8]), FrameError> {
    let (header, tail) = input.split_first_chunk::<4>().ok_or(FrameError::Header)?;
    let len = usize::try_from(u32::from_le_bytes(*header)).map_err(|_| FrameError::Length)?;
    if len > maximum {
        return Err(FrameError::Limit);
    }
    tail.split_at_checked(len).ok_or(FrameError::Payload)
}

/// Returns distinct mutable elements only after bounds and overlap validation.
/// The safe slice API establishes disjointness, including for zero-sized types.
pub fn two_mut<T>(items: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    let [a, b] = items.get_disjoint_mut([a, b]).ok()?;
    Some((a, b))
}

/// Performs defined modular addition without truncating unequal inputs.
/// A length error leaves the destination unchanged.
pub fn add_wrapping(dst: &mut [u32], src: &[u32]) -> Result<(), &'static str> {
    if dst.len() != src.len() {
        return Err("length mismatch");
    }
    for (dst, src) in dst.iter_mut().zip(src) {
        *dst = dst.wrapping_add(*src);
    }
    Ok(())
}

/// Fills disjoint halves using scoped borrows instead of shared locking.
/// Scope completion joins both OS threads before their input borrow can end.
pub fn fill_halves(bytes: &mut [u8]) {
    let mid = bytes.len() / 2;
    let (left, right) = bytes.split_at_mut(mid);
    std::thread::scope(|scope| {
        scope.spawn(|| left.fill(1));
        scope.spawn(|| right.fill(2));
    });
}

/// Holds a one-shot publication protocol with an atomic payload.
/// The ready flag never resets; this is not a general multi-writer snapshot.
pub struct Published {
    /// Payload written before the release store and immutable afterward.
    payload: AtomicUsize,
    /// Release/acquire handoff indicating that payload publication is complete.
    ready: AtomicBool,
}

impl Published {
    /// Creates an unpublished state for a single producer.
    /// Publication is intentionally absent from the public API to avoid repeat writes.
    pub fn new() -> Self {
        Self {
            payload: AtomicUsize::new(0),
            ready: AtomicBool::new(false),
        }
    }

    /// Observes the payload only after acquiring its publication event.
    /// The payload load can be relaxed because it follows the acquired handoff.
    pub fn observe(&self) -> Option<usize> {
        self.ready
            .load(Ordering::Acquire)
            .then(|| self.payload.load(Ordering::Relaxed))
    }
}

impl Default for Published {
    fn default() -> Self {
        Self::new()
    }
}

/// Copies a foreign range into independently owned Rust storage.
/// It rejects invalid shape and policy limits without claiming to validate liveness.
///
/// # Safety
/// For nonzero len, ptr must point to len initialized readable bytes in one live
/// allocation, without address wrap or mutation during this call. The allocation
/// must remain valid until copying completes. Empty input may use null.
pub unsafe fn copy_foreign(
    ptr: *const u8,
    len: usize,
    maximum: usize,
) -> Result<Vec<u8>, &'static str> {
    validate_foreign_shape(ptr, len, maximum)?;
    if len == 0 {
        return Ok(Vec::new());
    }
    // SAFETY: The caller supplies allocation, initialization, and access guarantees.
    // u8 needs alignment 1; the checks supply non-nullness and representable extent.
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let mut owned = Vec::new();
    owned
        .try_reserve_exact(len)
        .map_err(|_| "allocation failed")?;
    owned.extend_from_slice(bytes);
    Ok(owned)
}

/// Checks publicly observable buffer shape without reading foreign memory.
/// Successful validation leaves allocation validity as a separate unsafe obligation.
pub fn validate_foreign_shape(
    ptr: *const u8,
    len: usize,
    maximum: usize,
) -> Result<(), &'static str> {
    if len > maximum || len > isize::MAX as usize {
        return Err("foreign range exceeds limit");
    }
    if len != 0 && ptr.is_null() {
        return Err("null nonempty range");
    }
    Ok(())
}

/// Initializes byte spare capacity before exposing it through vector length.
/// Safe extend_from_slice is the preferred production operation unless a need is measured.
pub fn append_bytes(out: &mut Vec<u8>, input: &[u8]) {
    let old_len = out.len();
    let new_len = old_len.checked_add(input.len()).expect("length overflow");
    out.reserve(input.len());
    for (slot, byte) in out.spare_capacity_mut().iter_mut().zip(input) {
        slot.write(*byte);
    }
    // SAFETY: reserve provides input.len() spare slots. Non-panicking byte writes
    // initialize exactly the added prefix, leaving all previous elements valid.
    unsafe { out.set_len(new_len) };
}

/// Transfers and immediately restores a box's unique allocation ownership.
/// It uses ownership APIs instead of creating a leaked reference for later reclamation.
pub fn box_round_trip(value: Box<u64>) -> Box<u64> {
    let pointer = Box::into_raw(value);
    // SAFETY: This pointer came from exactly this live Box and was not reclaimed,
    // mutated, or used to create another owner before this matching reconstruction.
    unsafe { Box::from_raw(pointer) }
}

/// Demonstrates Rust 1.99's NonNull vector ownership transfer.
/// Original type, capacity, initialized length, and allocator remain unchanged.
pub fn vec_round_trip(value: Vec<u32>) -> Vec<u32> {
    let (pointer, length, capacity) = value.into_parts();
    // SAFETY: These are the unmodified parts of the consumed vector. No other
    // owner or borrow exists, and its initialized prefix and layout are unchanged.
    unsafe { Vec::from_parts(pointer, length, capacity) }
}

/// Demonstrates a C variadic definition with an exact promoted-type contract.
/// Modular arithmetic avoids an incidental overflow panic at a non-unwind ABI boundary.
///
/// # Safety
/// The caller must supply at least two C-promoted i32 arguments in this ABI's list.
pub unsafe extern "C" fn variadic_pair(mut args: ...) -> i32 {
    // SAFETY: The caller guarantees that the next argument exists and is i32.
    let first = unsafe { args.next_arg::<i32>() };
    // SAFETY: The caller guarantees the same for the second argument.
    let second = unsafe { args.next_arg::<i32>() };
    first.wrapping_add(second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conditional_ownership() {
        assert!(matches!(display_name("plain"), Cow::Borrowed(_)));
        assert!(matches!(display_name("two_words"), Cow::Owned(_)));
        assert_eq!(display_name("two_words"), "two words");
    }

    #[test]
    fn checked_frame_boundaries() {
        assert_eq!(frame(&[0, 0, 0], 3), Err(FrameError::Header));
        assert_eq!(frame(&[4, 0, 0, 0], 3), Err(FrameError::Limit));
        assert_eq!(frame(&[3, 0, 0, 0, 1], 3), Err(FrameError::Payload));
        let packet = [3, 0, 0, 0, 7, 8, 9, 10];
        let (payload, tail) = frame(&packet, 3).unwrap();
        assert_eq!(payload, &[7, 8, 9]);
        assert_eq!(tail, &[10]);
        assert_eq!(payload.as_ptr(), packet[4..].as_ptr());
        assert_eq!(frame(&[0, 0, 0, 0], 0), Ok((&[][..], &[][..])));
        assert_eq!(frame(&[255, 255, 255, 255], 16), Err(FrameError::Limit));
    }

    #[test]
    fn utf8_ranges_are_byte_boundaries() {
        let text = "éx";
        assert_eq!(text.get(..1), None);
        assert_eq!(text.get(..2), Some("é"));
    }

    #[test]
    fn disjoint_indices_validate_before_mutation() {
        let mut values = [10, 20, 30];
        assert!(two_mut(&mut values, 1, 1).is_none());
        assert!(two_mut(&mut values, 0, 3).is_none());
        let (a, b) = two_mut(&mut values, 2, 0).unwrap();
        std::mem::swap(a, b);
        assert_eq!(values, [30, 20, 10]);
        assert!(two_mut(&mut [] as &mut [u8], 0, 1).is_none());
        assert!(two_mut(&mut [(), ()], 0, 1).is_some());
    }

    #[test]
    fn zipped_work_never_silently_truncates() {
        let mut values = [u32::MAX, 2];
        assert!(add_wrapping(&mut values, &[1]).is_err());
        assert_eq!(values, [u32::MAX, 2]);
        add_wrapping(&mut values, &[1, 3]).unwrap();
        assert_eq!(values, [0, 5]);
    }

    #[test]
    fn scoped_workers_cover_odd_and_empty_inputs() {
        let mut values = [0; 5];
        fill_halves(&mut values);
        assert_eq!(values, [1, 1, 2, 2, 2]);
        fill_halves(&mut []);
    }

    #[test]
    fn publication_is_observed_after_release() {
        let state = Published::new();
        assert_eq!(state.observe(), None);
        std::thread::scope(|scope| {
            scope.spawn(|| {
                state.payload.store(42, Ordering::Relaxed);
                state.ready.store(true, Ordering::Release);
            });
            // Scope completion joins the producer; this assertion is deterministic.
        });
        assert_eq!(state.observe(), Some(42));
    }

    #[test]
    fn foreign_shape_checks_do_not_dereference_invalid_input() {
        // SAFETY: Empty input has no allocation obligation under this function's contract.
        assert_eq!(
            unsafe { copy_foreign(std::ptr::null(), 0, 4) },
            Ok(Vec::new())
        );
        assert!(validate_foreign_shape(std::ptr::null(), 1, 4).is_err());
        assert!(validate_foreign_shape(std::ptr::null(), usize::MAX, usize::MAX).is_err());
        let bytes = [1, 2, 3];
        // SAFETY: bytes is live, initialized, immutable, and fully within the size limit.
        assert_eq!(
            unsafe { copy_foreign(bytes.as_ptr(), 3, 3) },
            Ok(bytes.to_vec())
        );
        // SAFETY: The supplied live range is valid; the policy rejects its size.
        assert!(unsafe { copy_foreign(bytes.as_ptr(), 3, 2) }.is_err());
    }

    #[test]
    fn initialized_prefix_survives_growth_and_empty_append() {
        let mut bytes = vec![1, 2];
        append_bytes(&mut bytes, &[]);
        append_bytes(&mut bytes, &[3; 4096]);
        assert_eq!(&bytes[..2], &[1, 2]);
        assert_eq!(bytes.len(), 4098);
        assert!(bytes[2..].iter().all(|byte| *byte == 3));
    }

    #[test]
    fn raw_ownership_is_reconstructed_once() {
        assert_eq!(*box_round_trip(Box::new(99)), 99);
        let mut vector = Vec::with_capacity(16);
        vector.extend([1, 2, 3]);
        let pointer = vector.as_ptr();
        let capacity = vector.capacity();
        let restored = vec_round_trip(vector);
        assert_eq!(restored, [1, 2, 3]);
        assert_eq!(restored.as_ptr(), pointer);
        assert_eq!(restored.capacity(), capacity);
        assert!(vec_round_trip(Vec::new()).is_empty());
    }

    #[test]
    fn variadic_promoted_arguments_match_contract() {
        // SAFETY: Both supplied arguments have exactly the documented promoted i32 type.
        assert_eq!(unsafe { variadic_pair(10i32, 20i32) }, 30);
    }
}
