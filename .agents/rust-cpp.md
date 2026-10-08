I answered from language semantics rather than fetching the 1.99.0 release notes. The core rules below have been stable across Rust 1.8x through 1.99 and the 2024 edition. Release notes mostly add library APIs and polish, so I can pull them if you want 1.99-specific items.

## 0. The core difference

C++ is unsafe by default. The compiler trusts you, and undefined behavior (UB) can occur anywhere. Rust is safe by default. The compiler proves memory and thread safety in the "safe subset", and you opt out explicitly with `unsafe`. Nearly every other difference follows from this, plus Rust's choices of destructive moves, immutability by default, and no implicit conversions.

---

## 1. Ownership, slices, concurrency

### Ownership and moves

Rust has three rules: each value has exactly one owner, the value is dropped when the owner goes out of scope, and ownership can be moved or borrowed.

```rust
let a = String::from("hi");
let b = a;                 // MOVE: a is now statically unusable
// println!("{a}");        // error[E0382]: borrow of moved value
let c = b.clone();         // deep copy must be explicit
```

```cpp
std::string a = "hi";
auto b = a;                // COPY by default
auto c = std::move(a);     // a is "valid but unspecified"; compiler won't stop you using it
```

| | C++ | Rust |
|---|---|---|
| Default on assignment/pass | copy | move |
| Moved-from object | exists, usable, unspecified state | inaccessible (compile error) |
| Move implementation | user-written move ctor/assign | always a bitwise memcpy, never user code |
| Copy | copy ctor, can be arbitrarily expensive | `Copy` marker trait (bitwise only); otherwise explicit `.clone()` |
| Rule of 0/3/5 | yes | none; just `Drop` (optional) |

Because moves are plain memcpy, Rust types are always trivially relocatable. The consequence is that self-referential types need `Pin`, which C++ doesn't need because you can delete or customize your move constructor. Since there is no moved-from state, Rust uses `mem::take`, `mem::replace` and `mem::swap` to take values out of `&mut` places:

```rust
let old = std::mem::take(&mut self.buf);   // leaves Default (empty Vec) behind
```

### Borrowing: aliasing XOR mutability

At any moment you may have many `&T` (shared, read-only) or exactly one `&mut T` (exclusive), never both. C++ references have no such rule.

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);                 // error: cannot borrow `v` as mutable (immutable borrow alive)
println!("{first}");
```
The equivalent C++ compiles, and `push_back` may reallocate, leaving `first` dangling (UB). Borrows end at their last use (non-lexical lifetimes), not at the end of the scope.

Lifetimes are compile-time-only annotations that relate the validity of references. They cost nothing at runtime:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { if x.len() > y.len() { x } else { y } }

fn bad(x: &str) -> &str { let s = x.to_uppercase(); &s }  // error: returns reference to local
```
C++ has no standard equivalent. Clang's `[[clang::lifetimebound]]` and static analyzers are partial and optional. Returning a reference to a local compiles, with only a warning at best.

### Slices

A slice is a borrowed view of contiguous memory, represented as a fat pointer `(ptr, len)` (16 bytes on 64-bit). `&[T]` is read-only, `&mut [T]` is mutable, and `&str` is a UTF-8 text slice.

```rust
let v = vec![10, 20, 30, 40];
let mid: &[i32] = &v[1..3];        // [20, 30], bounds-checked
let (l, r) = v.split_at(2);        // two disjoint views
// mid[5] -> panic (defined, safe), never UB
```

Compared to `std::span` and `std::string_view`:

- **Bounds:** Rust slices are bounds-checked on `[]` in safe code. `span::operator[]` is unchecked by default, though hardened standard library modes exist and C++26 adds `span::at`.
- **Lifetime:** the borrow checker ties the slice to its owner, so `v.push(..)` while a slice lives is a compile error. A `string_view` of a temporary `std::string` dangles silently.
- **Strings:** `String`/`&str` are guaranteed valid UTF-8. Indexing is by byte range and panics off a character boundary. There are no null terminators. C++ `std::string` is just bytes.
- **Mutable disjoint access:** `split_at_mut`, `chunks_mut` and `iter_mut` give provably non-overlapping `&mut` slices.

