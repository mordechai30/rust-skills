# Ownership, access, and lifetimes: Rust versus C++

Examples use stable Rust 1.99, edition 2024, and C++20 unless marked otherwise. Rust `compile_fail` blocks  
are deliberately rejected. C++ examples contain no executed undefined behavior.

## Moves and copies

**Use:** borrow for inspection, move for ownership transfer, clone for deliberate independent ownership.  
Pass cheap `Copy` values by value. Do not clone merely to silence a borrow error.

**Difference:** C++ string initialization from an lvalue copies; `std::move` selects a move operation and  
leaves an existing source object with its type's moved-from contract. Rust assignment moves a non-`Copy`  
value and rejects subsequent use of the old binding. Rust moves invoke no custom move constructor. `Clone`  
may copy, allocate, or share; `Copy` can include large inline arrays.

```cpp
#include <cassert>
#include <string>
#include <utility>
int main() {
    std::string source = "job";
    auto copy = source;
    auto moved = std::move(source);
    source.clear(); // Valid operation on this moved-from std::string.
    assert(copy == moved);
}
```

```rust
fn main() {
    let source = String::from("job");
    let copy = source.clone();
    let moved = source;
    assert_eq!(copy, moved);
}
```

```rust,compile_fail
fn main() {
    let source = String::from("job");
    let moved = source;
    println!("{source} {moved}"); // E0382: source was moved.
}
```

## Exclusive access

**Use:** `&T` for shared access and `&mut T` for exclusive access. Keep borrows narrow. Split independent  
fields or slices before considering interior mutability.

**Difference:** C++ `T&` permits multiple writable aliases. Rust prevents conflicting overlapping borrows  
in safe code; `&mut T` is not simply C++ `T&` with a different spelling. Temporary reborrowing is allowed;  
the original exclusive borrow is unavailable during the reborrow. Shared references can expose interior  
mutability only through an appropriate abstraction.

```cpp
#include <cassert>
void add(int& a, int& b) { a += 1; b += 1; }
int main() {
    int value = 0;
    add(value, value); // Legal aliasing; value becomes 2.
    assert(value == 2);
}
```

```rust,compile_fail
fn add(a: &mut i32, b: &mut i32) { *a += 1; *b += 1; }
fn main() {
    let mut value = 0;
    add(&mut value, &mut value); // E0499: overlapping exclusive borrows.
}
```

```rust
fn main() {
    let mut values = [0, 0];
    let (left, right) = values.split_at_mut(1);
    left[0] += 1;
    right[0] += 1;
    assert_eq!(values, [1, 1]);
}
```

For two externally chosen indices, `get_disjoint_mut` (Rust 1.86) checks bounds and overlap. For  
contiguous regions, `split_at_mut_checked` (Rust 1.80) rejects invalid splits. Neither requires authored  
unsafe code.

## Reference lifetime enforcement

**Use:** `&[T]`, `&mut [T]`, and `&str` for immediate borrowed inputs. Use owned values for retained  
results when borrowing would bind the consumer to an unsuitable owner. Add lifetime parameters to express  
real input/output relationships; annotations do not extend storage lifetime.

**Difference:** C++ references, `span`, and `string_view` need a caller-maintained lifetime/invalidation  
contract. Rust safe references encode and enforce that relationship. A borrow can end after its last use;  
a stored reference or destructor can keep it relevant longer. Mutation requiring exclusive access is  
rejected while an overlapping borrow remains in use, even if reserved capacity would avoid reallocation in  
that particular run.

```cpp
#include <cassert>
#include <string>
#include <string_view>
std::string_view head(std::string_view text) { return text.substr(0, 1); }
int main() {
    std::string owner = "ab";
    auto view = head(owner);
    assert(view == "a"); // Caller keeps owner alive and avoids invalidation.
}
```

```rust
fn head(text: &str) -> Option<&str> { text.get(..1) }
fn main() {
    let owner = String::from("ab");
    let view = head(&owner).unwrap();
    assert_eq!(view, "a");
}
```

