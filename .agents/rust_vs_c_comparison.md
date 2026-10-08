# Modern Rust (1.99.0) vs Modern C++ (C++20/23/26): A Deep Dive

Both modern C++ and modern Rust are zero-overhead, ahead-of-time compiled systems programming languages. However, they approach the problem of resource management, concurrency, and memory safety from fundamentally opposing philosophies.

Modern C++ is **unsafe by default, but safe by convention**. It provides a massive toolbox of features (smart pointers, concepts, ranges) to write safe code, but relies entirely on developer discipline and static analysis tools to enforce those rules.

Modern Rust is **safe by default, but unsafe by explicit boundary**. It builds safety into the compiler itself. By version 1.99.0, Rust's borrow checker (including next-generation Polonius analysis) is incredibly smart at mathematically proving memory and thread safety before the code is ever allowed to run, reducing false positives while maintaining strict guarantees.

Here is a specific, practical breakdown of their major differences.

---

## 1. Ownership, Slices, and Concurrency

### Ownership and Use-After-Move

In C++, resource management relies on RAII (Resource Acquisition Is Initialization). C++11 introduced move semantics (`std::move`) to transfer ownership of resources without copying them. However, C++ does not strictly track the "moved" state at compile time. Using a variable after moving it is a classic C++ logic error that leads to a "valid but unspecified state."

Rust enforces strict **Ownership**. Every value has a single owner. When ownership is transferred (moved), the compiler explicitly invalidates the old variable. You simply *cannot* compile code that uses a moved value.

**C++20 Example (Compiles, but causes silent bugs):**
```cpp
#include <iostream>
#include <string>
#include <utility>

int main() {
    std::string a = "Hello, World!";
    std::string b = std::move(a); // Ownership of the heap buffer moved to b
    
    // VALID C++: 'a' is now in a valid but unspecified state (usually empty).
    // This compiles fine, but 'a' has lost its original data.
    std::cout << "A contains: " << a << std::endl; 
    return 0;
}
```

**Rust 1.99.0 Example (Fails to compile):**
```rust
fn main() {
    let a = String::from("Hello, World!");
    let b = a; // Ownership moved to b (implicit move for non-Copy types)
    
    // COMPILER ERROR: borrow of moved value: `a`
    // The compiler strictly prevents use-after-move.
    println!("A contains: {}", a); 
}
```

### Slices and Reference Invalidation

A "slice" is a view into a contiguous sequence of elements. C++20 introduced `std::span<T>`, which is highly efficient. However, because C++ lacks compile-time lifetime tracking, a `std::span` can easily outlive the collection it points to, resulting in a dangling pointer (use-after-free).

In Rust, a slice (`&[T]`) is a reference tied to a specific lifetime. The borrow checker enforces the golden rule of Rust references: **Mutability XOR Aliasing**. You can have multiple immutable references (`&T`) OR exactly one mutable reference (`&mut T`), but never both simultaneously. You cannot mutate a collection while references to its contents exist.

**C++20 Example (Compiles, crashes at runtime):**
```cpp
#include <vector>
#include <span>
#include <iostream>

int main() {
    std::vector<int> numbers = {1, 2, 3};
    std::span<int> slice = numbers; // slice points to numbers' buffer
    
    // We add an element. The vector reallocates its heap memory!
    numbers.push_back(4); 
    
    // RUNTIME CRASH / UNDEFINED BEHAVIOR:
    // The span 'slice' now points to freed memory.
    std::cout << slice[0] << std::endl; 
    return 0;
}
```

**Rust 1.99.0 Example (Fails to compile):**
```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    let slice: &[i32] = &numbers; // Immutable borrow occurs here
    
    // COMPILER ERROR: cannot borrow `numbers` as mutable because 
    // it is also borrowed as immutable.
    numbers.push(4); 
    
    println!("{}", slice[0]);
}
```

### Fearless Concurrency

Concurrency in C++ requires immense care. A `std::mutex` is entirely separate from the data it protects. The compiler will happily let you read or write to a variable without locking the mutex first, leading to data races.

Rust enforces thread safety at compile time using two core traits: 
* `Send` (safe to move across threads)
* `Sync` (safe to share references across threads)