### Concurrency

Rust makes data races a compile error in safe code, via two auto-traits:

- `Send`: the type can be moved to another thread.
- `Sync`: `&T` can be shared between threads.

```rust
let rc = std::rc::Rc::new(5);
std::thread::spawn(move || println!("{rc}"));
// error: `Rc<i32>` cannot be sent between threads safely
```

Mutexes own their data, so you cannot touch it without locking:

```rust
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let hs: Vec<_> = (0..4).map(|_| {
    let c = Arc::clone(&counter);
    std::thread::spawn(move || { *c.lock().unwrap() += 1; })   // guard dropped = unlocked
}).collect();
for h in hs { h.join().unwrap(); }
```

Scoped threads (`std::thread::scope`) can borrow stack data safely, because the compiler knows all threads join before the scope ends:

```rust
let mut data = vec![1, 2, 3];
std::thread::scope(|s| {
    s.spawn(|| println!("{}", data.len()));
});                       // all joined here
data.push(4);             // fine
```

| | C++20/23 | Rust |
|---|---|---|
| Data race | UB, found by TSan if you're lucky | compile error (safe code) |
| Mutex/data link | separate (`mutex` + variable) | `Mutex<T>` wraps data |
| Atomics/memory model | `std::atomic`, C++11 model | `std::sync::atomic`, same orderings |
| Thread join on destroy | `std::jthread` | `JoinHandle`/`scope` |
| Async | stackless coroutines (`co_await`), needs hand-written promise types or a library | `async`/`await`, lazy futures, runtime is a crate (Tokio, smol) |
| Parallel algorithms | `std::execution` policies | Rayon (crate) |

Rust still permits deadlocks and logic races (race conditions), only not data races. A panic while holding a lock "poisons" the `Mutex`.

---

## 2. Memory and pointers

### Stack, heap, and RAII

Both languages are value-oriented with deterministic destruction. Rust has no `new`/`delete`, and heap allocation happens inside owning types (`Box`, `Vec`, `String`, `Rc`...). Destructors run in reverse declaration order via `Drop`. Leaking is safe in Rust (`mem::forget`), whereas memory-unsafe behavior is not.

### Smart pointer mapping

| C++ | Rust | Notes |
|---|---|---|
| `unique_ptr<T>` | `Box<T>` | never null, destructive move |
| `shared_ptr<T>` (atomic) | `Arc<T>` | |
| `shared_ptr<T>` (non-atomic use) | `Rc<T>` | not `Send`; cheaper |
| `weak_ptr<T>` | `Weak<T>` | from `Rc`/`Arc` |
| `mutable` member / const_cast tricks | `Cell`, `RefCell`, `Mutex`, `RwLock`, `OnceCell`/`OnceLock`, `LazyLock` | interior mutability |
| `T*` nullable | `Option<&T>` / `Option<Box<T>>` | null-pointer optimization: same size as a raw pointer |
| `T&` | `&T` / `&mut T` | with borrow rules |
| `variant<A,B>` of copy-on-write | `Cow<'a, T>` | borrowed-or-owned |
| `pmr` allocators | `allocator_api` (still unstable) | use `#[global_allocator]` on stable |

The runtime-checked path to shared mutation is `Rc<RefCell<T>>`. It panics instead of invoking UB if you violate the borrow rules:

```rust
let shared = Rc::new(RefCell::new(vec![1]));
let a = Rc::clone(&shared);
a.borrow_mut().push(2);
let _g1 = shared.borrow_mut();
let _g2 = shared.borrow_mut();   // panic: already borrowed
```
Reference cycles leak in both languages (fix with `Weak`).