```rust,compile_fail
fn head(text: &str) -> Option<&str> { text.get(..1) }
fn main() {
    let view;
    {
        let owner = String::from("ab");
        view = head(&owner);
    }
    println!("{view:?}"); // E0597: owner cannot expire before this borrow use.
}
```

```rust,compile_fail
fn main() {
    let mut values = vec![1, 2];
    let first = &values[0];
    values.push(3);
    println!("{first}"); // E0502: growth conflicts with this outstanding borrow.
}
```

For multiple inputs, tie the output only to the input it actually borrows. `T: 'static` means no expiring  
borrowed data is contained in `T`; it does not require an owned value to live forever. Prefer IDs/ranges  
plus owned storage over self-referential growable containers; validate identity and UTF-8 boundaries when  
resolving them.

## Shared ownership and interior mutability

**Use:** `Box<T>` for exclusive heap ownership, `Rc<T>` for single-thread shared ownership, `Arc<T>` for  
ownership shared across threads. Use `Weak` for back-references. Do not add reference counting when an  
ordinary borrow suffices.

**Difference:** C++ `shared_ptr<T>` normally allows mutation through each handle; sharing its control  
block does not synchronize payload access. Rust `Arc<T>` normally supplies shared access only, and its  
cross-thread use requires the payload's `Send` /`Sync` contracts. `Arc::clone` shares one allocation, not  
a snapshot. `Arc::get_mut` can obtain exclusive access only when its documented uniqueness conditions  
hold.

```cpp
#include <cassert>
#include <memory>
int main() {
    auto owner = std::make_shared<int>(1);
    auto other = owner;
    *other += 1; // Single-thread use; concurrent writes would need synchronization.
    assert(*owner == 2);
}
```

```rust,compile_fail
use std::sync::Arc;
fn main() {
    let owner = Arc::new(1);
    let other = Arc::clone(&owner);
    *other += 1; // Arc does not supply DerefMut to its shared payload.
}
```

**Choose mutation by contract:** `Cell<T>` for replacement through shared access; `RefCell<T>` for  
single-thread runtime borrow checks; `Mutex<T>` for synchronized exclusive access; `RwLock<T>` only when  
measured access patterns justify reader bookkeeping. `Cell::get` requires `Copy`, but `Cell::replace` can  
work with a `String`. `Arc<RefCell<T>>` is not a cross-thread substitute for a mutex.

C++ `mutable` permits modification through a const access path without checking outstanding aliases. Rust  
`RefCell` checks live guards; recoverable conflicts need `try_borrow_mut`, not a panic-based protocol.

```cpp
#include <cassert>
struct Cache {
    mutable int hits = 0;
    void record() const { ++hits; }
};
int main() { const Cache cache; cache.record(); assert(cache.hits == 1); }
```

```rust
use std::cell::RefCell;
fn main() {
    let cache = RefCell::new(0);
    let reader = cache.borrow();
    assert!(cache.try_borrow_mut().is_err());
    drop(reader);
    *cache.borrow_mut() += 1;
    assert_eq!(*cache.borrow(), 1);
}
```

## Taking a field and enforcing RAII

**Use:** owning fields and `Drop` for synchronous cleanup. Use `Option::take`, `mem::take`, or  
`mem::replace` to move from borrowed storage while leaving a domain-valid replacement. Expose fallible  
close/commit explicitly. Do not depend on a destructor for memory-safety-critical repair: safe Rust can  
forget a guard, leak, or abort.

**Difference:** C++ moves can leave a moved-from member and can implement custom repair. Rust cannot leave  
an uninitialized field behind an ordinary `&mut` reference. Rust struct fields drop in declaration order;  
C++ members are destroyed in reverse declaration order. Both languages normally destroy locals in reverse  
order. Rust `Copy` and `Drop` cannot be implemented on the same type; owned members still drop when no  
custom `Drop` exists.