Furthermore, a Rust `Mutex<T>` *owns* the data it protects. You cannot physically access the data inside without calling `.lock()`, which returns a smart pointer (`MutexGuard`) that allows access and automatically unlocks when it goes out of scope.

**Rust Mutex Example:**
```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    // The Mutex strictly encapsulates the integer
    let data = Mutex::new(5); 

    // std::thread::scope (stabilized in recent editions) allows borrowing local variables
    thread::scope(|s| {
        s.spawn(|| {
            // You MUST lock to get the value. 
            // lock() returns a Result to handle thread panics safely.
            let mut num = data.lock().unwrap();
            *num += 1;
        }); // 'num' goes out of scope here, dropping the lock automatically.
    });
}
```

---

## 2. Memory and Pointer Operations

Modern C++ has largely moved away from `new` and `delete` in favor of `std::unique_ptr` and `std::shared_ptr`. However, you still deal heavily with raw pointers (`*` and `&`), which can be null. C++ relies on the programmer to check for `nullptr`.

### Eradicating Null Pointers

Rust entirely eliminates null pointers in safe code. 
* To represent a value that might be absent, Rust uses the `Option<T>` enum (`Some(T)` or `None`). 
* The compiler forces you to explicitly handle the `None` case (usually via `match`, `if let`, or the newer `let else` syntax), completely eradicating "NullReferenceExceptions" or segfaults caused by null dereferencing.

**Zero-cost abstractions:** Because of a feature called "null pointer optimization," an `Option<&T>` or `Option<Box<T>>` in Rust takes up the exact same amount of memory as a raw C++ pointer. Under the hood, Rust uses the memory address `0` to represent `None`, but wraps it in strict type safety so you cannot accidentally dereference it.

### Heap Allocation

For heap allocation, Rust uses `Box::new()`, which acts identically to C++'s `std::make_unique()`. Both have zero overhead compared to manual `malloc` and automatically free memory when going out of scope. 

For reference counting, C++ uses `std::shared_ptr`. Rust splits this into two:
* `Rc<T>`: Non-atomic reference counting (faster, but restricted to a single thread; compiler prevents sending it across threads).
* `Arc<T>`: Atomic reference counting (thread-safe, equivalent to `std::shared_ptr`).

---

## 3. Conceptually: Unsafe Rust vs Modern C++

When a C++ programmer hears about `unsafe` Rust, they often mistakenly believe it "turns off the borrow checker entirely," making it exactly like C++. This is a profound misunderstanding of Rust's safety model.

### The C++ Model: Total Codebase Burden
In C++, the entire codebase is unsafe by default. A memory bug could originate from literally any line of code. If a large C++ application crashes with a segmentation fault, debugging it can require tracing logic through millions of lines of code, because *any* function could have performed an invalid pointer operation, an out-of-bounds array access, or a double-free.

### The Rust Model: Encapsulated Boundaries
In Rust, the compiler guarantees that **Safe Rust cannot cause undefined behavior or memory corruption**. 

However, operating systems, hardware drivers, and fundamental data structures (like `Vec` or `HashMap`) inherently require manipulating raw memory. The `unsafe` keyword acts as an explicit, highly visible boundary for these necessary operations. 

`unsafe` unlocks exactly five specific "superpowers":
1. Dereferencing a raw pointer (`*const T` or `*mut T`).
2. Calling an unsafe function or C-FFI (Foreign Function Interface).
3. Accessing or modifying a mutable static variable.
4. Implementing an unsafe trait.
5. Accessing fields of unions.

**Crucially, `unsafe` does NOT disable the borrow checker for regular references.** If you borrow a variable via `&mut` inside an `unsafe` block, the compiler still enforces ownership rules on that reference.

### The Philosophy of Containment
The philosophy of `unsafe` Rust is **containment**. You write the dangerous raw-pointer logic inside a small, heavily scrutinized `unsafe` block, and then wrap that block in a safe public API. 

For example, the standard library's `Vec<T>` relies heavily on `unsafe` code under the hood to manually manage memory reallocation. But its public API (`push`, `pop`, `get`) is strictly bound by safe Rust rules, making it 100% safe for the end-user.

**The Practical Difference:** If a Rust program suffers a segmentation fault, you do not look at the entire codebase. You *only* need to audit the `unsafe` blocks and the C-FFI boundaries. The search space for memory bugs is reduced from millions of lines of code to perhaps a few dozen.