### References are not C++ references or pointers

Safe Rust references are always non-null, aligned, and point to valid initialized data. There is no pointer arithmetic on them (no `*p++`), and you cannot compare or cast them to integers without going through raw pointers. Iteration uses iterators and slices instead:

```rust
for x in v.iter_mut() { *x *= 2; }          // instead of for(p=begin;p!=end;++p)
```

### Raw pointers

`*const T` and `*mut T` behave like C++ pointers: nullable, no lifetime, no borrow checking. Creating one is safe, and dereferencing one is `unsafe`.

```rust
let mut x = 10;
let p = &mut x as *mut i32;        // safe
unsafe { *p += 1; }                // deref requires unsafe

let arr = [1, 2, 3];
let q = arr.as_ptr();
let third = unsafe { *q.add(2) };  // arithmetic: add/sub/offset (UB if out of bounds)
let w = q.wrapping_add(100);       // allowed to compute; deref would still be UB
let n: *const i32 = std::ptr::null();
```

Pointer arithmetic is measured in elements, not bytes (`add`, like C++ `p + n`). Rust also has a formal provenance model, with the strict-provenance APIs (`with_addr`, `expose_provenance`) stable since 1.84. Casting a pointer through an integer is a deliberate, documented operation. To take a pointer to a field without creating an intermediate reference (needed for packed or uninitialized data), use `&raw const`/`&raw mut`.

### Uninitialized memory

Safe Rust cannot read uninitialized values, and the compiler rejects possibly-uninitialized variables outright:

```rust
let x: i32;
println!("{x}");                   // error: used binding `x` isn't initialized
```
In C++, `int x; use(x);` was UB. C++26 reclassifies this as "erroneous behavior" (well-defined but wrong). For genuinely uninitialized buffers, Rust uses `MaybeUninit<T>` (an unsafe-to-finish API).

### Dynamic dispatch and function pointers

`&dyn Trait` and `Box<dyn Trait>` are fat pointers, `(data_ptr, vtable_ptr)`. The vtable pointer lives in the reference, not inside the object. So any type, including `i32` or foreign types, can implement a trait and be used as a trait object, and no hidden vptr bloats your structs.

Function types are `fn(i32) -> i32` (pointer, no capture). Closures implement `Fn`/`FnMut`/`FnOnce` depending on how they use captures. `Box<dyn Fn(i32) -> i32>` plays the role of `std::function`.

### Memory layout

Rust's default struct layout is unspecified, and the compiler may reorder fields to minimize padding. For C-compatible layout use `#[repr(C)]`, and for a single-field wrapper `#[repr(transparent)]`. Zero-sized types are real (`()`, `PhantomData`), and enums use niches, so `Option<Box<T>>` is 8 bytes.

### Arithmetic and conversions (memory-adjacent)

- Signed overflow is UB in C++. In Rust it panics in debug builds and wraps in release (never UB). You choose explicitly with `checked_*`, `wrapping_*`, `saturating_*` and `overflowing_*`.
- There are no implicit numeric conversions in Rust; you write `x as u64` or `u64::from(x)`.

### Allocation

`Vec::push` aborts the process on out-of-memory (no `bad_alloc` exception). Use `try_reserve` for fallible allocation. Raw allocation (`std::alloc::{alloc, dealloc, Layout}`) is `unsafe`.

### Statics and globals

Rust statics require const-initialization, so there is no static initialization order problem. Use `LazyLock`/`OnceLock` for lazy init. `static mut` is discouraged, and in edition 2024 taking references to one is denied by default.

### Mutability and constness

`let` is immutable and `let mut` is mutable, the opposite of C++'s default. Rust has no const-qualified methods. Instead, `&self` versus `&mut self` encodes it. `const fn` is roughly `constexpr`, and const generics are roughly non-type template parameters.

---

## 3. Unsafe Rust versus C++

