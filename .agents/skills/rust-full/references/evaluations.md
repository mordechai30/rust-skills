# Behavioral evaluation cases

## Contents

- [Borrowed packet with bounded retention](#case-1-borrowed-packet-with-bounded-retention)  
- [Bounded async workers with real termination](#case-2-bounded-async-workers-with-real-termination)  
- [Cancellation during partial input](#case-3-cancellation-during-partial-input)  
- [Foreign bytes and ownership reclamation](#case-4-foreign-bytes-and-ownership-reclamation)  
- [Atomic publication and lifetime](#case-5-atomic-publication-and-lifetime)  
- [Optimization under an older MSRV](#case-6-optimization-under-an-older-msrv)  
- [Evaluation limits](#evaluation-limits)  

These cases test whether an agent applies contracts rather than repeats Rust  
advice. Run each case in a fresh context with this skill available. Compare  
observable behavior against the acceptance criteria; do not grade by matching  
phrases. Code-example tests validate selected mechanisms, not agent behavior.  

## Case 1: Borrowed packet with bounded retention

**Prompt:** Implement a parser for a four-byte little-endian payload length  
followed by that many bytes. Return a borrowed payload and the unused suffix.  
Reject incomplete input and lengths above a supplied limit. Later, put selected  
payloads in a cache that can outlive the input buffer; avoid retaining a large  
input allocation solely for a small cached payload.  

**Expected behavior:** The parser checks header availability, converts the  
length for the target, enforces the limit, and checks range extent. It uses safe  
byte decoding rather than casting an unaligned header to `u32` . The cache  
boundary deliberately creates an owned payload or a suitable compact owner.  
The agent explains the copy-versus-retention decision without forcing every  
parser output to allocate.  

**Acceptance:** Truncated header/body, zero length, exact limit, over-limit,  
and trailing-data tests pass. A lifetime rejection test confirms that the  
borrowed view cannot survive destruction of its input. Cache ownership is  
independent of that input. No unchecked slicing or invented `'static` lifetime.  

## Case 2: Bounded async workers with real termination

**Prompt:** Process a streaming work source with at most N admitted async tasks.  
Each item has a maximum allowed size. Fail the group on the first worker error,  
cancel outstanding workers, and return only after their async tasks have ended.  
Shutdown must stop admission. Do not keep all inputs or outputs in memory.  

**Expected behavior:** The agent limits admission before spawning, enforces  
message-size/byte budgets, drains completions, distinguishes worker errors from  
join failures, and owns completion through shutdown. It avoids collecting the  
whole source or placing a semaphore inside unlimited spawned tasks. If cleanup  
needs async I/O, it defines an explicit cleanup phase instead of claiming Drop  
can await it.  

**Acceptance:** Observed admitted/active counts stay within N; no work is admitted  
after the supervisor's shutdown transition. Error return observes destruction  
of all ordinary async siblings. A blocked producer cannot bypass the budget.  
Started blocking closures are handled under an explicit different policy.  

## Case 3: Cancellation during partial input

**Prompt:** Review a framed reader that selects between shutdown and  
`read_exact` , stores its offset in a temporary future, and retries the reader  
after shutdown is withdrawn. Make partial input resumable without losing bytes.  

**Expected behavior:** The agent identifies cancellation progress loss and  
keeps buffer/offset in the reader's owner. It uses an operation with the needed  
cancellation contract, updates progress synchronously after a completed read,  
and distinguishes EOF from incomplete input. It preserves bounded allocation.  

**Acceptance:** Force cancellation after one fragment and before the remainder;  
resuming produces exactly the original payload. Test EOF after partial progress  
and cancellation before any bytes. No unexplained retry of a non-idempotent write.  

## Case 4: Foreign bytes and ownership reclamation

**Prompt:** A C API supplies `(pointer, length)` , permits null at length zero,  
and promises initialized immutable bytes valid during the call. Expose an owned  
Rust result. Review a separate API that obtains a leaked Box reference and  
later reclaims it with `Box::from_raw` .  

**Expected behavior:** The wrapper handles zero length without constructing a  
null slice, checks shape/limit conditions, and leaves allocation validity in an  
explicit unsafe caller contract. It does not pretend null/alignment checks  
prove liveness. The Box round trip uses an ownership-transfer pair and proves  
one matching reclamation, with no outstanding invalidated references.  

**Acceptance:** Valid nonempty and empty buffers pass; null/nonzero and oversized  
shapes reject before dereference. No invalid pointer is dereferenced to test  
rejection. The raw owner is reconstructed once with the original layout and  
allocator. Safety comments explain the source of the promises.  

## Case 5: Atomic publication and lifetime

**Prompt:** Review a shared pointer published with a relaxed store and a SeqCst  
flag. The producer can replace and free the pointed-to object while readers  
use it. Make the design sound and explain the relevant memory ordering.  

**Expected behavior:** The agent separates publication from reclamation. It  
identifies the exact release/acquire observation needed for non-atomic payload  
visibility, and uses a lifetime/reclamation mechanism or simpler locked ownership.  
It does not claim SeqCst alone keeps an allocation alive or makes all accesses  
safe. It considers ABA and replacement semantics if retaining a lock-free design.  

**Acceptance:** The owner outlives every reader. Ordering arguments name the  
observed store or release sequence. Bounded scheduling tests exercise replace,  
read, and retire interleavings without executing deliberate UB in ordinary tests.  

## Case 6: Optimization under an older MSRV

**Prompt:** Reduce allocations in a Rust library with an MSRV older than 1.99.  
An example skill recommends new Vec raw-parts APIs, `target-cpu=native` , and  
unchecked indexing with debug assertions.  

**Expected behavior:** The agent preserves MSRV and deployment portability,  
measures the suspected allocation cost, and first tries a safe representation  
or buffer-reuse change. It checks retained capacity after large inputs. If unsafe  
remains justified, release builds enforce the complete invariant and the proof  
does not depend on a debug assertion.  

**Acceptance:** The supported MSRV and feature matrix compile. Equivalent outputs  
and malformed-input behavior remain. A benchmark report states workload and  
limits rather than promising a fixed percentage gain. Advisory skills do not  
override the product's constraints.  

## Evaluation limits

The bundled Rust tests cover parsing, disjoint access, initialized capacity,  
publication, task admission, task cleanup, and cancellation progress. Structural  
validation checks metadata and reference navigation. These are necessary evidence  
for the examples and package; they do not establish that every agent/model will  
use the skill correctly. Record actual fresh-agent runs and model settings when  
those evaluations are performed. Do not label unrun cases as passed.
