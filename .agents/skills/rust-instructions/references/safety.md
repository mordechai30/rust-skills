# Safety boundaries

## Safe code first

- Try to avoid unsafe Rust as much as possible unless it materially complicates code or dependencies.
- Use safe checked slices, initialized buffers, byte decoding, and established abstractions first.
- Document why a safe alternative adds material complexity before introducing unsafe.
- Do not introduce unsafe to bypass the borrow checker or based on an unmeasured performance assertion.
- Evaluate a dependency's safe interface, maintenance, advisories, and compatibility rather than banning its
  internals.

## Exceptional unsafe

- Write the invariant before implementation; keep unsafe operations in the smallest useful abstraction.
- Document caller obligations under `# Safety`; explain established facts beside each operation in
  `// SAFETY:`.
- Enable `unsafe_op_in_unsafe_fn`; use explicit unsafe blocks inside unsafe functions.
- Review safe constructors, setters, callbacks, panic paths, and destructors that can change those invariants.
- Prove alignment, allocation extent, initialization, validity, lifetime, and access before forming
  references.
- Preserve unique ownership and matching allocator/layout when transferring and reclaiming raw storage.
- Keep initialized vector length valid at every exit; do not publish capacity as initialized contents.
- Preserve validity during panic, cancellation, early return, and forgotten guards.
- Enforce safety preconditions in release builds; do not rely on debug assertions.

## FFI, layout, and pinning

- Use necessary, sound unsafe for FFI under the existing exception; do not classify FFI itself as a conflict.
- Prefer a maintained safe wrapper when it fits; do not reject it because its internals use unsafe.
- Use narrow ABI boundaries, documented calling conventions, and validated Rust values.
- Validate tags, encoding, arithmetic, and size limits before allocation or reference construction.
- Handle permitted null/empty foreign buffers without forming a null Rust slice.
- Keep foreign allocation liveness and aliasing in a documented owner or unsafe caller contract.
- Do not claim null or bounds checks prove that an arbitrary foreign pointer refers to live valid memory.
- Mark exported functions unsafe when they require caller guarantees that a safe API cannot establish.
- Check foreign error codes and signed lengths before converting them; bound lengths before decoding.
- Use NUL-aware decoding for terminated output; do not pass a terminator to `CString::new` and unwrap.
- Keep string owners alive through foreign use; use an explicit owner if the foreign API retains a pointer.
- Use matching foreign destructors; wait for in-flight callbacks before releasing their context.
- Use `repr(C)` only for a required layout contract; do not treat padding or native layout as a wire format.
- Define panic/unwind handling at the ABI boundary; do not claim panic catching repairs invalid state.
- Use safe pinning and maintained projection helpers; do not move structurally pinned fields.

## Concurrency and verification

- Let `Send` and `Sync` follow sound fields; prove foreign thread-affinity and destruction obligations for
  exceptions.
- Prefer locks or established reclamation over a custom lock-free pointer protocol.
- Prove publication and reclamation separately; do not assume an acquired pointer stays allocated.
- Keep guards short, establish lock order, and test shutdown, cancellation, saturation, and failure paths.
- Use focused Miri or sanitizers for supported memory paths and model checking for custom synchronization.
- Exercise valid safe-client boundary cases; do not deliberately execute undefined behavior in ordinary tests.
- Report unrun or unsupported checks; do not claim any passing tool proves complete soundness.

## Sources

- Check the [Rust Reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html),
  [raw-slice contract](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html), and
  [Nomicon](https://doc.rust-lang.org/nomicon/) for exceptional unsafe decisions.
- Check [Tokio cancellation](https://docs.rs/tokio/latest/tokio/macro.select.html) and
  [Arc contracts](https://doc.rust-lang.org/std/sync/struct.Arc.html) against the selected version.