`unsafe` unlocks exactly five things: dereferencing raw pointers, calling `unsafe` functions, accessing or modifying `static mut`, implementing `unsafe` traits (`Send`/`Sync`), and reading `union` fields (plus declaring `unsafe extern` blocks in edition 2024). It does not turn off the borrow checker or type checking.

The conceptual difference comes down to four points:

1. **Partition versus blanket.** C++ has no safe subset, so any line can hit UB. Rust splits code into a safe subset, which promises no UB regardless of how it's used, and a small, grep-able unsafe set.
2. **Soundness contracts.** An `unsafe fn` declares preconditions (documented under `# Safety`). An `unsafe {}` block is your signed claim that you've met them. A safe function must be impossible to misuse into UB. If a caller of a safe API can trigger UB, the bug is in the API author's unsafe code. C++ has no such contract boundary.
3. **Encapsulation.** The idiom is a small unsafe core behind a safe API. `Vec`, `Mutex` and `Rc` are exactly that.
4. **Unsafe Rust is not "C++ mode".** It still must uphold Rust's rules: `&mut` is assumed non-aliased (the compiler emits `noalias`), references must always be valid, and `bool` must be 0 or 1. Violating these is UB even if you never dereference. That is why unsafe code tends to use raw pointers internally and why people run Miri to detect violations.

A safe abstraction over unsafe code, mirroring the standard library's `split_at_mut`:

```rust
use std::slice;

fn split_at_mut(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = s.len();
    assert!(mid <= len);                 // establish the precondition
    let p = s.as_mut_ptr();
    unsafe {
        // SAFETY: the two ranges are in-bounds and disjoint
        (slice::from_raw_parts_mut(p, mid),
         slice::from_raw_parts_mut(p.add(mid), len - mid))
    }
}
```
Callers of this function see a fully safe API. The borrow checker alone cannot prove disjointness, so the author did.

Edition 2024 tightened this model: `unsafe_op_in_unsafe_fn` warns by default (each unsafe operation inside an unsafe fn needs its own `unsafe {}` block), `extern` blocks are written `unsafe extern`, and attributes like `no_mangle` become `#[unsafe(no_mangle)]`.

On the C++ side, there is no `unsafe` keyword. Tools such as `reinterpret_cast`, `memcpy`, `std::bit_cast` and `volatile` are all available everywhere. Mitigations are external or partial: sanitizers, hardened standard libraries, C++26 erroneous behavior for uninitialized reads, and contracts. Lifetime or safety profiles have been proposed but are not a language-level guarantee.

For interop, C and Rust share `extern "C"` with `#[repr(C)]`. For C++, use `bindgen` or `cxx`, because Rust has no stable ABI and cannot directly consume C++ templates or exceptions.

---

## 4. Other practical differences

| Topic | Modern C++ (20/23) | Rust |
|---|---|---|
| Errors | exceptions, `std::expected` (C++23), error codes | `Result<T,E>` + `?`; `panic!` for bugs (unwind or abort) |
| Sum types | `std::variant` + `visit` | `enum` with data + exhaustive `match` |
| Null | raw pointers, `optional` | `Option<T>`; no null in safe code |
| Generics | templates, checked at instantiation; concepts add constraints | traits, bounds checked at definition; monomorphized |
| Polymorphism | inheritance, virtual, overloading | traits only; no inheritance, no overloading, no default args |
| Construction | constructors, init lists | struct literals plus `new()` convention; no constructors |
| Metaprogramming | templates, `constexpr`/`consteval` | `macro_rules!`, procedural macros, `const fn` |
| Iteration | ranges (C++20) | lazy iterator adapters, zero-cost |
| Modules/build | headers, modules (uneven support), CMake/vcpkg/Conan | crates/modules, Cargo as the one standard tool |
| ABI | de facto per platform | none stable for Rust types |
| Compile model | long compile times from templates | also slow, but incremental and parallel; the borrow checker adds negligible cost |
| Tooling | external clang-tidy, sanitizers | built-in `cargo test`, `clippy`, `rustfmt`, Miri |
| Stdlib | very broad (regex, etc.) | smaller; regex, rand, serde are crates |
| Evolution | 3-year ISO standard | 6-week releases, editions for opt-in breaking changes |