```cpp
#include <cassert>
#include <utility>
#include <vector>
struct Queue {
    std::vector<int> jobs;
    std::vector<int> drain() { return std::exchange(jobs, std::vector<int>{}); }
};
int main() { Queue q{{1, 2}}; auto jobs = q.drain(); assert(q.jobs.empty()); }
```

```rust
struct Queue { jobs: Vec<i32> }
impl Queue {
    fn drain(&mut self) -> Vec<i32> { std::mem::take(&mut self.jobs) }
}
fn main() {
    let mut q = Queue { jobs: vec![1, 2] };
    let jobs = q.drain();
    assert!(q.jobs.is_empty());
    assert_eq!(jobs, [1, 2]);
}
```

Do not make destruction order a hidden dependency. Finish a protocol explicitly or arrange owners to  
express it. Rust `Drop` cannot await asynchronous cleanup. Rust 1.99 recommends against leaking a `Box`  
and later reclaiming it: use a matching ownership-transfer API instead.

### Observe member destruction order

Keep ordering dependencies explicit. This trace demonstrates the semantic difference; use explicit finish  
operations for a real protocol rather than depending on incidental field ordering.

```cpp
#include <cassert>
#include <vector>
struct Trace {
    std::vector<int>& log;
    int id;
    ~Trace() { log.push_back(id); }
};
struct Owner { Trace first; Trace second; };
int main() {
    std::vector<int> log;
    { Owner owner{{log, 1}, {log, 2}}; }
    assert((log == std::vector<int>{2, 1}));
}
```

```rust
use std::cell::RefCell;
struct Trace<'a> { log: &'a RefCell<Vec<i32>>, id: i32 }
impl Drop for Trace<'_> { fn drop(&mut self) { self.log.borrow_mut().push(self.id); } }
struct Owner<'a> { first: Trace<'a>, second: Trace<'a> }
fn main() {
    let log = RefCell::new(Vec::new());
    {
        let _owner = Owner {
            first: Trace { log: &log, id: 1 },
            second: Trace { log: &log, id: 2 },
        };
    }
    assert_eq!(*log.borrow(), [1, 2]);
}
```

## Pinning versus deleting moves

**Use:** `pin!`, `Box::pin`, and maintained projection helpers when a `!Unpin` value must remain at its  
location. Prefer external owned buffers or indices when that avoids self-reference. Do not use pinning as  
a general borrow-checker workaround.

**Difference:** C++ can delete or customize move construction. Rust has no user-defined move constructor;  
pinning makes an address-stability promise through a pointer API. A `Pin` handle may move; its `!Unpin`  
pointee must not be moved or invalidated contrary to the contract until dropped. `Pin` does not imply  
`'static`, ownership, or immutability. `Unpin` values do not gain an immovability restriction.

```cpp
#include <memory>
struct Stable {
    Stable() = default;
    Stable(const Stable&) = delete;
    Stable(Stable&&) = delete;
};
int main() { auto owner = std::make_unique<Stable>(); auto moved_owner = std::move(owner); }
```

```rust
use std::marker::PhantomPinned;
struct Stable { _pin: PhantomPinned }
fn main() {
    let owner = Box::pin(Stable { _pin: PhantomPinned });
    let moved_owner = owner; // Handle moves; the pinned allocation does not.
    assert_eq!(std::mem::size_of_val(moved_owner.as_ref().get_ref()), 0);
}
```

## Sources

- [Reference: pointers](https://doc.rust-lang.org/reference/types/pointer.html)
- [Book: borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Book: lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Interior mutability](https://doc.rust-lang.org/std/cell/index.html)
- [Arc contracts](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [Destructors](https://doc.rust-lang.org/reference/destructors.html)
- [Pin contracts](https://doc.rust-lang.org/std/pin/index.html)
- [Nomicon: leaked guards](https://doc.rust-lang.org/nomicon/leaking.html)