**Practical takeaway.** Rust front-loads effort into satisfying the compiler (ownership design, lifetimes), and in return eliminates use-after-free, double free, iterator invalidation, data races and uninitialized reads in safe code. C++ gives more freedom (inheritance, placement control, mature ecosystems and compile-time programming) but you carry the proof burden. In practice, a C++ codebase's safety depends on discipline and tooling, while in Rust it is a property of the language, localized to your `unsafe` blocks.




# Modern Rust vs Modern C++: A Conceptual and Practical Deep Dive


Both modern C++ (C++20 and beyond) and modern Rust (up through its latest iterations) are highly capable, zero-overhead systems programming languages. However, they approach the problem of resource management and safety from fundamentally opposing philosophies.

Modern C++ is **unsafe by default, but safe by convention**. It gives you a massive toolbox of modern features (smart pointers, concepts, ranges) to write safe code, but relies on developer discipline to enforce those rules.

Modern Rust is **safe by default, but unsafe by explicit boundary**. It builds safety into the compiler itself, mathematically proving memory and thread safety before the code is ever allowed to run.

Here is a deep dive into the major practical and conceptual differences.

---

### 1. Ownership, Slices, and Concurrency

#### Ownership and Use-After-Move

In C++, resource management relies on RAII (Resource Acquisition Is Initialization) and destructors. C++11 introduced move semantics (`std::move`) to transfer ownership of resources (like heap memory) without copying them. However, C++ does not strictly track the "moved" state at compile time. Using a variable after moving it is a classic C++ logic error that leads to an "unspecified but valid state."

Rust introduces the **Borrow Checker**. Every value has a single owner. When ownership is transferred (moved), the compiler explicitly invalidates the old variable. You simply *cannot* compile code that uses a moved value.

**C++20 Example (Compiles, but causes bugs):**

```cpp
#include <iostream>
#include <string>

int main() {
    std::string a = "Hello";
    std::string b = std::move(a); // Ownership moved to b
    
    // VALID C++: 'a' is now in a valid but unspecified state (usually empty).
    // This compiles fine, but 'a' has lost its original data.
    std::cout << a << std::endl; 
    return 0;
}

```

**Rust Example (Fails to compile):**

```rust
fn main() {
    let a = String::from("Hello");
    let b = a; // Ownership moved to b (implicit move for non-Copy types)
    
    // COMPILER ERROR: value borrowed here after move
    println!("{}", a); 
}

```

#### Slices and Reference Invalidation

A "slice" is a view into a contiguous sequence of elements. C++20 introduced `std::span<T>`, which is highly efficient. However, because C++ lacks lifetime tracking, a `std::span` can easily outlive the data it points to, resulting in a dangling pointer (use-after-free).

In Rust, a slice (`&[T]`) is a reference. The borrow checker enforces the rule of references: **you cannot mutate a collection while references to its contents exist.**

**C++20 Example (Compiles, crashes at runtime):**

```cpp
#include <vector>
#include <span>
#include <iostream>

int main() {
    std::vector<int> numbers = {1, 2, 3};
    std::span<int> slice = numbers; 
    
    // We add an element. The vector reallocates its heap memory.
    numbers.push_back(4); 
    
    // RUNTIME CRASH / UNDEFINED BEHAVIOR:
    // The slice now points to freed memory.
    std::cout << slice[0] << std::endl; 
    return 0;
}

```

**Rust Example (Fails to compile):**

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    let slice: &[i32] = &numbers; // Immutable borrow occurs here
    
    // COMPILER ERROR: cannot borrow `numbers` as mutable because 
    // it is also borrowed as immutable
    numbers.push(4); 
    
    println!("{}", slice[0]);
}

```

#### Fearless Concurrency

Concurrency in C++ requires immense care. A `std::mutex` is entirely separate from the data it protects. The compiler will happily let you read or write to a variable without locking the mutex first, leading to data races.

Rust enforces thread safety at compile time using two traits: `Send` (safe to move across threads) and `Sync` (safe to reference across threads). Furthermore, a Rust `Mutex<T>` *owns* the data it protects. You cannot physically access the data inside without calling `.lock()`, which returns a smart pointer (`MutexGuard`) that automatically unlocks when it goes out of scope.

**Rust Mutex Example:**

```rust
use std::sync::Mutex;

let m = Mutex::new(5); // The Mutex encapsulates the integer

{
    // You MUST lock to get the value. lock() returns a Result to handle thread panics.
    let mut num = m.lock().unwrap();
    *num = 6;
} // 'num' goes out of scope here, dropping the lock automatically.

```

---

### 2. Memory and Pointer Operations

Modern C++ has largely moved away from `new` and `delete` in favor of `std::unique_ptr` and `std::shared_ptr`. However, you still deal with raw pointers (`*` and `&`), which can be null. C++ relies on the programmer to check for `nullptr`.

Rust entirely eliminates null pointers in safe code.

* To represent a value that might be absent, Rust uses the `Option<T>` enum (`Some(T)` or `None`).
* The compiler forces you to explicitly handle the `None` case (usually via `match` or `if let`), completely eradicating the "NullReferenceException" or segfaults caused by null dereferencing.

**Zero-cost abstractions:** Because of a feature called "null pointer optimization," an `Option<&T>` in Rust takes up the exact same amount of memory as a raw C++ pointer. Under the hood, Rust uses the memory address `0` to represent `None`, but wraps it in strict type safety so you can't accidentally dereference it.

For heap allocation, Rust uses `Box::new()`, which acts identically to C++'s `std::make_unique()`. Both have exactly zero overhead compared to manual `malloc` and automatically free memory when going out of scope.

---

### 3. Conceptually: Unsafe Rust vs Modern C++

When a C++ programmer hears about `unsafe` Rust, they often mistakenly believe it turns off the borrow checker entirely, making it "just like C++." This is a profound misunderstanding of Rust's safety model.

#### The C++ Model: Total Codebase Burden

In C++, the entire codebase is unsafe by default. A memory bug could originate from literally any line of code. If a large C++ application crashes with a segmentation fault, debugging it can require tracing logic through millions of lines of code, because any function *could* have performed an invalid pointer operation.

#### The Rust Model: Encapsulated Boundaries

In Rust, the compiler guarantees that **Safe Rust cannot cause undefined behavior or memory corruption**. Period.

However, operating systems, hardware drivers, and fundamental data structures require manipulating raw memory. The `unsafe` keyword acts as an explicit boundary for these operations. `unsafe` unlocks exactly five superpowers:

1. Dereferencing a raw pointer (`*const T` or `*mut T`).
2. Calling an unsafe function or C-FFI (Foreign Function Interface).
3. Accessing or modifying a mutable static variable.
4. Implementing an unsafe trait.
5. Accessing fields of unions.

**Crucially, `unsafe` does not disable the borrow checker for regular references.**

The philosophy of `unsafe` Rust is **containment**. You write the dangerous raw-pointer logic inside a small, heavily scrutinized `unsafe` block, and then wrap that block in a safe API. The standard library's `Vec<T>` is heavily implemented using `unsafe` code under the hood (to manually manage heap allocations), but its public API is 100% safe to use.

If a Rust program suffers a segmentation fault, you do not look at the entire codebase. You *only* need to audit the `unsafe` blocks and the C-FFI boundaries. The search space for memory bugs is reduced from millions of lines of code to perhaps a few dozen.