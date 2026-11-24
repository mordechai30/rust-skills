Source PDF: <file:///Users/motyzas/Downloads/C-to-Rust/Rust-for-C_CPP-Programmers.pdf>

<a id="page-1"></a>

<a id="dest-1-158"></a>

## [Rust Bootstrap Course for C/C++](<#dest-1-158>) [Programmers](<#dest-1-158>)

<a id="dest-1-1252"></a>

### [Course Overview](<#dest-1-1252>)

- Course overview
    - The case for Rust (from both C and C++ perspectives)
    - Local installation
    - Types, functions, control flow, pattern matching
    - Modules, cargo
    - Traits, generics
    - Collections, error handling
    - Closures, memory management, lifetimes, smart pointers
    - Concurrency
    - Unsafe Rust, including Foreign Function Interface (FFI)
    - `no_std` and embedded Rust essentials for firmware teams
    - Case studies: real-world C++ to Rust translation patterns
- We’ll not cover `async` Rust in this course — see the companion [Async Rust Training](<https://microsoft.github.io/RustTraining/async-book>) for a full treatment of futures, executors, `Pin` , tokio, and production async patterns

<a id="dest-1-4402"></a>

## [Self-Study Guide](<#dest-1-4402>)

This material works both as an instructor-led course and for self-study. If you’re working through it on your own, here’s how to get the most out of it:

#### **Pacing recommendations:**

![Source region, PDF page 1](Rust-for-C_CPP-Programmers-images/page-0001-region-0001.png)

> PDF page 1: Complex table retained as a source image.

<a id="page-2"></a>

![Source region, PDF page 2](Rust-for-C_CPP-Programmers-images/page-0002-region-0002.png)

> PDF page 2: Complex table retained as a source image.

#### **How to use the exercises:**

- Every chapter has hands-on exercises marked with difficulty: 🟢 Starter, 🟡 Intermediate, 🔴 Challenge
- Always try the exercise before expanding the solution. Struggling with the borrow checker is part of learning — the compiler’s error messages are your teacher
- If you’re stuck for more than 15 minutes, expand the solution, study it, then close it and try again from scratch
- The [Rust Playground](<https://play.rust-lang.org/>) lets you run code without a local install

#### **When you hit a wall:**

- Read the compiler error message carefully — Rust’s errors are exceptionally helpful
- Re-read the relevant section; concepts like ownership (ch7) often click on the second pass
- The [Rust standard library docs](<https://doc.rust-lang.org/std/>) are excellent — search for any type or method
- For async patterns, see the companion [Async Rust Training](<https://microsoft.github.io/RustTraining/async-book>)

<a id="dest-2-4958"></a>

## [Table of Contents](<#dest-2-4958>)

<a id="dest-2-5730"></a>

### [Part I — Foundations](<#dest-2-5730>)

1. [1. Introduction and Motivation](<#page-2>)
    - [Speaker intro and general approach](<#dest-8-158>)
    - [The case for Rust](<#dest-8-3818>)
    - [How does Rust address these issues?](<#dest-9-172>)
    - [Other Rust USPs and features](<#dest-12-5452>) 2/289
- <a id="page-3"></a> [Quick Reference: Rust vs C/C++](<#dest-13-1395>)
- [Why C/C++ Developers Need Rust](<#dest-15-158>)
- [What Rust Eliminates — The Complete List](<#dest-15-3270>)
- [The Problems Shared by C and C++](<#dest-16-5362>)
- [C++ Adds More Problems on Top](<#dest-18-3405>)
- [How Rust Addresses All of This](<#dest-21-6518>)
- [2. Getting Started](<#page-3>)
    - [Enough talk already: Show me some code](<#dest-25-158>)
    - [Rust Local installation](<#dest-25-4912>)
    - [Rust packages (crates)](<#dest-26-338>)
    - [Example: cargo and crates](<#dest-28-4418>)
- [3. Basic Types and Variables](<#page-3>)
    - [Built-in Rust types](<#dest-29-158>)
    - [Rust type specification and assignment](<#dest-29-3968>)
    - [Rust type specification and inference](<#dest-30-172>)
    - [Rust variables and mutability](<#dest-30-3322>)
- [4. Control Flow](<#page-3>)
    - [Rust if keyword](<#dest-31-158>)
    - [Rust loops using while and for](<#dest-31-3990>)
    - [Rust loops using loop](<#dest-32-0>)
    - [Rust expression blocks](<#dest-32-3465>)
- [5. Data Structures and Collections](<#page-3>)
    - [Rust array type](<#dest-33-832>)
    - [Rust tuples](<#dest-34-1980>)
    - [Rust references](<#dest-34-5422>)

C++ References vs Rust References — Key Differences

    - [Rust slices](<#dest-35-3548>)
    - [Rust constants and statics](<#dest-35-6375>)
    - [Rust strings: String vs &amp;str](<#dest-36-1905>)
    - [Rust structs](<#page-38>)
- <a id="page-4"></a> [Rust Vec&lt;T&gt;](<#dest-40-2138>)
- [Rust HashMap](<#dest-41-4980>)
- [Exercise: Vec and HashMap](<#dest-42-2738>)
- [6. Pattern Matching and Enums](<#page-4>)
    - [Rust enum types](<#dest-47-158>)
    - [Rust match statement](<#dest-47-5528>)
    - [Exercise: Implement add and subtract using match and enum](<#dest-50-0>)
- [7. Ownership and Memory Management](<#page-4>)
    - [Rust memory management](<#dest-52-158>)
    - [Rust ownership, borrowing and lifetimes](<#dest-53-645>)
    - [Rust move semantics](<#dest-54-3255>)
    - [Rust Clone](<#dest-57-0>)
    - [Rust Copy trait](<#dest-57-5190>)
    - [Rust Drop trait](<#dest-58-1658>)
    - [Exercise: Move, Copy and Drop](<#dest-59-4215>)
    - [Rust lifetime and borrowing](<#dest-60-158>)
    - [Rust lifetime annotations](<#dest-60-4620>)
    - [Exercise: Slice storage with lifetimes](<#page-62>)
    - [Lifetime Elision Rules Deep Dive](<#dest-63-2445>)
    - [Rust Box&lt;T&gt;](<#dest-68-158>)
    - [Interior Mutability: Cell&lt;T&gt; and RefCell&lt;T&gt;](<#dest-71-4935>)
    - [Shared Ownership: Rc&lt;T&gt;](<#dest-73-0>)
    - [Exercise: Shared ownership and interior mutability](<#dest-75-1988>)
- [8. Modules and Crates](<#page-4>)
    - [Rust crates and modules](<#dest-77-158>)
    - [Exercise: Modules and functions](<#dest-77-3705>)
    - [Workspaces and crates (packages)](<#dest-78-2505>)
    - [Exercise: Using workspaces and package dependencies](<#dest-78-6480>)
    - [Using community crates from crates.io](<#page-79>)
    - [Crates dependencies and SemVer](<#dest-80-1912>)
    - [Exercise: Using the rand crate](<#dest-80-5925>)
    - [Cargo.toml and Cargo.lock](<#page-81>)
    - [Cargo test feature](<#dest-81-4020>)
- <a id="page-5"></a> [Other Cargo features](<#dest-82-2062>)
- [Testing Patterns](<#page-92>)
- [9. Error Handling](<#page-5>)
    - [Connecting enums to Option and Result](<#dest-101-930>)
    - [Rust Option type](<#dest-102-1860>)
    - [Rust Result type](<#dest-103-2198>)
    - [Exercise: log() function implementation with Option](<#dest-104-3278>)
    - [Rust error handling](<#page-104>)
    - [Exercise: error handling](<#dest-108-5482>)
    - [Error Handling Best Practices](<#dest-110-158>)
- [10. Traits and Generics](<#page-5>)
    - [Rust traits](<#dest-117-158>)
    - [C++ Operator Overloading → Rust std::ops Traits](<#dest-119-5168>)
    - [Exercise: Logger trait implementation](<#dest-124-3412>)

When to use enum vs dyn Trait Exercise: Trait constraints and generics

    - [Exercise: Think Before You Translate](<#dest-206-6795>)
    - [Rust generics](<#dest-129-158>)
    - [Exercise: Generics](<#dest-130-4088>)
    - [Combining Rust traits and generics](<#dest-130-5685>)
    - [Rust traits constraints in data types](<#dest-131-4748>)
    - [Rust type state pattern and generics](<#dest-132-6405>)
    - [Rust builder pattern](<#dest-134-885>)
- [11. Type System Advanced Features](<#page-5>)
    - [Rust From and Into traits](<#dest-135-158>)
    - [Exercise: From and Into](<#dest-135-4762>)
    - [Rust Default trait](<#page-135>)
    - [Other Rust type conversions](<#dest-136-6165>)
- [12. Functional Programming](<#page-5>)
    - [Rust closures](<#dest-138-930>)
- <a id="page-6"></a> [Exercise: Closures and capturing](<#dest-138-5842>)
- [Rust iterators](<#dest-139-615>)
- [Exercise: Rust iterators](<#dest-140-3075>)
- [Iterator Power Tools Reference](<#dest-146-930>)
- [13. Concurrency](<#page-6>)
    - [Rust concurrency](<#dest-151-158>)
    - [Why Rust prevents data races: Send and Sync](<#dest-159-0>)
    - [Exercise: Multi-threaded word count](<#dest-159-3668>)
- [14. Unsafe Rust and FFI](<#page-6>)
    - [Unsafe Rust](<#dest-160-832>)
    - [Simple FFI example](<#dest-161-3390>)
    - [Complex FFI example](<#dest-162-6585>)
    - [Ensuring correctness of unsafe code](<#dest-168-1950>)
    - [Exercise: Writing a safe FFI wrapper](<#dest-170-4088>)
- [Part II — Deep Dives](<#page-6>)
- [15. no\_std — Rust for Bare Metal](<#page-6>)
    - [What is no\_std?](<#dest-172-2722>)
    - [When to use no\_std vs std](<#dest-176-0>)
    - [Exercise: no\_std ring buffer](<#dest-176-3668>)
    - [Embedded Deep Dive](<#dest-178-158>)
- [16. Case Studies: Real-World C++ to Rust Translation](<#page-6>)
    - [Case Study 1: Inheritance hierarchy → Enum dispatch](<#dest-197-892>)
    - [Case Study 2: shared\_ptr tree → Arena/index pattern](<#dest-199-172>)

[Case Study 3: Framework communication → Lifetime borrowing](<https://microsoft.github.io/RustTraining/c-cpp-book/ch16-1-case-study-lifetime-borrowing.html#case-study-3-framework-communication--lifetime-borrowing>) [Case Study 4: God object → Composable state](<https://microsoft.github.io/RustTraining/c-cpp-book/ch16-1-case-study-lifetime-borrowing.html#case-study-4-god-object--composable-state>) [Case Study 5: Trait objects — when they ARE right](<https://microsoft.github.io/RustTraining/c-cpp-book/ch16-1-case-study-lifetime-borrowing.html#case-study-5-trait-objects--when-they-are-right>)

<a id="page-7"></a>

- [Part III — Best Practices &amp; Reference](<#page-7>)
- [17. Best Practices](<#page-7>)
    - [Rust Best Practices Summary](<#dest-209-158>)
    - [Avoiding excessive clone()](<#dest-218-930>)
    - [Avoiding unchecked indexing](<#dest-233-930>)
    - [Collapsing assignment pyramids](<#dest-242-158>)
    - [Capstone Exercise: Diagnostic Event Pipeline](<#dest-245-1605>)
    - [Logging and Tracing Ecosystem](<#page-248>)
- [18. C++ → Rust Semantic Deep Dives](<#page-7>)
    - [Casting, Preprocessor, Modules, volatile, static, constexpr, SFINAE, and more](<#page-254>)
- [19. Rust Macros](<#page-7>)
    - [Declarative macros ( `macro_rules!` )](<#dest-276-1898>)
    - [Common standard library macros](<#dest-280-5932>)
    - [Derive macros](<#dest-283-0>)
    - [Attribute macros](<#dest-284-4350>)
    - [Procedural macros](<#dest-285-4688>)
    - [When to use what: macros vs functions vs generics](<#dest-287-0>)
    - [Exercises](<#dest-287-4162>)

<a id="page-8"></a>

<a id="dest-8-158"></a>

## [Speaker intro and general approach](<#dest-8-158>)

**What you’ll learn:** Course structure, the interactive format, and how familiar C/C++ concepts map to Rust equivalents. This chapter sets expectations and gives you a roadmap for the rest of the book.

- Speaker intro
    - Principal Firmware Architect in Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) team
    - Industry veteran with expertise in security, systems programming (firmware, operating systems, hypervisors), CPU and platform architecture, and C++ systems
    - Started programming in Rust in 2017 (@AWS EC2), and have been in love with the language ever since
- This course is intended to be as interactive as possible
    - Assumption: You know C, C++, or both
    - Examples are deliberately designed to map familiar concepts to Rust equivalents
    - Please feel free to ask clarifying questions at any point of time
- Speaker is looking forward to continued engagement with teams

<a id="dest-8-3818"></a>

## [The case for Rust](<#dest-8-3818>)

Want to skip straight to code? Jump to [Show me some code](<#dest-25-158>)

Whether you’re coming from C or C++, the core pain points are the same: memory safety bugs that compile cleanly but crash, corrupt, or leak at runtime.

- Over 70% of CVEs are caused by memory safety issues — buffer overflows, dangling pointers, use-after-free
- C++ `shared_ptr` , `unique_ptr` , RAII, and move semantics are steps in the right direction, but they are bandaids, not cures — they leave use-after-move, reference cycles, iterator invalidation, and exception safety gaps wide open
- Rust provides the performance you rely on from C/C++, but with compile-time guarantees for safety

📖 Deep dive: See [Why C/C++ Developers Need Rust](<#dest-15-158>) for concrete vulnerability examples, the complete list of what Rust eliminates, and why C++ smart pointers aren’t enough

<a id="page-9"></a>

<a id="dest-9-172"></a>

## [How does Rust address these issues?](<#dest-9-172>)

<a id="dest-9-945"></a>

### [Buffer overflows and bounds violations](<#dest-9-945>)

- All Rust arrays, slices, and strings have explicit bounds associated with them. The compiler inserts checks to ensure that any bounds violation results in a runtime crash (panic in Rust terms) — never undefined behavior

<a id="dest-9-2310"></a>

### [Dangling pointers and references](<#dest-9-2310>)

- Rust introduces lifetimes and borrow checking to eliminate dangling references at compile time
- No dangling pointers, no use-after-free — the compiler simply won’t let you

<a id="dest-9-3682"></a>

### [Use-after-move](<#dest-9-3682>)

- Rust’s ownership system makes moves destructive — once you move a value, the compiler refuses to let you use the original. No zombie objects, no “valid but unspecified state”

<a id="dest-9-5048"></a>

### [Resource management](<#dest-9-5048>)

- Rust’s `Drop` trait is RAII done right — the compiler automatically frees resources when they go out of scope, and prevents use-after-move which C++ RAII cannot
- No Rule of Five needed (no copy ctor, move ctor, copy assign, move assign, destructor to define)

<a id="dest-9-6600"></a>

### [Error handling](<#dest-9-6600>)

- Rust has no exceptions. All errors are values ( `Result<T, E>` ), making error handling explicit and visible in the type signature 9/289

<a id="page-10"></a>

<a id="dest-10-0"></a>

## [Iterator invalidation](<#dest-10-0>)

- Rust’s borrow checker forbids modifying a collection while iterating over it. You simply cannot write the bugs that plague C++ codebases:

```
// Rust equivalent of erase-during-iteration: retain()
pending_faults.retain(|f| f.id != fault_to_remove.id);
// Or: collect into a new Vec (functional style)
let remaining: Vec<_> = pending_faults
    .into_iter()
    .filter(|f| f.id != fault_to_remove.id)
    .collect();
```

<a id="dest-10-2542"></a>

## [Data races](<#dest-10-2542>)

- The type system prevents data races at compile time through the `Send` and `Sync` traits

<a id="dest-10-3570"></a>

## [Memory Safety Visualization](<#dest-10-3570>)

<a id="dest-10-4170"></a>

### [Rust Ownership — Safe by Design](<#dest-10-4170>)

```
fn safe_rust_ownership() {
    // Move is destructive: original is gone
    let data = vec![1, 2, 3];
    let data2 = data;           // Move happens
    // data.len();              // Compile error: value used after move
    // Borrowing: safe shared access
    let owned = String::from("Hello, World!");
    let slice: &str = &owned;  // Borrow — no allocation
    println!("{}", slice);     // Always safe
    // No dangling references possible
    /*
    let dangling_ref;
    {
        let temp = String::from("temporary");
        dangling_ref = &temp;  // Compile error: temp doesn't live long enough
    }
    */
}
```

<a id="page-11"></a>

![Source region, PDF page 11](Rust-for-C_CPP-Programmers-images/page-0011-region-0005.png)

<a id="dest-11-1238"></a>

## [Memory Layout: Rust References](<#dest-11-1238>)

![Source region, PDF page 11](Rust-for-C_CPP-Programmers-images/page-0011-region-0004.png)

> PDF page 11: Complex table retained as a source image.

Borrow Checker

Prevents dangling refs at

Stack/Heap

compile time

Safe reference — Lifetime

checked

![Source region, PDF page 11](Rust-for-C_CPP-Programmers-images/page-0011-region-0003.png)

> PDF page 11: Mathematics retained as a source image.

<a id="dest-11-4605"></a>

### [Box&lt;T&gt; Heap Allocation Visualization](<#dest-11-4605>)

```
fn box_allocation_example() {
    // Stack allocation
    let stack_value = 42;
    // Heap allocation with Box
    let heap_value = Box::new(42);
    // Moving ownership
    let moved_box = heap_value;
    // heap_value is no longer accessible
}
```

<a id="page-12"></a>

![Source region, PDF page 12](Rust-for-C_CPP-Programmers-images/page-0012-region-0006.png)

<a id="dest-12-2715"></a>

## [Slice Operations Visualization](<#dest-12-2715>)

```
fn slice_operations() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let full_slice = &data[..];        // [1,2,3,4,5,6,7,8]
    let partial_slice = &data[2..6];   // [3,4,5,6]
    let from_start = &data[..4];       // [1,2,3,4]
    let to_end = &data[3..];           // [4,5,6,7,8]
}
```

![Source region, PDF page 12](Rust-for-C_CPP-Programmers-images/page-0012-region-0007.png)

<a id="dest-12-5452"></a>

## [Other Rust USPs and features](<#dest-12-5452>)

- No data races between threads (compile-time `Send` / `Sync` checking)
- No use-after-move (unlike C++ `std::move` which leaves zombie objects)
- No uninitialized variables
    - All variables must be initialized before use
- No trivial memory leaks
    - `Drop` trait = RAII done right, no Rule of Five needed
    - Compiler automatically releases memory when it goes out of scope
- No forgotten locks on mutexes 12/289
    - <a id="page-13"></a> Lock guards are the only way to access the data ( `Mutex<T>` wraps the data, not the access)
- No exception handling complexity
    - Errors are values ( `Result<T, E>` ), visible in function signatures, propagated with `?`
- Excellent support for type inference, enums, pattern matching, zero cost abstractions
- Built-in support for dependency management, building, testing, formatting, linting
    - `cargo` replaces make/CMake + lint + test frameworks

<a id="dest-13-1395"></a>

## [Quick Reference: Rust vs C/C++](<#dest-13-1395>)

Key Concept C C++ Rust Differe

`Box<T>` ,

Memory `unique_ptr` , `malloc()/free()` management

Automa no cycle

`Rc<T>` ,

```
shared_ptr
                      Arc<T>
```

Bounds checking default

`std::vector<T>` , Arrays `int arr[10]`

`Vec<T>` ,

```
std::array<T>         [T; N]
```

UTF-8 guarant lifetime- checked

`std::string` , Strings `char*` with `\0`

`String` ,

```
string_view           &str
```

Borrow checking lifetimes

`&T` , `&mut` References `T&` , `T&&` (move) `int* ptr`

```
T
```

Compos over inherita

Virtual functions, Polymorphism Function pointers inheritance

Traits, trait objects

Generics + trait bounds

Generic Macros ( `void*` ) Templates programming

Better e messag

```
                                           Result<T,
errno                std::optional
                                           Option<T>
```

Return codes, Error handling

Exceptions,

No hidd control

`E>` ,

NULL/null `nullptr` , `ptr == NULL` safety

Forced n `Option<T>` checking `std::optional<T>`

Compile- time guarantees

Manual Thread safety (pthreads)

Manual synchronization

Data rac impossi

<a id="page-14"></a>

Key Concept C C++ Rust Differe

| Build system | Make, CMake | CMake, Make, etc. | Cargo |
| --- | --- | --- | --- |
| Undefined behavior | Runtime crashes | Subtle UB (signed overflow, aliasing) | Compile- time errors |

Integrat toolchai

Safety guarant

<a id="page-15"></a>

<a id="dest-15-158"></a>

## [Why C/C++ Developers Need Rust](<#dest-15-158>)

#### **What you’ll learn:**

- The full list of problems Rust eliminates — memory safety, undefined behavior, data races, and more
- Why `shared_ptr` , `unique_ptr` , and other C++ mitigations are bandaids, not solutions
- Concrete C and C++ vulnerability examples that are structurally impossible in safe Rust

Want to skip straight to code? Jump to [Show me some code](<#dest-25-158>)

<a id="dest-15-3270"></a>

### [What Rust Eliminates — The Complete List](<#dest-15-3270>)

Before diving into examples, here’s the executive summary. Safe Rust **structurally prevents** every issue in this list — not through discipline, tooling, or code review, but through the type system and compiler:

![Source region, PDF page 15](Rust-for-C_CPP-Programmers-images/page-0015-region-0008.png)

> PDF page 15: Complex table retained as a source image.

<a id="page-16"></a>

![Source region, PDF page 16](Rust-for-C_CPP-Programmers-images/page-0016-region-0009.png)

> PDF page 16: Complex table retained as a source image.

Bottom line: These aren’t aspirational goals enforced by coding standards. They are compile-time guarantees. If your code compiles, these bugs cannot exist.

<a id="dest-16-5362"></a>

### [The Problems Shared by C and C++](<#dest-16-5362>)

Want to skip the examples? Jump to [How Rust Addresses All of This](<#dest-21-6518>) or straight to [Show](<#dest-25-158>) [me some code](<#dest-25-158>)

Both languages share a core set of memory safety problems that are the root cause of over 70% of CVEs (Common Vulnerabilities and Exposures):

<a id="page-17"></a>

<a id="dest-17-0"></a>

### [Buffer overflows](<#dest-17-0>)

C arrays, pointers, and strings have no intrinsic bounds. It is trivially easy to exceed them:

```
#include <stdlib.h>
#include <string.h>
void buffer_dangers() {
    char buffer[10];
    strcpy(buffer, "This string is way too long!");  // Buffer overflow
    int arr[5] = {1, 2, 3, 4, 5};
    int *ptr = arr;           // Loses size information
    ptr[10] = 42;             // No bounds check — undefined behavior
}
```

In C++, `std::vector::operator[]` still performs no bounds checking. Only `.at()` does — and who catches the exception?

<a id="dest-17-3082"></a>

### [Dangling pointers and use-after-free](<#dest-17-3082>)

```
int *bar() {
    int i = 42;
    return &i;    // Returns address of stack variable — dangling!
}
void use_after_free() {
    char *p = (char *)malloc(20);
    free(p);
    *p = '\0';   // Use after free — undefined behavior
}
```

<a id="dest-17-5258"></a>

### [Uninitialized variables and undefined behavior](<#dest-17-5258>)

C and C++ both allow uninitialized variables. The resulting values are indeterminate, and reading them is undefined behavior:

```
int x;               // Uninitialized
if (x > 0) { ... }  // UB — x could be anything
```

Integer overflow is **defined** in C for unsigned types but **undefined** for signed types. In C++, signed overflow is also undefined behavior. Both compilers can and do exploit this for “optimizations” that break programs in surprising ways.

<a id="page-18"></a>

<a id="dest-18-0"></a>

[NULL pointer dereferences](<#dest-18-0>)

```
int *ptr = NULL;
*ptr = 42;           // SEGV — but the compiler won't stop you
```

In C++, `std::optional<T>` helps but is verbose and often bypassed with `.value()` which throws.

<a id="dest-18-1568"></a>

[The visualization: shared problems](<#dest-18-1568>)

![Source region, PDF page 18](Rust-for-C_CPP-Programmers-images/page-0018-region-0011.png)

<a id="dest-18-3405"></a>

### [C++ Adds More Problems on Top](<#dest-18-3405>)

C audience: You can [skip ahead to How Rust Addresses These Issues](<#dest-21-6518>) if you don’t use C++.

Want to skip straight to code? Jump to [Show me some code](<#dest-25-158>)

C++ introduced smart pointers, RAII, move semantics, and exceptions to address C’s problems. These are **bandaids, not cures** — they shift the failure mode from “crash at runtime” to “subtler bug at runtime”:

<a id="dest-18-5550"></a>

[unique\_ptr and shared\_ptr — bandaids, not solutions](<#dest-18-5550>)

C++ smart pointers are a significant improvement over raw `malloc` / `free` , but they don’t solve the underlying problems:

![Source region, PDF page 18](Rust-for-C_CPP-Programmers-images/page-0018-region-0010.png)

> PDF page 18: Complex table retained as a source image.

<a id="page-19"></a>

![Source region, PDF page 19](Rust-for-C_CPP-Programmers-images/page-0019-region-0012.png)

> PDF page 19: Complex table retained as a source image.

```
// unique_ptr: use-after-move compiles cleanly
std::unique_ptr<int> ptr = std::make_unique<int>(42);
std::unique_ptr<int> ptr2 = std::move(ptr);
std::cout << *ptr;  // Compiles! Undefined behavior at runtime.
                     // In Rust, this is a compile error: "value used after move"
```

```
// shared_ptr: reference cycles leak silently
struct Node {
    std::shared_ptr<Node> next;
    std::shared_ptr<Node> parent;  // Cycle! Destructor never called.
};
auto a = std::make_shared<Node>();
auto b = std::make_shared<Node>();
a->next = b;
b->parent = a;  // Memory leak — ref count never reaches 0
                 // In Rust, Rc<T> + Weak<T> makes cycles explicit and breakable
```

<a id="dest-19-4830"></a>

### [Use-after-move — the silent killer](<#dest-19-4830>)

C++ `std::move` is not a move — it’s a cast. The original object remains in a “valid but unspecified state”. The compiler lets you keep using it:

```
auto vec = std::make_unique<std::vector<int>>({1, 2, 3});
auto vec2 = std::move(vec);
vec->size();  // Compiles! But dereferencing nullptr — crash at runtime
```

In Rust, moves are destructive. The original binding is gone:

```
let vec = vec![1, 2, 3];
let vec2 = vec;           // Move — vec is consumed
// vec.len();             // Compile error: value used after move
```

<a id="page-20"></a>

<a id="dest-20-0"></a>

### [Iterator invalidation — real bugs from production C++](<#dest-20-0>)

These aren’t contrived examples — they represent **real bug patterns** found in large C++ codebases:

```
// BUG 1: erase without reassigning iterator (undefined behavior)
while (it != pending_faults.end()) {
    if (*it != nullptr && (*it)->GetId() == fault->GetId()) {
        pending_faults.erase(it);   // ← iterator invalidated!
        removed_count++;            //   next loop uses dangling iterator
    } else {
        ++it;
    }
}
// Fix: it = pending_faults.erase(it);
```

```
// BUG 2: index-based erase skips elements
for (auto i = 0; i < entries.size(); i++) {
    if (config_status == ConfigDisable::Status::Disabled) {
        entries.erase(entries.begin() + i);  // ← shifts elements
    }                                         //   i++ skips the shifted one
}
```

```
// BUG 3: one erase path correct, the other isn't
while (it != incomplete_ids.end()) {
    if (current_action == nullptr) {
        incomplete_ids.erase(it);  // ← BUG: iterator not reassigned
        continue;
    }
    it = incomplete_ids.erase(it); // ← Correct path
}
```

**These compile without any warning.** In Rust, the borrow checker makes all three a compile error — you cannot mutate a collection while iterating over it, period.

<a id="dest-20-5542"></a>

### [Exception safety and the dynamic\_cast/new pattern](<#dest-20-5542>)

Modern C++ codebases still lean heavily on patterns that have no compile-time safety:

<a id="page-21"></a>

```
// Typical C++ factory pattern — every branch is a potential bug
DriverBase* driver = nullptr;
if (dynamic_cast<ModelA*>(device)) {
    driver = new DriverForModelA(framework);
} else if (dynamic_cast<ModelB*>(device)) {
    driver = new DriverForModelB(framework);
}
// What if driver is still nullptr? What if new throws? Who owns driver?
```

In a typical 100K-line C++ codebase you might find hundreds of `dynamic_cast` calls (each a potential runtime failure), hundreds of raw `new` calls (each a potential leak), and hundreds of

`virtual` / `override` methods (vtable overhead everywhere).

<a id="dest-21-2242"></a>

### [Dangling references and lambda captures](<#dest-21-2242>)

```
int& get_reference() {
    int x = 42;
    return x;  // Dangling reference — compiles, UB at runtime
}
auto make_closure() {
    int local = 42;
    return [&local]() { return local; };  // Dangling capture!
}
```

<a id="dest-21-4282"></a>

### [The visualization: C++ additional problems](<#dest-21-4282>)

![Source region, PDF page 21](Rust-for-C_CPP-Programmers-images/page-0021-region-0013.png)

<a id="dest-21-6518"></a>

## [How Rust Addresses All of This](<#dest-21-6518>)

Every problem listed above — from both C and C++ — is prevented by Rust’s compile-time guarantees:

<a id="page-22"></a>

![Source region, PDF page 22](Rust-for-C_CPP-Programmers-images/page-0022-region-0014.png)

> PDF page 22: Complex table retained as a source image.

<a id="page-23"></a>

```
fn rust_prevents_everything() {
    // ✅ No buffer overflow — bounds checked
    let arr = [1, 2, 3, 4, 5];
    // arr[10];  // panic at runtime, never UB
    // ✅ No use-after-move — compile error
    let data = vec![1, 2, 3];
    let moved = data;
    // data.len();  // error: value used after move
    // ✅ No dangling pointer — lifetime error
    // let r;
    // { let x = 5; r = &x; }  // error: x does not live long enough
    // ✅ No null — Option forces handling
    let maybe: Option<i32> = None;
    // maybe.unwrap();  // panic, but you'd use match or if let instead
    // ✅ No data race — compile error
    // let mut shared = vec![1, 2, 3];
    // std::thread::spawn(|| shared.push(4));  // error: closure may outlive
    // shared.push(5);                         //   borrowed value
}
```

<a id="dest-23-3788"></a>

### [Rust’s safety model — the full picture](<#dest-23-3788>)

Rust Safety Guarantees

Ownership System Borrow Checker Type System Send/Sync Traits

No use-after-free No use-after-move

No dangling references No iterator invalidation No data races through refs

No NULL (Option&lt;T&gt;) No exceptions (Result&lt;T,E&gt;)

No data races Send = safe to transfer

No double-free

No uninitialized values

![Source region, PDF page 23](Rust-for-C_CPP-Programmers-images/page-0023-region-0015.png)

> PDF page 23: Mathematics retained as a source image.

<a id="dest-23-6172"></a>

## [Quick Reference: C vs C++ vs Rust](<#dest-23-6172>)

Ke Concept C C++ Rust Differ

Memory management

`unique_ptr` , `malloc()/free()`

`Box<T>` ,

Automa no cycl

`Rc<T>` ,

```
shared_ptr
```

<a id="page-24"></a>

Ke Concept C C++ Rust Differ

no zom `Arc<T>`

Bounds checkin default

`std::vector<T>` , Arrays `int arr[10]`

`Vec<T>` ,

```
std::array<T>         [T; N]
```

UTF-8 guaran lifetime checke

`std::string` , Strings `char*` with `\0`

`String` ,

```
string_view           &str
```

Lifetim borrow checkin

`&T` , `&mut`

References `int*` (raw) `T&` , `T&&` (move)

```
T
```

Compo over inherita

Virtual functions, Polymorphism Function pointers inheritance

Traits, trait objects

Generics + trait bounds

Clear e messag

Generics Macros / `void*` Templates

```
                                           Result<T,
errno                std::optional
                                           Option<T>
```

Return codes, Error handling

Exceptions,

No hidd control

`E>` ,

`nullptr` , NULL safety `ptr == NULL`

Forced `Option<T>` checkin `std::optional<T>`

Compile- time

Manual Thread safety (pthreads)

Manual ( `std::mutex` , etc.)

Data ra imposs

`Send` / `Sync`

Integra Build system Make, CMake CMake, Make, etc. Cargo toolcha

Undefined Subtle (signed Rampant behavior overflow, aliasing)

Zero in safe code

Safety guaran

<a id="page-25"></a>

<a id="dest-25-158"></a>

## [Enough talk already: Show me some code](<#dest-25-158>)

What you’ll learn: Your first Rust program — `fn main()` , `println!()` , and how Rust macros differ fundamentally from C/C++ preprocessor macros. By the end you’ll be able to write, compile, and run simple Rust programs.

```
fn main() {
    println!("Hello world from Rust");
}
```

- The above syntax should be similar to anyone familiar with C-style languages
    - All functions in Rust begin with the `fn` keyword
    - The default entry point for executables is `main()`
    - The `println!` looks like a function, but is actually a macro. Macros in Rust are very different from C/C++ preprocessor macros — they are hygienic, type-safe, and operate on the syntax tree rather than text substitution
- Two great ways to quickly try out Rust snippets:
    - Online: [Rust Playground](<https://play.rust-lang.org/>) — paste code, hit Run, share results. No install needed
    - Local REPL: Install [`evcxr_repl`](<https://github.com/evcxr/evcxr>) for an interactive Rust REPL (like Python’s REPL, but for Rust):

```
cargo install --locked evcxr_repl
evcxr   # Start the REPL, type Rust expressions interactively
```

<a id="dest-25-4912"></a>

[Rust Local installation](<#dest-25-4912>)

- Rust can be locally installed using the following methods
    - Windows: https://static.rust-lang.org/rustup/dist/x86\_64-pc-windows-msvc/rustup- init.exe
    - Linux / WSL: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- The Rust ecosystem is composed of the following components
    - `rustc` is the standalone compiler, but it’s seldom used directly
    - The preferred tool, `cargo` is the Swiss Army knife and is used for dependency management, building, testing, formatting, linting, etc.
    - The Rust toolchain comes in the `stable` , `beta` and `nightly` (experimental) channels, but we’ll stick with `stable` . Use the `rustup update` command to upgrade the `stable` installation that’s released every six weeks
- <a id="page-26"></a> We’ll also install the `rust-analyzer` plug-in for VSCode

<a id="dest-26-338"></a>

## [Rust packages (crates)](<#dest-26-338>)

- Rust binaries are created using packages (hereby called crates)
    - A crate may either be standalone, or may have dependency on other crates. The crates for the dependencies can be local or remote. Third-party crates are typically downloaded from a centralized repository called `crates.io` .
    - The `cargo` tool automatically handles the downloading of crates and their dependencies. This is conceptually equivalent to linking to C-libraries
    - Crate dependencies are expressed in a file called `Cargo.toml` . It also defines the target type for the crate: standalone executable, static library, dynamic library (uncommon)
    - Reference: https://doc.rust-lang.org/cargo/reference/cargo-targets.html

<a id="page-27"></a>

### 10/25/26, 2:04 AM Rust for C/C++ Programmers

<a id="dest-27-0"></a>

## [Cargo vs Traditional C Build Systems](<#dest-27-0>)

<a id="dest-27-592"></a>

## [Dependency Management Comparison](<#dest-27-592>)

![Source region, PDF page 27](Rust-for-C_CPP-Programmers-images/page-0027-region-0016.png)

<a id="dest-27-4695"></a>

## [Cargo Project Structure](<#dest-27-4695>)

```
my_project/
|-- Cargo.toml          # Project configuration (like package.json)
|-- Cargo.lock          # Exact dependency versions (auto-generated)
|-- src/
|   |-- main.rs         # Main entry point for binary
|   |-- lib.rs          # Library root (if creating a library)
|   `-- bin/            # Additional binary targets
|-- tests/              # Integration tests
|-- examples/           # Example code
|-- benches/            # Benchmarks
`-- target/             # Build artifacts (like C's build/ or obj/)
    |-- debug/          # Debug builds (fast compile, slow runtime)
    `-- release/        # Release builds (slow compile, fast runtime)
```

### https://microsoft.github.io/RustTraining/c-cpp-book/print.html 27/289

<a id="page-28"></a>

### 10/25/26, 2:04 AM Rust for C/C++ Programmers

<a id="dest-28-0"></a>

## [Common Cargo Commands](<#dest-28-0>)

![Source region, PDF page 28](Rust-for-C_CPP-Programmers-images/page-0028-region-0017.png)

<a id="dest-28-4418"></a>

## [Example: cargo and crates](<#dest-28-4418>)

- In this example, we have a standalone executable crate with no other dependencies
- Use the following commands to create a new crate called `helloworld`

```
cargo new helloworld
cd helloworld
cat Cargo.toml
```

- By default, `cargo run` will compile and run the `debug` (unoptimized) version of the crate. To execute the `release` version, use `cargo run --release`
- Note that actual binary file resides under the `target` folder under the `debug` or `release` folder
- We might have also noticed a file called `Cargo.lock` in the same folder as the source. It is automatically generated and should not be modified by hand
    - We will revisit the specific purpose of `Cargo.lock` later

### https://microsoft.github.io/RustTraining/c-cpp-book/print.html 28/289

<a id="page-29"></a>

<a id="dest-29-158"></a>

## [Built-in Rust types](<#dest-29-158>)

What you’ll learn: Rust’s fundamental types ( `i32` , `u64` , `f64` , `bool` , `char` ), type inference, explicit type annotations, and how they compare to C/C++ primitive types. No implicit conversions — Rust requires explicit casts.

- Rust has type inference, but also allows explicit specification of the type

![Source region, PDF page 29](Rust-for-C_CPP-Programmers-images/page-0029-region-0018.png)

> PDF page 29: Complex table retained as a source image.

- Rust permits arbitrary use of `_` between numbers for ease of reading

<a id="dest-29-3968"></a>

### [Rust type specification and assignment](<#dest-29-3968>)

- Rust uses the `let` keyword to assign values to variables. The type of the variable can be optionally specified after a `:`

```
fn main() {
    let x : i32 = 42;
    // These two assignments are logically equivalent
    let y : u32 = 42;
    let z = 42u32;
}
```

- Function parameters and return values (if any) require an explicit type. The following takes a u8 parameter and returns u32

```
fn foo(x : u8) -> u32
{
    return x as u32 * x as u32;
}
```

- Unused variables are prefixed with `_` to avoid compiler warnings

<a id="page-30"></a>

<a id="dest-30-172"></a>

## [Rust type specification and inference](<#dest-30-172>)

- Rust can automatically infer the type of the variable based on the context.
- [▶ Try it in the Rust Playground](<https://play.rust-lang.org/>)

```
fn secret_of_life_u32(x : u32) {
    println!("The u32 secret_of_life is {}", x);
}
fn secret_of_life_u8(x : u8) {
    println!("The u8 secret_of_life is {}", x);
}
fn main() {
    let a = 42; // The let keyword assigns a value; type of a is u32
    let b = 42; // The let keyword assigns a value; inferred type of b is u8
    secret_of_life_u32(a);
    secret_of_life_u8(b);
}
```

<a id="dest-30-3322"></a>

## [Rust variables and mutability](<#dest-30-3322>)

- Rust variables are immutable by default unless the `mut` keyword is used to denote that a variable is mutable. For example, the following code will not compile unless the `let a =`

![Source region, PDF page 30](Rust-for-C_CPP-Programmers-images/page-0030-region-0019.png)

> PDF page 30: Mathematics retained as a source image.

```
fn main() {
    let a = 42; // Must be changed to let mut a = 42 to permit the assignment
below
    a = 43;  // Will not compile unless the above is changed
}
```

- Rust permits the reuse of the variable names (shadowing)

```
fn main() {
    let a = 42;
    {
        let a = 43; //OK: Different variable with the same name
    }
    // a = 43; // Not permitted
    let a = 43; // Ok: New variable and assignment
}
```

<a id="page-31"></a>

<a id="dest-31-158"></a>

## [Rust if keyword](<#dest-31-158>)

What you’ll learn: Rust’s control flow constructs — `if` / `else` as expressions,

`loop` / `while` / `for` , `match` , and how they differ from C/C++ counterparts. The key insight: most Rust control flow returns values.

- In Rust, `if` is actually an expression, i.e., it can be used to assign values, but it also behaves like a statement. [▶ Try it](<https://play.rust-lang.org/>)

```
fn main() {
    let x = 42;
    if x < 42 {
        println!("Smaller than the secret of life");
    } else if x == 42 {
        println!("Is equal to the secret of life");
    } else {
        println!("Larger than the secret of life");
    }
    let is_secret_of_life = if x == 42 {true} else {false};
    println!("{}", is_secret_of_life);
}
```

<a id="dest-31-3990"></a>

## [Rust loops using while and for](<#dest-31-3990>)

- The `while` keyword can be used to loop while an expression is true

```
fn main() {
    let mut x = 40;
    while x != 42 {
        x += 1;
    }
}
```

- The `for` keyword can be used to iterate over ranges

```
fn main() {
    // Will not print 43; use 40..=43 to include last element
    for x in 40..43 {
        println!("{}", x);
    }
}
```

<a id="page-32"></a>

<a id="dest-32-0"></a>

## [Rust loops using loop](<#dest-32-0>)

- The `loop` keyword creates an infinite loop until a `break` is encountered

```
fn main() {
    let mut x = 40;
    // Change the below to 'here: loop to specify optional label for the loop
    loop {
        if x == 42 {
            break; // Use break x; to return the value of x
        }
        x += 1;
    }
}
```

- The `break` statement can include an optional expression that can be used to assign the value of a `loop` expression
- The `continue` keyword can be used to return to the top of the `loop`
- Loop labels can be used with `break` or `continue` and are useful when dealing with nested loops

<a id="dest-32-3465"></a>

## [Rust expression blocks](<#dest-32-3465>)

- Rust expression blocks are simply a sequence of expressions enclosed in `{}` . The evaluated value is simply the last expression in the block

```
fn main() {
    let x = {
        let y = 40;
        y + 2 // Note: ; must be omitted
    };
    // Notice the Python style printing
    println!("{x}");
}
```

- Rust style is to use this to omit the `return` keyword in functions

```
fn is_secret_of_life(x: u32) -> bool {
    // Same as if x == 42 {true} else {false}
    x == 42 // Note: ; must be omitted
}
fn main() {
    println!("{}", is_secret_of_life(42));
}
```

<a id="page-33"></a>

5. [5. Data Structures](<#page-33>)

<a id="dest-33-832"></a>

### [Rust array type](<#dest-33-832>)

What you’ll learn: Rust’s core data structures — arrays, tuples, slices, strings, structs,

`Vec` , and `HashMap` . This is a dense chapter; focus on understanding `String` vs `&str` and

how structs work. You’ll revisit references and borrowing in depth in chapter 7.

- Arrays contain a fixed number of elements of the same type
    - Like all other Rust types, arrays are immutable by default (unless mut is used)
    - Arrays are indexed using \[\] and are bounds checked. The len() method can be used to obtain the length of the array

```
    fn get_index(y : usize) -> usize {
        y+1
    }
    fn main() {
        // Initializes an array of 3 elements and sets all to 42
        let a : [u8; 3] = [42; 3];
        // Alternative syntax
        // let a = [42u8, 42u8, 42u8];
        for x in a {
            println!("{x}");
        }
        let y = get_index(a.len());
        // Commenting out the below will cause a panic
        //println!("{}", a[y]);
    }
```

<a id="dest-33-5715"></a>

### [Rust array type continued](<#dest-33-5715>)

- Arrays can be nested
    - Rust has several built-in formatters for printing. In the below, the `:?` is the `debug` print formatter. The `:#?` formatter can be used for `pretty print` . These formatters can be customized per type (more on this later)

<a id="page-34"></a>

```
    fn main() {
        let a = [
            [40, 0], // Define a nested array
            [41, 0],
            [42, 1],
        ];
        for x in a {
            println!("{x:?}");
        }
    }
```

<a id="dest-34-1980"></a>

### [Rust tuples](<#dest-34-1980>)

- Tuples have a fixed size and can group arbitrary types into a single compound type
    - The constituent types can be indexed by their relative location (.0, .1, .2, …). An empty tuple, i.e., () is called the unit value and is the equivalent of a void return value
    - Rust supports tuple destructuring to make it easy to bind variables to individual elements

```
fn get_tuple() -> (u32, bool) {
    (42, true)
}
fn main() {
   let t : (u8, bool) = (42, true);
   let u : (u32, bool) = (43, false);
   println!("{}, {}", t.0, t.1);
   println!("{}, {}", u.0, u.1);
   let (num, flag) = get_tuple(); // Tuple destructuring
   println!("{num}, {flag}");
}
```

<a id="dest-34-5422"></a>

### [Rust references](<#dest-34-5422>)

- References in Rust are roughly equivalent to pointers in C with some key differences
    - It is legal to have any number of read-only (immutable) references to a variable at any point of time. A reference cannot outlive the variable scope (this is a key concept called lifetime; discussed in detail later)
    - Only a single writable (mutable) reference to a mutable variable is permitted and it must not overlap with any other reference.

<a id="page-35"></a>

```
fn main() {
    let mut a = 42;
    {
        let b = &a;
        let c = b;
        println!("{} {}", *b, *c); // The compiler automatically dereferences *c
        let d = &mut a;
        /*
         * Uncommenting the line below would cause the
         * program to not compile, because `b` is used
         * while the mutable reference `d` is live in the current scope
         *
         * You cannot have a mutable and immutable reference in use in the same
scope
         * at the same time!
         */
        // println!("{}", *b);
    }
    let d = &mut a; // Ok: b and c are not in scope
    *d = 43;
}
```

<a id="dest-35-3548"></a>

## [Rust slices](<#dest-35-3548>)

- Rust references can be used to create subsets of arrays
    - Unlike arrays, which have a static fixed length determined at compile time, slices can be of arbitrary size. Internally, slices are implemented as a “fat-pointer” that contains the length of the slice and a pointer to the starting element in the original array

```
fn main() {
    let a = [40, 41, 42, 43];
    let b = &a[1..a.len()]; // A slice starting with the second element in the
original
    let c = &a[1..]; // Same as the above
    let d = &a[..]; // Same as &a[0..] or &a[0..a.len()]
    println!("{b:?} {c:?} {d:?}");
}
```

<a id="dest-35-6375"></a>

## [Rust constants and statics](<#dest-35-6375>)

- The `const` keyword can be used to define a constant value. Constant values are evaluated at compile time and are inlined into the program
- <a id="page-36"></a> The `static` keyword is used to define the equivalent of global variables in languages like C/C++ Static variables have an addressable memory location and are created once and last the entire lifetime of the program

```
const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_VARIABLE : u32 = 2;
fn main() {
    println!("The secret of life is {}", SECRET_OF_LIFE);
    println!("Value of global variable is {GLOBAL_VARIABLE}")
}
```

<a id="dest-36-1905"></a>

## [Rust strings: String vs &amp;str](<#dest-36-1905>)

- Rust has two string types that serve different purposes
    - `String` — owned, heap-allocated, growable (like C’s `malloc` ’d buffer, or C++’s `std::string` )
    - `&str` — borrowed, lightweight reference (like C’s `const char*` with length, or C++’s `std::string_view` — but `&str` is lifetime-checked so it can never dangle)
    - Unlike C’s null-terminated strings, Rust strings track their length and are guaranteed valid UTF-8

For C++ developers: `String` ≈ `std::string` , `&str` ≈ `std::string_view` . Unlike `std::string_view` , a `&str` is guaranteed valid for its entire lifetime by the borrow checker.

<a id="dest-36-5025"></a>

### [String vs &amp;str: Owned vs Borrowed](<#dest-36-5025>)

Production patterns: See [JSON handling: nlohmann::json → serde](<#dest-238-1942>) for how string handling works with serde in production code.

![Source region, PDF page 36](Rust-for-C_CPP-Programmers-images/page-0036-region-0020.png)

> PDF page 36: Complex table retained as a source image.

<a id="page-37"></a>

![Source region, PDF page 37](Rust-for-C_CPP-Programmers-images/page-0037-region-0021.png)

> PDF page 37: Complex table retained as a source image.

```
fn main() {
    // &str - string slice (borrowed, immutable, usually a string literal)
    let greeting: &str = "Hello";  // Points to read-only memory
    // String - owned, heap-allocated, growable
    let mut owned = String::from(greeting);  // Copies data to heap
    owned.push_str(", World!");        // Grow the string
    owned.push('!');                   // Append a single character
    // Converting between String and &str
    let slice: &str = &owned;          // String -> &str (free, just a borrow)
    let owned2: String = slice.to_string();  // &str -> String (allocates)
    let owned3: String = String::from(slice); // Same as above
    // String concatenation (note: + consumes the left operand)
    let hello = String::from("Hello");
    let world = String::from(", World!");
    let combined = hello + &world;  // hello is moved (consumed), world is
borrowed
    // println!("{hello}");  // Won't compile: hello was moved
    // Use format! to avoid move issues
    let a = String::from("Hello");
    let b = String::from("World");
    let combined = format!("{a}, {b}!");  // Neither a nor b is consumed
    println!("{combined}");
}
```

<a id="page-38"></a>

<a id="dest-38-0"></a>

## [Why You Cannot Index Strings with \[\]](<#dest-38-0>)

```
fn main() {
    let s = String::from("hello");
    // let c = s[0];  // Won't compile! Rust strings are UTF-8, not byte arrays
    // Safe alternatives:
    let first_char = s.chars().next();           // Option<char>: Some('h')
    let as_bytes = s.as_bytes();                 // &[u8]: raw UTF-8 bytes
    let substring = &s[0..1];                    // &str: "h" (byte range, must be
valid UTF-8 boundary)
    println!("First char: {:?}", first_char);
    println!("Bytes: {:?}", &as_bytes[..5]);
}
```

<a id="dest-38-2745"></a>

## [Exercise: String manipulation](<#dest-38-2745>)

#### 🟢 **Starter**

    - Write a function `fn count_words(text: &str) -> usize` that counts the number of whitespace-separated words in a string
    - Write a function `fn longest_word(text: &str) -> &str` that returns the longest word (hint: you’ll need to think about lifetimes – why does the return type need to be `&str` and not `String` ?)
- [ ] Solution (click to expand) [Rust structs](<#page-38>)
    - The `struct` keyword declares a user-defined struct type
        - `struct` members can either be named, or anonymous (tuple structs)
    - Unlike languages like C++, there’s no notion of “data inheritance” in Rust

<a id="page-39"></a>

```
fn main() {
    struct MyStruct {
        num: u32,
        is_secret_of_life: bool,
    }
    let x = MyStruct {
        num: 42,
        is_secret_of_life: true,
    };
    let y = MyStruct {
        num: x.num,
        is_secret_of_life: x.is_secret_of_life,
    };
    let z = MyStruct { num: x.num, ..x }; // The .. means copy remaining
    println!("{} {} {}", x.num, y.is_secret_of_life, z.num);
}
```

<a id="dest-39-2468"></a>

## [Rust tuple structs](<#dest-39-2468>)

- Rust tuple structs are similar to tuples and individual fields don’t have names
    - Like tuples, individual elements are accessed using .0, .1, .2, …. A common use case for tuple structs is to wrap primitive types to create custom types. This can be useful to avoid mixing differing values of the same type

```
struct WeightInGrams(u32);
struct WeightInMilligrams(u32);
fn to_weight_in_grams(kilograms: u32) -> WeightInGrams {
    WeightInGrams(kilograms * 1000)
}
fn to_weight_in_milligrams(w : WeightInGrams) -> WeightInMilligrams  {
    WeightInMilligrams(w.0 * 1000)
}
fn main() {
    let x = to_weight_in_grams(42);
    let y = to_weight_in_milligrams(x);
    // let z : WeightInGrams = x;  // Won't compile: x was moved into
to_weight_in_milligrams()
    // let a : WeightInGrams = y;   // Won't compile: type mismatch
(WeightInMilligrams vs WeightInGrams)
}
```

Note: The `#[derive(...)]` attribute automatically generates common trait implementations for structs and enums. You’ll see this used throughout the course:

<a id="page-40"></a>

```
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }
fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);           // Debug: works because of #[derive(Debug)]
    let p2 = p.clone();           // Clone: works because of #[derive(Clone)]
    assert_eq!(p, p2);            // PartialEq: works because of #
[derive(PartialEq)]
}
```

We’ll cover the trait system in depth later, but `#[derive(Debug)]` is so useful that you should add it to nearly every `struct` and `enum` you create.

<a id="dest-40-2138"></a>

## [Rust Vec type](<#dest-40-2138>)

- The `Vec<T>` type implements a dynamic heap allocated buffer (similar to manually managed `malloc` / `realloc` arrays in C, or C++’s `std::vector` )
    - Unlike arrays with fixed size, `Vec` can grow and shrink at runtime
    - `Vec` owns its data and automatically manages memory allocation/deallocation
- Common operations: `push()` , `pop()` , `insert()` , `remove()` , `len()` , `capacity()`

<a id="page-41"></a>

```
fn main() {
    let mut v = Vec::new();    // Empty vector, type inferred from usage
    v.push(42);                // Add element to end - Vec<i32>
    v.push(43);
    // Safe iteration (preferred)
    for x in &v {              // Borrow elements, don't consume vector
        println!("{x}");
    }
    // Initialization shortcuts
    let mut v2 = vec![1, 2, 3, 4, 5];           // Macro for initialization
    let v3 = vec![0; 10];                       // 10 zeros
    // Safe access methods (preferred over indexing)
    match v2.get(0) {
        Some(first) => println!("First: {first}"),
        None => println!("Empty vector"),
    }
    // Useful methods
    println!("Length: {}, Capacity: {}", v2.len(), v2.capacity());
    if let Some(last) = v2.pop() {             // Remove and return last element
        println!("Popped: {last}");
    }
    // Dangerous: direct indexing (can panic!)
    // println!("{}", v2[100]);  // Would panic at runtime
}
```

Production patterns: See [Avoiding unchecked indexing](<#dest-233-930>) for safe `.get()` patterns from production Rust code.

<a id="dest-41-4980"></a>

## [Rust HashMap type](<#dest-41-4980>)

- `HashMap` implements generic `key` -&gt; `value` lookups (a.k.a. `dictionary` or `map` )

<a id="page-42"></a>

```
fn main() {
    use std::collections::HashMap;  // Need explicit import, unlike Vec
    let mut map = HashMap::new();       // Allocate an empty HashMap
    map.insert(40, false);  // Type is inferred as int -> bool
    map.insert(41, false);
    map.insert(42, true);
    for (key, value) in map {
        println!("{key} {value}");
    }
    let map = HashMap::from([(40, false), (41, false), (42, true)]);
    if let Some(x) = map.get(&43) {
        println!("43 was mapped to {x:?}");
    } else {
        println!("No mapping was found for 43");
    }
    let x = map.get(&43).or(Some(&false));  // Default value if key isn't found
    println!("{x:?}");
}
```

<a id="dest-42-2738"></a>

## [Exercise: Vec and HashMap](<#dest-42-2738>)

#### 🟢 **Starter**

    - Create a `HashMap<u32, bool>` with a few entries (make sure that some values are `true` and others are `false` ). Loop over all elements in the hashmap and put the keys into one `Vec` and the values into another
- [ ] Solution (click to expand)

<a id="dest-42-4868"></a>

## [Deep Dive: C++ References vs Rust References](<#dest-42-4868>)

For C++ developers: C++ programmers often assume Rust `&T` works like C++ `T&` . While superficially similar, there are fundamental differences that cause confusion. C developers can skip this section — Rust references are covered in [Ownership and Borrowing](<#dest-52-158>).

1. [1. No Rvalue References or Universal References](<#page-42>)

In C++, `&&` has two meanings depending on context:

<a id="page-43"></a>

```
// C++: && means different things:
int&& rref = 42;           // Rvalue reference — binds to temporaries
void process(Widget&& w);   // Rvalue reference — caller must std::move
// Universal (forwarding) reference — deduced template context:
template<typename T>
void forward(T&& arg) {     // NOT an rvalue ref! Deduced as T& or T&&
    inner(std::forward<T>(arg));  // Perfect forwarding
}
```

In Rust: none of this exists. `&&` is simply the logical AND operator.

```
// Rust: && is just boolean AND
let a = true && false; // false
// Rust has NO rvalue references, no universal references, no perfect forwarding.
// Instead:
//   - Move is the default for non-Copy types (no std::move needed)
//   - Generics + trait bounds replace universal references
//   - No temporary-binding distinction — values are values
fn process(w: Widget) { }      // Takes ownership (like C++ value param + implicit
move)
fn process_ref(w: &Widget) { } // Borrows immutably (like C++ const T&)
fn process_mut(w: &mut Widget) { } // Borrows mutably (like C++ T&, but exclusive)
```

![Source region, PDF page 43](Rust-for-C_CPP-Programmers-images/page-0043-region-0022.png)

> PDF page 43: Complex table retained as a source image.

2. [2. Moves Are Bitwise — No Move Constructors](<#page-43>)

In C++, moving is a _user-defined operation_ (move constructor / move assignment). In Rust, moving is always a **bitwise memcpy** of the value, and the source is invalidated:

<a id="page-44"></a>

```
// Rust move = memcpy the bytes, mark source as invalid
let s1 = String::from("hello");
let s2 = s1; // Bytes of s1 are copied to s2's stack slot
              // s1 is now invalid — compiler enforces this
```

`// println!("{s1}"); //` ❌ `Compile error: value used after move`

```
// C++ move = call the move constructor (user-defined!)
std::string s1 = "hello";
std::string s2 = std::move(s1); // Calls string's move ctor
// s1 is now a "valid but unspecified state" zombie
std::cout << s1; // Compiles! Prints... something (empty string, usually)
```

Consequences:

    - Rust has no Rule of Five (no copy ctor, move ctor, copy=, move=, destructor to define)
    - No moved-from “zombie” state — the compiler simply prevents access
    - No `noexcept` considerations for moves — bitwise copy can’t throw
3. [3. Auto-Deref: The Compiler Sees Through Indirection](<#page-44>)

Rust automatically dereferences through multiple layers of pointers/wrappers via the `Deref` trait. This has no C++ equivalent:

```
use std::sync::{Arc, Mutex};
// Nested wrapping: Arc<Mutex<Vec<String>>>
let data = Arc::new(Mutex::new(vec!["hello".to_string()]));
// In C++, you'd need explicit unlocking and manual dereferencing at each layer.
// In Rust, the compiler auto-derefs through Arc → Mutex → MutexGuard → Vec:
let guard = data.lock().unwrap(); // Arc auto-derefs to Mutex
let first: &str = &guard[0];      // MutexGuard→Vec (Deref), Vec[0] (Index),
                                   // &String→&str (Deref coercion)
println!("First: {first}");
// Method calls also auto-deref:
let boxed_string = Box::new(String::from("hello"));
println!("Length: {}", boxed_string.len());  // Box→String, then String::len()
// No need for (*boxed_string).len() or boxed_string->len()
```

**Deref coercion** also applies to function arguments — the compiler inserts dereferences to make types match:

<a id="page-45"></a>

```
fn greet(name: &str) {
    println!("Hello, {name}");
}
fn main() {
    let owned = String::from("Alice");
    let boxed = Box::new(String::from("Bob"));
    let arced = std::sync::Arc::new(String::from("Carol"));
    greet(&owned);  // &String → &str  (1 deref coercion)
    greet(&boxed);  // &Box<String> → &String → &str  (2 deref coercions)
    greet(&arced);  // &Arc<String> → &String → &str  (2 deref coercions)
    greet("Dave");  // &str already — no coercion needed
}
// In C++ you'd need .c_str() or explicit conversions for each case.
```

The Deref chain: When you call `x.method()` , Rust’s method resolution tries the receiver type

`T` , then `&T` , then `&mut T` . If no match, it dereferences via the `Deref` trait and repeats with the target type. This continues through multiple layers — which is why `Box<Vec<T>>` “just works”

like a `Vec<T>` . Deref coercion (for function arguments) is a separate but related mechanism that automatically converts `&Box<String>` to `&str` by chaining `Deref` impls.

4. [4. No Null References, No Optional References](<#page-45>)

```
// C++: references can't be null, but pointers can, and the distinction is blurry
Widget& ref = *ptr;  // If ptr is null → UB
Widget* opt = nullptr;  // "optional" reference via pointer
```

```
// Rust: references are ALWAYS valid — guaranteed by the borrow checker
// No way to create a null or dangling reference in safe code
let r: &i32 = &42; // Always valid
// "Optional reference" is explicit:
let opt: Option<&Widget> = None; // Clear intent, no null pointer
if let Some(w) = opt {
    w.do_something(); // Only reachable when present
}
```

5. [5. References Cannot Be Reseated](<#page-45>)

```
// C++: a reference is an alias — it can't be rebound
int a = 1, b = 2;
int& r = a;
r = b;  // This ASSIGNS b's value to a — it does NOT rebind r!
// a is now 2, r still refers to a
```

<a id="page-46"></a>

```
// Rust: let bindings can shadow, but references follow different rules
let a = 1;
let b = 2;
let r = &a;
let r = &b;  // ✅ But you can SHADOW r with a new binding
             // The old binding is gone, not reseated
// With mut:
let mut r = &a;
```

`// r = &b;   //` ❌ `Cannot assign to immutable variable`

`r = &b;      //` ✅ `r now points to b — this IS rebinding (not assignment through)`

Mental model: In C++, a reference is a permanent alias for one object. In Rust, a reference is a value (a pointer with lifetime guarantees) that follows normal variable binding rules — immutable by default, rebindable only if declared `mut` .

<a id="page-47"></a>

<a id="dest-47-158"></a>

## [Rust enum types](<#dest-47-158>)

What you’ll learn: Rust enums as discriminated unions (tagged unions done right),

`match` for exhaustive pattern matching, and how enums replace C++ class hierarchies and C tagged unions with compiler-enforced safety.

- Enum types are discriminated unions, i.e., they are a sum type of several possible different types with a tag that identifies the specific variant
    - For C developers: enums in Rust can carry data (tagged unions done right — the compiler tracks which variant is active)
    - For C++ developers: Rust enums are like `std::variant` but with exhaustive pattern matching, no `std::get` exceptions, and no `std::visit` boilerplate
    - The size of the `enum` is that of the largest possible type. The individual variants are not related to one another and can have completely different types
    - `enum` types are one of the most powerful features of the language — they replace entire class hierarchies in C++ (more on this in the Case Studies)

```
fn main() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let a = Numbers::Zero;
    let b = Numbers::SmallNumber(42);
    let c : Numbers = a; // Ok -- the type of a is Numbers
    let d : Numbers = b; // Ok -- the type of b is Numbers
}
```

<a id="dest-47-5528"></a>

## [Rust match statement](<#dest-47-5528>)

- The Rust `match` is the equivalent of the C “switch” on steroids
    - `match` can be used for pattern matching on simple data types, `struct` , `enum`
    - The `match` statement must be exhaustive, i.e., they must cover all possible cases for a given `type` . The `_` can be used a wildcard for the “all else” case
    - `match` can yield a value, but all arms ( `=>` ) must return a value of the same type

<a id="page-48"></a>

```
fn main() {
    let x = 42;
    // In this case, the _ covers all numbers except the ones explicitly listed
    let is_secret_of_life = match x {
        42 => true, // return type is boolean value
        _ => false, // return type boolean value
        // This won't compile because return type isn't boolean
        // _ => 0
    };
    println!("{is_secret_of_life}");
}
```

<a id="dest-48-1792"></a>

## [Rust match statement](<#dest-48-1792>)

- `match` supports ranges, boolean filters, and `if` guard statements

```
fn main() {
    let x = 42;
    match x {
        // Note that the =41 ensures the inclusive range
        0..=41 => println!("Less than the secret of life"),
        42 => println!("Secret of life"),
        _ => println!("More than the secret of life"),
    }
    let y = 100;
    match y {
        100 if x == 43 => println!("y is 100% not secret of life"),
        100 if x == 42 => println!("y is 100% secret of life"),
        _ => (),    // Do nothing
    }
}
```

<a id="dest-48-4912"></a>

## [Rust match statement](<#dest-48-4912>)

- `match` and `enums` are often combined together
    - The match statement can “bind” the contained value to a variable. Use `_` if the value is a don’t care
    - The `matches!` macro can be used to match to specific variant

<a id="page-49"></a>

```
fn main() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let b = Numbers::SmallNumber(42);
    match b {
        Numbers::Zero => println!("Zero"),
        Numbers::SmallNumber(value) => println!("Small number {value}"),
        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => println!("Some
BiggerNumber or EvenBiggerNumber"),
    }
    // Boolean test for specific variants
    if matches!(b, Numbers::Zero | Numbers::SmallNumber(_)) {
        println!("Matched Zero or small number");
    }
}
```

<a id="dest-49-3008"></a>

## [Rust match statement](<#dest-49-3008>)

- `match` can also perform matches using destructuring and slices

```
fn main() {
    struct Foo {
        x: (u32, bool),
        y: u32
    }
    let f = Foo {x: (42, true), y: 100};
    match f {
        // Capture the value of x into a variable called tuple
        Foo{y: 100, x : tuple} => println!("Matched x: {tuple:?}"),
        _ => ()
    }
    let a = [40, 41, 42];
    match a {
        // Last element of slice must be 42. @ is used to bind the match
        [rest @ .., 42] => println!("{rest:?}"),
        // First element of the slice must be 42. @ is used to bind the match
        [42, rest @ ..] => println!("{rest:?}"),
        _ => (),
    }
}
```

<a id="page-50"></a>

<a id="dest-50-0"></a>

## [Exercise: Implement add and subtract using](<#dest-50-0>) [match and enum](<#dest-50-0>)

#### 🟢 **Starter**

- Write a function that implements arithmetic operations on unsigned 64-bit numbers
- Step 1: Define an enum for operations:

```
enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}
```

- Step 2: Define a result enum:

```
enum CalcResult {
    Ok(u64),                    // Successful result
    Invalid(String),            // Error message for invalid operations
}
```

- Step 3: Implement `calculate(op: Operation) -> CalcResult`
    - For Add: return Ok(sum)
    - For Subtract: return Ok(difference) if first &gt;= second, otherwise Invalid(“Underflow”)
- Hint: Use pattern matching in your function:

```
match op {
    Operation::Add(a, b) => { /* your code */ },
    Operation::Subtract(a, b) => { /* your code */ },
}
```

- [ ] Solution (click to expand) [Rust associated methods](<#page-50>)
    - `impl` can define methods associated for types like `struct` , `enum` , etc
        - The methods may optionally take `self` as a parameter. `self` is conceptually similar to passing a pointer to the struct as the first parameter in C, or `this` in C++
        - The reference to `self` can be immutable (default: `&self` ), mutable ( `&mut self` ), or `self` (transferring ownership)
        - The `Self` keyword can be used a shortcut to imply the type

<a id="page-51"></a>

```
struct Point {x: u32, y: u32}
impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point {x, y}
    }
    fn increment_x(&mut self) {
        self.x += 1;
    }
}
fn main() {
    let mut p = Point::new(10, 20);
    p.increment_x();
}
```

<a id="dest-51-2062"></a>

## [Exercise: Point add and transform](<#dest-51-2062>)

🟡 **Intermediate** — requires understanding move vs borrow from method signatures

    - Implement the following associated methods for `Point`
        - `add()` will take another `Point` and will increment the x and y values in place (hint: use `&mut self` )
        - `transform()` will consume an existing `Point` (hint: use `self` ) and return a new `Point` by squaring the x and y
- [ ] Solution (click to expand)

<a id="page-52"></a>

<a id="dest-52-158"></a>

## [Rust memory management](<#dest-52-158>)

What you’ll learn: Rust’s ownership system — the single most important concept in the language. After this chapter you’ll understand move semantics, borrowing rules, and the `Drop` trait. If you grasp this chapter, the rest of Rust follows naturally. If you’re struggling, re-read it — ownership clicks on the second pass for most C/C++ developers.

- Memory management in C/C++ is a source of bugs:
    - In C: memory is allocated with `malloc()` and freed with `free()` . No checks against dangling pointers, use-after-free, or double-free
    - In C++: RAII (Resource Acquisition Is Initialization) and smart pointers help, but `std::move(ptr)` compiles even after the move — use-after-move is UB
- Rust makes RAII foolproof:
    - Move is destructive — the compiler refuses to let you touch the moved-from variable
    - No Rule of Five needed (no copy ctor, move ctor, copy assign, move assign, destructor)
    - Rust gives complete control of memory allocation, but enforces safety at compile time
    - This is done by a combination of mechanisms including ownership, borrowing, mutability and lifetimes
    - Rust runtime allocations can happen both on the stack and the heap

#### **For C++ developers — Smart Pointer Mapping:**

![Source region, PDF page 52](Rust-for-C_CPP-Programmers-images/page-0052-region-0023.png)

> PDF page 52: Complex table retained as a source image.

<a id="page-53"></a>

For C developers: `Box<T>` replaces `malloc` / `free` pairs. `Rc<T>` replaces manual reference counting. Raw pointers exist but are confined to `unsafe` blocks.

<a id="dest-53-645"></a>

## [Rust ownership, borrowing and lifetimes](<#dest-53-645>)

- Recall that Rust only permits a single mutable reference to a variable and multiple read- only references
    - The initial declaration of the variable establishes `ownership`
    - Subsequent references `borrow` from the original owner. The rule is that the scope of the borrow can never exceed the owning scope. In other words, the `lifetime` of a borrow cannot exceed the owning lifetime

```
fn main() {
    let a = 42; // Owner
    let b = &a; // First borrow
    {
        let aa = 42;
        let c = &a; // Second borrow; a is still in scope
        // Ok: c goes out of scope here
        // aa goes out of scope here
    }
    // let d = &aa; // Will not compile unless aa is moved to outside scope
    // b implicitly goes out of scope before a
    // a goes out of scope last
}
```

- Rust can pass parameters to methods using several different mechanisms
    - By value (copy): Typically types that can be trivially copied (ex: u8, u32, i8, i32)
    - By reference: This is the equivalent of passing a pointer to the actual value. This is also commonly known as borrowing, and the reference can be immutable ( `&` ), or mutable ( `&mut` )
    - By moving: This transfers “ownership” of the value to the function. The caller can no longer reference the original value

```
fn foo(x: &u32) {
    println!("{x}");
}
fn bar(x: u32) {
    println!("{x}");
}
fn main() {
    let a = 42;
    foo(&a);    // By reference
    bar(a);     // By value (copy)
}
```

<a id="page-54"></a>

- Rust prohibits dangling references from methods
    - References returned by methods must still be in scope
    - Rust will automatically `drop` a reference when it goes out of scope.

```
fn no_dangling() -> &u32 {
    // lifetime of a begins here
    let a = 42;
    // Won't compile. lifetime of a ends here
    &a
}
fn ok_reference(a: &u32) -> &u32 {
    // Ok because the lifetime of a always exceeds ok_reference()
    a
}
fn main() {
    let a = 42;     // lifetime of a begins here
    let b = ok_reference(&a);
    // lifetime of b ends here
    // lifetime of a ends here
}
```

<a id="dest-54-3255"></a>

## [Rust move semantics](<#dest-54-3255>)

- By default, Rust assignment transfers ownership

```
fn main() {
    let s = String::from("Rust");    // Allocate a string from the heap
    let s1 = s; // Transfer ownership to s1. s is invalid at this point
    println!("{s1}");
    // This will not compile
    //println!("{s}");
    // s1 goes out of scope here and the memory is deallocated
    // s goes out of scope here, but nothing happens because it doesn't own
anything
}
```

<a id="page-55"></a>

![Source region, PDF page 55](Rust-for-C_CPP-Programmers-images/page-0055-region-0024.png)

> PDF page 55: Ambiguous layout; inspect the intact source page: multiple mathematical regions among graphics.

<a id="page-56"></a>

<a id="dest-56-0"></a>

## [Rust move semantics and borrowing](<#dest-56-0>)

```
fn foo(s : String) {
    println!("{s}");
    // The heap memory pointed to by s will be deallocated here
}
fn bar(s : &String) {
    println!("{s}");
    // Nothing happens -- s is borrowed
}
fn main() {
    let s = String::from("Rust string move example");    // Allocate a string from
the heap
    foo(s); // Transfers ownership; s is invalid now
    // println!("{s}");  // will not compile
    let t = String::from("Rust string borrow example");
    bar(&t);    // t continues to hold ownership
    println!("{t}");
}
```

<a id="dest-56-3082"></a>

## [Rust move semantics and ownership](<#dest-56-3082>)

- It is possible to transfer ownership by moving
    - It is illegal to reference outstanding references after the move is completed
    - Consider borrowing if a move is not desirable

```
struct Point {
    x: u32,
    y: u32,
}
fn consume_point(p: Point) {
    println!("{} {}", p.x, p.y);
}
fn borrow_point(p: &Point) {
    println!("{} {}", p.x, p.y);
}
fn main() {
    let p = Point {x: 10, y: 20};
    // Try flipping the two lines
    borrow_point(&p);
    consume_point(p);
}
```

<a id="page-57"></a>

<a id="dest-57-0"></a>

## [Rust Clone](<#dest-57-0>)

- The `clone()` method can be used to copy the original memory. The original reference continues to be valid (the downside is that we have 2x the allocation)

```
fn main() {
    let s = String::from("Rust");    // Allocate a string from the heap
    let s1 = s.clone(); // Copy the string; creates a new allocation on the heap
    println!("{s1}");
    println!("{s}");
    // s1 goes out of scope here and the memory is deallocated
    // s goes out of scope here, and the memory is deallocated
}
```

![Source region, PDF page 57](Rust-for-C_CPP-Programmers-images/page-0057-region-0025.png)

> PDF page 57: Mathematics retained as a source image.

s (stack)

s1 (stack)

ptr

ptr

owns owns (copy)

Heap: R u s t

Heap: R u s t

`clone()` creates a separate heap allocation. Both `s` and `s1` are valid — each owns its own copy.

<a id="dest-57-5190"></a>

## [Rust Copy trait](<#dest-57-5190>)

- Rust implements copy semantics for built-in types using the `Copy` trait
    - Examples include u8, u32, i8, i32, etc. Copy semantics use “pass by value”
    - User defined data types can optionally opt into `copy` semantics using the `derive` macro with to automatically implement the `Copy` trait
    - The compiler will allocate space for the copy following a new assignment

<a id="page-58"></a>

```
// Try commenting this out to see the change in let p1 = p; below
#[derive(Copy, Clone, Debug)]   // We'll discuss this more later
struct Point{x: u32, y:u32}
fn main() {
    let p = Point {x: 42, y: 40};
    let p1 = p;     // This will perform a copy now instead of move
    println!("p: {p:?}");
    println!("p1: {p:?}");
    let p2 = p1.clone();    // Semantically the same as copy
}
```

<a id="dest-58-1658"></a>

## [Rust Drop trait](<#dest-58-1658>)

- Rust automatically calls the `drop()` method at the end of scope
    - `drop` is part of a generic trait called `Drop` . The compiler provides a blanket NOP implementation for all types, but types can override it. For example, the `String` type overrides it to release heap-allocated memory
    - For C developers: this replaces the need for manual `free()` calls — resources are automatically released when they go out of scope (RAII)
- Key safety: You cannot call `.drop()` directly (the compiler forbids it). Instead, use `drop(obj)` which moves the value into the function, runs its destructor, and prevents any further use — eliminating double-free bugs

For C++ developers: `Drop` maps directly to C++ destructors ( `~ClassName()` ):

C++ destructor Rust Drop

```
impl Drop for MyType {
fn drop(&mut self) {
... } }
```

Syntax `~MyClass() { ... }`

When End of scope (RAII) End of scope (same) called

Source left in “valid but unspecified” state — destructor still runs on the moved-from object

Source is gone — no destructor call on moved- from value

Called on move

`drop(obj)` (safe — takes ownership, calls `drop` , prevents further use)

Manual call

`obj.~MyClass()` (dangerous, rarely used)

Reverse declaration order Order Reverse declaration order (same)

<a id="page-59"></a>

![Source region, PDF page 59](Rust-for-C_CPP-Programmers-images/page-0059-region-0026.png)

> PDF page 59: Complex table retained as a source image.

```
struct Point {x: u32, y:u32}
// Equivalent to: ~Point() { printf("Goodbye point x:%u, y:%u\n", x, y); }
impl Drop for Point {
    fn drop(&mut self) {
        println!("Goodbye point x:{}, y:{}", self.x, self.y);
    }
}
fn main() {
    let p = Point{x: 42, y: 42};
    {
        let p1 = Point{x:43, y: 43};
        println!("Exiting inner block");
        // p1.drop() called here — like C++ end-of-scope destructor
    }
    println!("Exiting main");
    // p.drop() called here
}
```

<a id="dest-59-4215"></a>

## [Exercise: Move, Copy and Drop](<#dest-59-4215>)

🟡 **Intermediate** — experiment freely; the compiler will guide you

- Create your own experiments with `Point` with and without `Copy` in `#[derive(Debug)]` in the below make sure you understand the differences. The idea is to get a solid understanding of how move vs. copy works, so make sure to ask
- Implement a custom `Drop` for `Point` that sets x and y to 0 in `drop` . This is a pattern that’s useful for releasing locks and other resources for example

```
struct Point{x: u32, y: u32}
fn main() {
    // Create Point, assign it to a different variable, create a new scope,
    // pass point to a function, etc.
}
```

- [ ] Solution (click to expand)

<a id="page-60"></a>

<a id="dest-60-158"></a>

## [Rust lifetime and borrowing](<#dest-60-158>)

**What you’ll learn:** How Rust’s lifetime system ensures references never dangle — from implicit lifetimes through explicit annotations to the three elision rules that make most code annotation-free. Understanding lifetimes here is essential before moving on to smart pointers in the next section.

- Rust enforces a single mutable reference and any number of immutable references
    - The lifetime of any reference must be at least as long as the original owning lifetime. These are implicit lifetimes and are inferred by the compiler (see https://doc.rust- lang.org/nomicon/lifetime-elision.html)

```
fn borrow_mut(x: &mut u32) {
    *x = 43;
}
fn main() {
    let mut x = 42;
    let y = &mut x;
    borrow_mut(y);
    let _z = &x; // Permitted because the compiler knows y isn't subsequently used
    //println!("{y}"); // Will not compile if this is uncommented
    borrow_mut(&mut x); // Permitted because _z isn't used
    let z = &x; // Ok -- mutable borrow of x ended after borrow_mut() returned
    println!("{z}");
}
```

<a id="dest-60-4620"></a>

## [Rust lifetime annotations](<#dest-60-4620>)

- Explicit lifetime annotations are needed when dealing with multiple lifetimes
    - Lifetimes are denoted with `'` and can be any identifier ( `'a` , `'b` , `'static` , etc.)
    - The compiler needs help when it can’t figure out how long references should live
- Common scenario: Function returns a reference, but which input does it come from?

<a id="page-61"></a>

```
#[derive(Debug)]
struct Point {x: u32, y: u32}
// Without lifetime annotation, this won't compile:
// fn left_or_right(pick_left: bool, left: &Point, right: &Point) -> &Point
// With lifetime annotation - all references share the same lifetime 'a
fn left_or_right<'a>(pick_left: bool, left: &'a Point, right: &'a Point) -> &'a
Point {
    if pick_left { left } else { right }
}
// More complex: different lifetimes for inputs
fn get_x_coordinate<'a, 'b>(p1: &'a Point, _p2: &'b Point) -> &'a u32 {
    &p1.x  // Return value lifetime tied to p1, not p2
}
fn main() {
    let p1 = Point {x: 20, y: 30};
    let result;
    {
        let p2 = Point {x: 42, y: 50};
        result = left_or_right(true, &p1, &p2);
        // This works because we use result before p2 goes out of scope
        println!("Selected: {result:?}");
    }
    // This would NOT work - result references p2 which is now gone:
    // println!("After scope: {result:?}");
}
```

<a id="dest-61-4222"></a>

## [Rust lifetime annotations](<#dest-61-4222>)

- Lifetime annotations are also needed for references in data structures

<a id="page-62"></a>

```
use std::collections::HashMap;
#[derive(Debug)]
struct Point {x: u32, y: u32}
struct Lookup<'a> {
    map: HashMap<u32, &'a Point>,
}
fn main() {
    let p = Point{x: 42, y: 42};
    let p1 = Point{x: 50, y: 60};
    let mut m = Lookup {map : HashMap::new()};
    m.map.insert(0, &p);
    m.map.insert(1, &p1);
    {
        let p3 = Point{x: 60, y:70};
        //m.map.insert(3, &p3); // Will not compile
        // p3 is dropped here, but m will outlive
    }
    for (k, v) in m.map {
        println!("{v:?}");
    }
    // m is dropped here
    // p1 and p are dropped here in that order
}
```

<a id="dest-62-3412"></a>

## [Exercise: First word with lifetimes](<#dest-62-3412>)

🟢 **Starter** — practice lifetime elision in action

Write a function `fn first_word(s: &str) -> &str` that returns the first whitespace-delimited word from a string. Think about why this compiles without explicit lifetime annotations (hint: elision rule \#1 and \#2).

- [ ] Solution (click to expand) [Exercise: Slice storage with lifetimes](<#page-62>)

🟡 **Intermediate** — your first encounter with lifetime annotations

- Create a structure that stores references to the slice of a `&str`
    - Create a long `&str` and store references slices from it inside the structure
    - Write a function that accepts the structure and returns the contained slice

<a id="page-63"></a>

```
// TODO: Create a structure to store a reference to a slice
struct SliceStore {
}
fn main() {
    let s = "This is long string";
    let s1 = &s[0..];
    let s2 = &s[1..2];
    // let slice = struct SliceStore {...};
    // let slice2 = struct SliceStore {...};
}
```

- [ ] Solution (click to expand)

<a id="dest-63-2445"></a>

## [Lifetime Elision Rules Deep Dive](<#dest-63-2445>)

C programmers often ask: “If lifetimes are so important, why don’t most Rust functions have

`'a` annotations?” The answer is lifetime elision — the compiler applies three deterministic rules to infer lifetimes automatically.

<a id="dest-63-3720"></a>

### [The Three Elision Rules](<#dest-63-3720>)

The Rust compiler applies these rules **in order** to function signatures. If all output lifetimes are determined after applying the rules, no annotations are needed.

<a id="page-64"></a>

![Source region, PDF page 64](Rust-for-C_CPP-Programmers-images/page-0064-region-0027.png)

![Source region, PDF page 64](Rust-for-C_CPP-Programmers-images/page-0064-region-0028.png)

![Source region, PDF page 64](Rust-for-C_CPP-Programmers-images/page-0064-region-0029.png)

![Source region, PDF page 64](Rust-for-C_CPP-Programmers-images/page-0064-region-0030.png)

![Source region, PDF page 64](Rust-for-C_CPP-Programmers-images/page-0064-region-0031.png)

<a id="page-65"></a>

![Source region, PDF page 65](Rust-for-C_CPP-Programmers-images/page-0065-region-0032.png)

<a id="dest-65-1290"></a>

### [Rule-by-Rule Examples](<#dest-65-1290>)

**Rule 1** — each input reference gets its own lifetime parameter:

```
// What you write:
fn first_word(s: &str) -> &str { ... }
// What the compiler sees after Rule 1:
fn first_word<'a>(s: &'a str) -> &str { ... }
// Only one input lifetime → Rule 2 applies
```

#### **Rule 2** — single input lifetime propagates to all outputs:

```
// After Rule 2:
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

`//` ✅ `All output lifetimes determined — no annotation needed!`

Rule 3 — `&self` lifetime propagates to outputs:

```
// What you write:
impl SliceStore<'_> {
    fn get_slice(&self) -> &str { self.slice }
}
// What the compiler sees after Rules 1 + 3:
impl SliceStore<'_> {
    fn get_slice<'a>(&'a self) -> &'a str { self.slice }
}
```

`//` ✅ `No annotation needed — &self lifetime used for output`

#### **When elision fails** — you must annotate:

```
// Two input references, no &self → Rules 2 and 3 don't apply
// fn longest(a: &str, b: &str) -> &str  ← WON'T COMPILE
// Fix: tell the compiler which input the output borrows from
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

<a id="page-66"></a>

<a id="dest-66-0"></a>

### [C Programmer Mental Model](<#dest-66-0>)

In C, every pointer is independent — the programmer mentally tracks which allocation each pointer refers to, and the compiler trusts you completely. In Rust, lifetimes make this tracking explicit and compiler-verified:

C Rust What happens

Rule 3 elides: output `fn get_name(&self) -> &str` borrows from `self` `User* u)`

```
char* get_name(struct
```

Must annotate — two inputs

```
char* concat(char* a,       fn concat<'a>(a: &'a str,
char* b)                   b: &'a str) -> &'a str
void process(char*          fn process(input: &str,
in, char* out)             output: &mut String)
```

No output reference — no lifetime needed

Compile error if lifetime is wrong

Compiler catches dangling pointers

`char* buf; /* who`

```
owns this? */
```

<a id="dest-66-3075"></a>

### [The 'static Lifetime](<#dest-66-3075>)

`'static` means the reference is valid for the entire program duration. It’s the Rust equivalent of a C global or string literal:

```
// String literals are always 'static — they live in the binary's read-only
section
let s: &'static str = "hello";  // Same as: static const char* s = "hello"; in C
// Constants are also 'static
static GREETING: &str = "hello";
// Common in trait bounds for thread spawning:
fn spawn<F: FnOnce() + Send + 'static>(f: F) { /* ... */ }
// 'static here means: "the closure must not borrow any local variables"
// (either move them in, or use only 'static data)
```

<a id="dest-66-5865"></a>

### [Exercise: Predict the Elision](<#dest-66-5865>)

#### 🟡 **Intermediate**

For each function signature below, predict whether the compiler can elide lifetimes. If not, add the necessary annotations:

<a id="page-67"></a>

```
// 1. Can the compiler elide?
fn trim_prefix(s: &str) -> &str { &s[1..] }
// 2. Can the compiler elide?
fn pick(flag: bool, a: &str, b: &str) -> &str {
    if flag { a } else { b }
}
// 3. Can the compiler elide?
struct Parser { data: String }
impl Parser {
    fn next_token(&self) -> &str { &self.data[..5] }
}
// 4. Can the compiler elide?
fn split_at(s: &str, pos: usize) -> (&str, &str) {
    (&s[..pos], &s[pos..])
}
```

- [ ] Solution (click to expand)

<a id="page-68"></a>

<a id="dest-68-158"></a>

## [Rust Box&lt;T&gt;](<#dest-68-158>)

What you’ll learn: Rust’s smart pointer types — `Box<T>` for heap allocation, `Rc<T>` for shared ownership, and `Cell<T>` / `RefCell<T>` for interior mutability. These build on the ownership and lifetime concepts from the previous sections. You’ll also see a brief introduction to `Weak<T>` for breaking reference cycles.

Why Box&lt;T&gt; ? In C, you use `malloc` / `free` for heap allocation. In C++, `std::unique_ptr<T>`

wraps `new` / `delete` . Rust’s `Box<T>` is the equivalent — a heap-allocated, single-owner pointer that is automatically freed when it goes out of scope. Unlike `malloc` , there’s no matching `free` to forget. Unlike `unique_ptr` , there’s no use-after-move — the compiler prevents it entirely.

#### **When to use** Box **vs stack allocation:**

- The contained type is large and you don’t want to copy it on the stack
- You need a recursive type (e.g., a linked list node that contains itself)
- You need trait objects ( `Box<dyn Trait>` )
- `Box<T>` can be use to create a pointer to a heap allocated type. The pointer is always a fixed size regardless of the type of `<T>`

```
fn main() {
    // Creates a pointer to an integer (with value 42) created on the heap
    let f = Box::new(42);
    println!("{} {}", *f, f);
    // Cloning a box creates a new heap allocation
    let mut g = f.clone();
    *g = 43;
    println!("{f} {g}");
    // g and f go out of scope here and are automatically deallocated
}
```

<a id="page-69"></a>

Stack Heap

f: Box&lt;i32&gt;

owns

- [ ] 42

g: Box&lt;i32&gt;

owns (cloned)

- [ ] 43

<a id="dest-69-2295"></a>

## [Ownership and Borrowing Visualization](<#dest-69-2295>)

<a id="dest-69-2895"></a>

### [C/C++ vs Rust: Pointer and Ownership Management](<#dest-69-2895>)

```
// C - Manual memory management, potential issues
void c_pointer_problems() {
    int* ptr1 = malloc(sizeof(int));
    *ptr1 = 42;
    int* ptr2 = ptr1;  // Both point to same memory
    int* ptr3 = ptr1;  // Three pointers to same memory
    free(ptr1);        // Frees the memory
    *ptr2 = 43;        // Use after free - undefined behavior!
    *ptr3 = 44;        // Use after free - undefined behavior!
}
```

**For C++ developers:** Smart pointers help, but don’t prevent all issues:

<a id="page-70"></a>

```
// C++ - Smart pointers help, but don't prevent all issues
void cpp_pointer_issues() {
    auto ptr1 = std::make_unique<int>(42);
    // auto ptr2 = ptr1;  // Compile error: unique_ptr not copyable
    auto ptr2 = std::move(ptr1);  // OK: ownership transferred
    // But C++ still allows use-after-move:
    // std::cout << *ptr1;  // Compiles! But undefined behavior!
    // shared_ptr aliasing:
    auto shared1 = std::make_shared<int>(42);
    auto shared2 = shared1;  // Both own the data
    // Who "really" owns it? Neither. Ref count overhead everywhere.
}
```

```
// Rust - Ownership system prevents these issues
fn rust_ownership_safety() {
    let data = Box::new(42);  // data owns the heap allocation
    let moved_data = data;    // Ownership transferred to moved_data
    // data is no longer accessible - compile error if used
    let borrowed = &moved_data;  // Immutable borrow
    println!("{}", borrowed);    // Safe to use
    // moved_data automatically freed when it goes out of scope
}
```

![Source region, PDF page 70](Rust-for-C_CPP-Programmers-images/page-0070-region-0033.png)

<a id="page-71"></a>

<a id="dest-71-0"></a>

### [Borrowing Rules Visualization](<#dest-71-0>)

```
fn borrowing_rules_example() {
    let mut data = vec![1, 2, 3, 4, 5];
    // Multiple immutable borrows - OK
    let ref1 = &data;
    let ref2 = &data;
    println!("{:?} {:?}", ref1, ref2);  // Both can be used
    // Mutable borrow - exclusive access
    let ref_mut = &mut data;
    ref_mut.push(6);
    // ref1 and ref2 can't be used while ref_mut is active
    // After ref_mut is done, immutable borrows work again
    let ref3 = &data;
    println!("{:?}", ref3);
}
```

![Source region, PDF page 71](Rust-for-C_CPP-Programmers-images/page-0071-region-0034.png)

<a id="dest-71-4935"></a>

## [Interior Mutability: Cell&lt;T&gt; and RefCell&lt;T&gt;](<#dest-71-4935>)

Recall that by default variables are immutable in Rust. Sometimes it’s desirable to have most of a type read-only while permitting write access to a single field.

```
struct Employee {
    employee_id : u64,   // This must be immutable
    on_vacation: bool,   // What if we wanted to permit write-access to this
field, but make employee_id immutable?
}
```

- Recall that Rust permits a single mutable reference to a variable and any number of immutable references — enforced at compile-time
- <a id="page-72"></a> What if we wanted to pass an immutable vector of employees, but allow the `on_vacation` field to be updated, while ensuring `employee_id` cannot be mutated?

<a id="dest-72-712"></a>

[Cell&lt;T&gt; — interior mutability for Copy types](<#dest-72-712>)

- `Cell<T>` provides interior mutability, i.e., write access to specific elements of references that are otherwise read-only
- Works by copying values in and out (requires `T: Copy` for `.get()` )

<a id="dest-72-1928"></a>

[RefCell&lt;T&gt; — interior mutability with runtime borrow checking](<#dest-72-1928>)

- `RefCell<T>` provides a variation that works with references
    - Enforces Rust borrow-checks at runtime instead of compile-time
    - Allows a single mutable borrow, but panics if there are any other references outstanding
    - Use `.borrow()` for immutable access and `.borrow_mut()` for mutable access

<a id="dest-72-3488"></a>

[When to Choose Cell vs RefCell](<#dest-72-3488>)

![Source region, PDF page 72](Rust-for-C_CPP-Programmers-images/page-0072-region-0035.png)

> PDF page 72: Complex table retained as a source image.

<a id="page-73"></a>

<a id="dest-73-0"></a>

## [Shared Ownership: Rc&lt;T&gt;](<#dest-73-0>)

`Rc<T>` allows reference-counted shared ownership of immutable data. What if we wanted to store the same `Employee` in multiple places without copying?

```
#[derive(Debug)]
struct Employee {
    employee_id: u64,
}
fn main() {
    let mut us_employees = vec![];
    let mut all_global_employees = Vec::<Employee>::new();
    let employee = Employee { employee_id: 42 };
    us_employees.push(employee);
    // Won't compile — employee was already moved
    //all_global_employees.push(employee);
}
```

`Rc<T>` solves the problem by allowing shared immutable access:

- The contained type is automatically dereferenced
- The type is dropped when the reference count goes to 0

```
use std::rc::Rc;
#[derive(Debug)]
struct Employee {employee_id: u64}
fn main() {
    let mut us_employees = vec![];
    let mut all_global_employees = vec![];
    let employee = Employee { employee_id: 42 };
    let employee_rc = Rc::new(employee);
    us_employees.push(employee_rc.clone());
    all_global_employees.push(employee_rc.clone());
    let employee_one = all_global_employees.get(0); // Shared immutable reference
    for e in us_employees {
        println!("{}", e.employee_id);  // Shared immutable reference
    }
    println!("{employee_one:?}");
}
```

#### **For C++ developers: Smart Pointer Mapping**

C++ Smart Pointer Rust Equivalent Key Difference

Rust’s version is the default — move is language-level, not opt- in

```
std::unique_ptr<T>      Box<T>
```

<a id="page-74"></a>

C++ Smart Pointer Rust Equivalent Key Difference

No atomic overhead for

`Rc<T>` (single-thread) / `std::shared_ptr<T>`

`Rc` ; use `Arc` only when sharing across threads

`Arc<T>` (multi-thread)

`Weak<T>` (from

Same purpose: break reference cycles

`Rc::downgrade()` or

```
std::weak_ptr<T>
```

`Arc::downgrade()` )

Key distinction: In C++, you choose to use smart pointers. In Rust, owned values ( `T` ) and borrowing ( `&T` ) cover most use cases — reach for `Box` / `Rc` / `Arc` only when you need heap allocation or shared ownership.

<a id="dest-74-2452"></a>

### [Breaking Reference Cycles with Weak&lt;T&gt;](<#dest-74-2452>)

`Rc<T>` uses reference counting — if two `Rc` values point to each other, neither will ever be dropped (a cycle). `Weak<T>` solves this:

```
use std::rc::{Rc, Weak};
struct Node {
    value: i32,
    parent: Option<Weak<Node>>,  // Weak reference — doesn't prevent drop
}
fn main() {
    let parent = Rc::new(Node { value: 1, parent: None });
    let child = Rc::new(Node {
        value: 2,
        parent: Some(Rc::downgrade(&parent)),  // Weak ref to parent
    });
    // To use a Weak, try to upgrade it — returns Option<Rc<T>>
    if let Some(parent_rc) = child.parent.as_ref().unwrap().upgrade() {
        println!("Parent value: {}", parent_rc.value);
    }
    println!("Parent strong count: {}", Rc::strong_count(&parent)); // 1, not 2
}
```

`Weak<T>` is covered in more depth in [Avoiding Excessive clone()](<#dest-218-158>). For now, the key takeaway: use Weak for “back-references” in tree/graph structures to avoid memory leaks.

<a id="page-75"></a>

<a id="dest-75-0"></a>

### [Combining Rc with Interior Mutability](<#dest-75-0>)

The real power emerges when you combine `Rc<T>` (shared ownership) with `Cell<T>` or `RefCell<T>` (interior mutability). This lets multiple owners read and modify shared data:

![Source region, PDF page 75](Rust-for-C_CPP-Programmers-images/page-0075-region-0036.png)

> PDF page 75: Complex table retained as a source image.

<a id="dest-75-1988"></a>

## [Exercise: Shared ownership and interior](<#dest-75-1988>) [mutability](<#dest-75-1988>)

#### 🟡 **Intermediate**

- Part 1 (Rc): Create an `Employee` struct with `employee_id: u64` and `name: String` . Place it in an `Rc<Employee>` and clone it into two separate `Vec` s ( `us_employees` and `global_employees` ). Print from both vectors to show they share the same data.
- Part 2 (Cell): Add an `on_vacation: Cell<bool>` field to `Employee` . Pass an immutable `&Employee` reference to a function and toggle `on_vacation` from inside that function — without making the reference mutable.
- Part 3 (RefCell): Replace `name: String` with `name: RefCell<String>` and write a function that appends a suffix to the employee’s name through an `&Employee` (immutable reference).

#### **Starter code:**

<a id="page-76"></a>

```
use std::cell::{Cell, RefCell};
use std::rc::Rc;
#[derive(Debug)]
struct Employee {
    employee_id: u64,
    name: RefCell<String>,
    on_vacation: Cell<bool>,
}
fn toggle_vacation(emp: &Employee) {
    // TODO: Flip on_vacation using Cell::set()
}
fn append_title(emp: &Employee, title: &str) {
    // TODO: Borrow name mutably via RefCell and push_str the title
}
fn main() {
    // TODO: Create an employee, wrap in Rc, clone into two Vecs,
    // call toggle_vacation and append_title, print results
}
```

- [ ] Solution (click to expand)

<a id="page-77"></a>

<a id="dest-77-158"></a>

## [Rust crates and modules](<#dest-77-158>)

What you’ll learn: How Rust organizes code into modules and crates — privacy-by- default visibility, `pub` modifiers, workspaces, and the `crates.io` ecosystem. Replaces C/C++ header files, `#include` , and CMake dependency management.

- Modules are the fundamental organizational unit of code within crates
    - Each source file (.rs) is its own module, and can create nested modules using the `mod` keyword.
    - All types in a (sub-) module are private by default, and aren’t externally visible within the same crate unless they are explicitly marked as `pub` (public). The scope of `pub` can be further restricted to `pub(crate)` , etc
    - Even if a type is public, it doesn’t automatically become visible within the scope of another module unless it’s imported using the `use` keyword. Child submodules can reference types in the parent scope using the `use super::`
    - Source files (.rs) aren’t automatically included in the crate unless they are explicitly listed in `main.rs` (executable) or `lib.rs`

<a id="dest-77-3705"></a>

## [Exercise: Modules and functions](<#dest-77-3705>)

- We’ll take a look at modifying our [hello world](<https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=522d86dbb8c4af71ff2ec081fb76aee7>) to call another function
    - As previously mentioned, function are defined with the `fn` keyword. The `->` keyword declares that the function returns a value (the default is void) with the type `u32` (unsigned 32-bit integer)
    - Functions are scoped by module, i.e., two functions with exact same name in two modules won’t have a name collision
        - The module scoping extends to all types (for example, a `struct foo` in `mod a` `{ struct foo; }` is a distinct type ( `a::foo` ) from `mod b { struct foo; }` ( `b::foo` ))

**Starter code** — complete the functions:

<a id="page-78"></a>

```
mod math {
    // TODO: implement pub fn add(a: u32, b: u32) -> u32
}
fn greet(name: &str) -> String {
    // TODO: return "Hello, <name>! The secret number is <math::add(21,21)>"
    todo!()
}
fn main() {
    println!("{}", greet("Rustacean"));
}
```

- [ ] Solution (click to expand)

<a id="dest-78-2505"></a>

## [Workspaces and crates (packages)](<#dest-78-2505>)

- Any significant Rust project should use workspaces to organize component crates
    - A workspace is simply a collection of local crates that will be used to build the target binaries. The `Cargo.toml` at the workspace root should have a pointer to the constituent packages (crates)

```
[workspace]
resolver = "2"
members = ["package1", "package2"]
```

```
workspace_root/
|-- Cargo.toml      # Workspace configuration
|-- package1/
|   |-- Cargo.toml  # Package 1 configuration
|   `-- src/
|       `-- lib.rs  # Package 1 source code
|-- package2/
|   |-- Cargo.toml  # Package 2 configuration
|   `-- src/
|       `-- main.rs # Package 2 source code
```

<a id="dest-78-6480"></a>

## [Exercise: Using workspaces and package dependencies](<#dest-78-6480>)

- We’ll create a simple package and use it from our `hello world` program\`
- Create the workspace directory

<a id="page-79"></a>

```
mkdir workspace
cd workspace
```

- Create a file called Cargo.toml and add the following to it. This creates an empty workspace

```
[workspace]
resolver = "2"
members = []
```

- Add the packages ( `cargo new --lib` specifies a library instead of an executable\`)

```
cargo new hello
cargo new --lib hellolib
```

<a id="dest-79-2850"></a>

## [Exercise: Using workspaces and package dependencies](<#dest-79-2850>)

- Take a look at the generated Cargo.toml in `hello` and `hellolib` . Notice that both of them have been added to the upper level `Cargo.toml`
- The presence of `lib.rs` in `hellolib` implies a library package (see https://doc.rust- lang.org/cargo/reference/cargo-targets.html for customization options)
- Adding a dependency on `hellolib` in `Cargo.toml` for `hello`

```
[dependencies]
hellolib = {path = "../hellolib"}
```

- Using `add()` from `hellolib`

```
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
```

- [ ] Solution (click to expand) [Using community crates from crates.io](<#page-79>)
    - Rust has a vibrant ecosystem of community crates (see https://crates.io/)
        - The Rust philosophy is to keep the standard library compact and outsource functionality to community crates
    - <a id="page-80"></a> There is no hard and fast rule about using community crates, but the rule of thumb should be to ensure that the crate has a decent maturity level (indicated by the version number), and that it’s being actively maintained. Reach out to internal sources if in doubt about a crate
- Every crate published on `crates.io` has a major and minor version
    - Crates are expected to observe the major and minor `SemVer` guidelines defined here: https://doc.rust-lang.org/cargo/reference/semver.html
    - The TL;DR version is that there should be no breaking changes for the same minor version. For example, v0.11 must be compatible with v0.15 (but v0.20 may have breaking changes)

<a id="dest-80-1912"></a>

## [Crates dependencies and SemVer](<#dest-80-1912>)

- Crates can define dependencies on a specific versions of a crate, specific minor or major version, or don’t care. The following examples show the `Cargo.toml` entries for declaring a dependency on the `rand` crate
- At least `0.10.0` , but anything `< 0.11.0` is fine

```
[dependencies]
rand = { version = "0.10.0"}
```

- Only `0.10.0` , and nothing else

```
[dependencies]
rand = { version = "=0.10.0"}
```

- Don’t care; `cargo` will select the latest version

```
[dependencies]
rand = { version = "*"}
```

- Reference: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html

<a id="dest-80-5925"></a>

## [Exercise: Using the rand crate](<#dest-80-5925>)

- Modify the `helloworld` example to print a random number
- Use `cargo add rand` to add a dependency
- Use `https://docs.rs/rand/latest/rand/` as a reference for the API

Starter code — add this to `main.rs` after running `cargo add rand` :

<a id="page-81"></a>

```
use rand::RngExt;
fn main() {
    let mut rng = rand::rng();
    // TODO: Generate and print a random u32 in 1..=100
    // TODO: Generate and print a random bool
    // TODO: Generate and print a random f64
}
```

- [ ] Solution (click to expand) [Cargo.toml and Cargo.lock](<#page-81>)
    - As mentioned previously, Cargo.lock is automatically generated from Cargo.toml
        - The main idea behind Cargo.lock is to ensure reproducible builds. For example, if `Cargo.toml` had specified a version of `0.10.0` , cargo is free to choose any version that is `< 0.11.0`
        - Cargo.lock contains the specific version of the rand crate that was used during the build.
        - The recommendation is to include `Cargo.lock` in the git repo to ensure reproducible builds

<a id="dest-81-4020"></a>

## [Cargo test feature](<#dest-81-4020>)

- Rust unit tests reside in the same source file (by convention), and are usually grouped into separate module
    - The test code is never included in the actual binary. This is made possible by the `cfg` (configuration) feature. Configurations are useful for creating platform specific code ( `Linux` vs. `Windows` ) for example
    - Tests can be executed with `cargo test` . Reference: https://doc.rust- lang.org/reference/conditional-compilation.html

<a id="page-82"></a>

```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
// Will be included only during testing
#[cfg(test)]
mod tests {
    use super::*; // This makes all types in the parent scope visible
    #[test]
    fn it_works() {
        let result = add(2, 2); // Alternatively, super::add(2, 2);
        assert_eq!(result, 4);
    }
}
```

<a id="dest-82-2062"></a>

## [Other Cargo features](<#dest-82-2062>)

- `cargo` has several other useful features including:
    - `cargo clippy` is a great way of linting Rust code. In general, warnings should be fixed (or rarely suppressed if really warranted)
    - `cargo format` executes the `rustfmt` tool to format source code. Using the tool ensures standard formatting of checked-in code and puts an end to debates about style
    - `cargo doc` can be used to generate documentation from the `///` style comments. The documentation for all crates on `crates.io` was generated using this method

<a id="dest-82-4328"></a>

### [Build Profiles: Controlling Optimization](<#dest-82-4328>)

In C, you pass `-O0` , `-O2` , `-Os` , `-flto` to `gcc` / `clang` . In Rust, you configure build profiles in

`Cargo.toml` :

```
# Cargo.toml — build profile configuration
[profile.dev]
opt-level = 0          # No optimization (fast compile, like -O0)
debug = true           # Full debug symbols (like -g)
[profile.release]
opt-level = 3          # Maximum optimization (like -O3)
lto = "fat"            # Link-Time Optimization (like -flto)
strip = true           # Strip symbols (like the strip command)
codegen-units = 1      # Single codegen unit — slower compile, better optimization
panic = "abort"        # No unwind tables (smaller binary)
```

<a id="page-83"></a>

![Source region, PDF page 83](Rust-for-C_CPP-Programmers-images/page-0083-region-0037.png)

> PDF page 83: Complex table retained as a source image.

```
cargo build              # Uses [profile.dev]
cargo build --release    # Uses [profile.release]
```

<a id="dest-83-2692"></a>

### [Build Scripts (build.rs): Linking C Libraries](<#dest-83-2692>)

In C, you use Makefiles or CMake to link libraries and run code generation. Rust uses a

`build.rs` file at the crate root:

```
// build.rs — runs before compiling the crate
fn main() {
    // Link a system C library (like -lbmc_ipmi in gcc)
    println!("cargo::rustc-link-lib=bmc_ipmi");
    // Where to find the library (like -L/usr/lib/bmc)
    println!("cargo::rustc-link-search=/usr/lib/bmc");
    // Re-run if the C header changes
    println!("cargo::rerun-if-changed=wrapper.h");
}
```

You can even compile C source files directly from a Rust crate:

```
# Cargo.toml
[build-dependencies]
cc = "1"  # C compiler integration
```

<a id="page-84"></a>

```
// build.rs
fn main() {
    cc::Build::new()
        .file("src/c_helpers/ipmi_raw.c")
        .include("/usr/include/bmc")
        .compile("ipmi_raw");   // Produces libipmi_raw.a, linked automatically
    println!("cargo::rerun-if-changed=src/c_helpers/ipmi_raw.c");
}
```

C / Make / CMake Rust build.rs

```
-lfoo                println!("cargo::rustc-link-lib=foo")
-L/path              println!("cargo::rustc-link-search=/path")
```

Compile C source `cc::Build::new().file("foo.c").compile("foo")`

Generate code Write files to `$OUT_DIR` , then `include!()`

<a id="dest-84-2790"></a>

### [Cross-Compilation](<#dest-84-2790>)

In C, cross-compilation requires installing a separate toolchain ( `arm-linux-gnueabihf-gcc` ) and configuring Make/CMake. In Rust:

```
# Install a cross-compilation target
rustup target add aarch64-unknown-linux-gnu
# Cross-compile
cargo build --target aarch64-unknown-linux-gnu --release
```

Specify the linker in `.cargo/config.toml` :

```
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

C Cross-Compile Rust Equivalent

`rustup target add aarch64-unknown-linux-gnu` + install linker

`apt install gcc-aarch64-`

```
linux-gnu
CC=aarch64-linux-gnu-gcc
                                 .cargo/config.toml [target.X] linker = "..."
make
#ifdef __aarch64__               #[cfg(target_arch = "aarch64")]
```

Separate Makefile targets `cargo build --target ...`

<a id="page-85"></a>

<a id="dest-85-0"></a>

### [Feature Flags: Conditional Compilation](<#dest-85-0>)

C uses `#ifdef` and `-DFOO` for conditional compilation. Rust uses feature flags defined in

`Cargo.toml` :

```
# Cargo.toml
[features]
default = ["json"]         # Enabled by default
json = ["dep:serde_json"]  # Optional dependency
verbose = []               # Flag with no dependency
gpu = ["dep:cuda-sys"]     # Optional GPU support
```

```
// Code gated on features:
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}
#[cfg(feature = "verbose")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose"))]
macro_rules! verbose {
    ($($arg:tt)*) => {}; // Compiles to nothing
}
```

C Preprocessor Rust Feature Flags

```
gcc -DDEBUG           cargo build --features verbose
#ifdef DEBUG          #[cfg(feature = "verbose")]
#define MAX 100       const MAX: u32 = 100;
#ifdef __linux__      #[cfg(target_os = "linux")]
```

<a id="dest-85-5490"></a>

### [Integration Tests vs Unit Tests](<#dest-85-5490>)

Unit tests live next to the code with `#[cfg(test)]` . Integration tests live in `tests/` and test your crate’s public API only:

<a id="page-86"></a>

```
// tests/smoke_test.rs — no #[cfg(test)] needed
use my_crate::parse_config;
#[test]
fn parse_valid_config() {
    let config = parse_config("test_data/valid.json").unwrap();
    assert_eq!(config.max_retries, 5);
}
```

![Source region, PDF page 86](Rust-for-C_CPP-Programmers-images/page-0086-region-0038.png)

> PDF page 86: Complex table retained as a source image.

<a id="dest-86-2572"></a>

### [Testing Patterns and Strategies](<#dest-86-2572>)

C firmware teams typically write tests in CUnit, CMocka, or custom frameworks with a lot of boilerplate. Rust’s built-in test harness is far more capable. This section covers patterns you’ll need for production code.

<a id="dest-86-3668"></a>

[\#\[should\_panic\] — Testing Expected Failures](<#dest-86-3668>)

```
// Test that certain conditions cause panics (like C's assert failures)
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_bounds_check() {
    let v = vec![1, 2, 3];
    let _ = v[10];  // Should panic
}
#[test]
#[should_panic(expected = "temperature exceeds safe limit")]
fn test_thermal_shutdown() {
    fn check_temperature(celsius: f64) {
        if celsius > 105.0 {
            panic!("temperature exceeds safe limit: {celsius}°C");
        }
    }
    check_temperature(110.0);
}
```

<a id="page-87"></a>

<a id="dest-87-0"></a>

[\#\[ignore\] — Slow or Hardware-Dependent Tests](<#dest-87-0>)

```
// Mark tests that require special conditions (like C's #ifdef HARDWARE_TEST)
#[test]
#[ignore = "requires GPU hardware"]
fn test_gpu_ecc_scrub() {
    // This test only runs on machines with GPUs
    // Run with: cargo test -- --ignored
    // Run with: cargo test -- --include-ignored  (runs ALL tests)
}
```

<a id="dest-87-1792"></a>

[Result-Returning Tests (replacing unwrap chains)](<#dest-87-1792>)

```
// Instead of many unwrap() calls that hide the actual failure:
#[test]
fn test_config_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"hostname": "node-01", "port": 8080}"#;
    let config: ServerConfig = serde_json::from_str(json)?;  // ? instead of
unwrap()
    assert_eq!(config.hostname, "node-01");
    assert_eq!(config.port, 8080);
    Ok(())  // Test passes if we reach here without error
}
```

<a id="dest-87-3855"></a>

[Test Fixtures with Builder Functions](<#dest-87-3855>)

C uses `setUp()` / `tearDown()` functions. Rust uses helper functions and `Drop` :

<a id="page-88"></a>

```
struct TestFixture {
    temp_dir: std::path::PathBuf,
    config: Config,
}
impl TestFixture {
    fn new() -> Self {
        let temp_dir = std::env::temp_dir().join(format!("test_{}",
std::process::id()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let config = Config {
            log_dir: temp_dir.clone(),
            max_retries: 3,
            ..Default::default()
        };
        Self { temp_dir, config }
    }
}
impl Drop for TestFixture {
    fn drop(&mut self) {
        // Automatic cleanup — like C's tearDown() but can't be forgotten
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}
#[test]
fn test_with_fixture() {
    let fixture = TestFixture::new();
    // Use fixture.config, fixture.temp_dir...
    assert!(fixture.temp_dir.exists());
    // fixture is automatically dropped here → cleanup runs
}
```

<a id="dest-88-4845"></a>

[Mocking Traits for Hardware Interfaces](<#dest-88-4845>)

In C, mocking hardware requires preprocessor tricks or function pointer swapping. In Rust, traits make this natural:

<a id="page-89"></a>

```
// Production trait for IPMI communication
trait IpmiTransport {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String>;
}
// Real implementation (used in production)
struct RealIpmi { /* BMC connection details */ }
impl IpmiTransport for RealIpmi {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String> {
        // Actually talks to BMC hardware
        todo!("Real IPMI call")
    }
}
// Mock implementation (used in tests)
struct MockIpmi {
    responses: std::collections::HashMap<u8, Vec<u8>>,
}
impl IpmiTransport for MockIpmi {
    fn send_command(&self, cmd: u8, _data: &[u8]) -> Result<Vec<u8>, String> {
        self.responses.get(&cmd)
            .cloned()
            .ok_or_else(|| format!("No mock response for cmd 0x{cmd:02x}"))
    }
}
// Generic function that works with both real and mock
fn read_sensor_temperature(transport: &dyn IpmiTransport) -> Result<f64, String> {
    let response = transport.send_command(0x2D, &[])?;
    if response.len() < 2 {
        return Err("Response too short".into());
    }
    Ok(response[0] as f64 + (response[1] as f64 / 256.0))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_temperature_reading() {
        let mut mock = MockIpmi { responses: std::collections::HashMap::new() };
        mock.responses.insert(0x2D, vec![72, 128]); // 72.5°C
        let temp = read_sensor_temperature(&mock).unwrap();
        assert!((temp - 72.5).abs() < 0.01);
    }
    #[test]
    fn test_short_response() {
        let mock = MockIpmi { responses: std::collections::HashMap::new() };
        // No response configured → error
        assert!(read_sensor_temperature(&mock).is_err());
```

<a id="page-90"></a>

```
    }
}
```

<a id="dest-90-585"></a>

[Property-Based Testing with proptest](<#dest-90-585>)

Instead of testing specific values, test **properties** that must always hold:

```
// Cargo.toml: [dev-dependencies] proptest = "1"
use proptest::prelude::*;
fn parse_sensor_id(s: &str) -> Option<u32> {
    s.strip_prefix("sensor_")?.parse().ok()
}
fn format_sensor_id(id: u32) -> String {
    format!("sensor_{id}")
}
proptest! {
    #[test]
    fn roundtrip_sensor_id(id in 0u32..10000) {
        // Property: format then parse should give back the original
        let formatted = format_sensor_id(id);
        let parsed = parse_sensor_id(&formatted);
        prop_assert_eq!(parsed, Some(id));
    }
    #[test]
    fn parse_rejects_garbage(s in "[^s].*") {
        // Property: strings not starting with 's' should never parse
        let result = parse_sensor_id(&s);
        prop_assert!(result.is_none());
    }
}
```

<a id="dest-90-5235"></a>

[C vs Rust Testing Comparison](<#dest-90-5235>)

C Testing Rust Equivalent

`CUnit` , `CMocka` , custom Built-in `#[test]` + `cargo test` framework

`setUp()` / `tearDown()` Builder function + `Drop` trait

`#ifdef TEST` mock functions Trait-based dependency injection

`assert_eq!(x, y)` with auto diff output `assert(x == y)`

Same binary, conditional compilation with `#` Separate test executable

```
[cfg(test)]
```

<a id="page-91"></a>

C Testing Rust Equivalent

`cargo test` (memory safe by default) + `cargo`

`valgrind --leak-check=full`

```
./test                             miri test
```

Code coverage: `gcov` / `lcov` `cargo tarpaulin` or `cargo llvm-cov`

Test discovery: manual Automatic — any `#[test]` fn is discovered registration

<a id="page-92"></a>

![Source region, PDF page 92](Rust-for-C_CPP-Programmers-images/page-0092-region-0039.png)

<a id="page-93"></a>

<a id="dest-93-0"></a>

### [Test attributes beyond \#\[test\]](<#dest-93-0>)

```
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_pass() {
        assert_eq!(2 + 2, 4);
    }
    // Expect a panic — equivalent to GTest's EXPECT_DEATH
    #[test]
    #[should_panic]
    fn out_of_bounds_panics() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // Panics — test passes
    }
    // Expect a panic with a specific message substring
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn specific_panic_message() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }
    // Tests that return Result<(), E> — use ? instead of unwrap()
    #[test]
    fn test_with_result() -> Result<(), String> {
        let value: u32 = "42".parse().map_err(|e| format!("{e}"))?;
        assert_eq!(value, 42);
        Ok(())
    }
    // Ignore slow tests by default — run with `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn slow_integration_test() {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
```

```
cargo test                          # Run all non-ignored tests
cargo test -- --ignored             # Run only ignored tests
cargo test -- --include-ignored     # Run ALL tests including ignored
cargo test test_name                # Run tests matching a name pattern
cargo test -- --nocapture           # Show println! output during tests
cargo test -- --test-threads=1      # Run tests serially (for shared state)
```

<a id="page-94"></a>

<a id="dest-94-0"></a>

### [Test helpers: builder pattern for test data](<#dest-94-0>)

In C++ you’d use Google Test fixtures ( `class MyTest : public ::testing::Test` ). In Rust, use builder functions or the `Default` trait:

```
#[cfg(test)]
mod tests {
    use super::*;
    // Builder function — creates test data with sensible defaults
    fn make_gpu_event(severity: Severity, fault_code: u32) -> DiagEvent {
        DiagEvent {
            source: "accel_diag".to_string(),
            severity,
            message: format!("Test event FC:{fault_code}"),
            fault_code,
        }
    }
    // Reusable test fixture — a set of pre-built events
    fn sample_events() -> Vec<DiagEvent> {
        vec![
            make_gpu_event(Severity::Critical, 67956),
            make_gpu_event(Severity::Warning, 32709),
            make_gpu_event(Severity::Info, 10001),
        ]
    }
    #[test]
    fn filter_critical_events() {
        let events = sample_events();
        let critical: Vec<_> = events.iter()
            .filter(|e| e.severity == Severity::Critical)
            .collect();
        assert_eq!(critical.len(), 1);
        assert_eq!(critical[0].fault_code, 67956);
    }
}
```

<a id="dest-94-5760"></a>

### [Mocking with traits](<#dest-94-5760>)

In C++, mocking requires frameworks like Google Mock or manual virtual overrides. In Rust, define a trait for the dependency and swap implementations in tests:

<a id="page-95"></a>

```
// Production trait
trait SensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String>;
}
// Production implementation
struct HwSensorReader;
impl SensorReader for HwSensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
        // Real hardware call...
        Ok(72.5)
    }
}
// Test mock — returns predictable values
#[cfg(test)]
struct MockSensorReader {
    temperatures: std::collections::HashMap<u32, f64>,
}
#[cfg(test)]
impl SensorReader for MockSensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
        self.temperatures.get(&sensor_id)
            .copied()
            .ok_or_else(|| format!("Unknown sensor {sensor_id}"))
    }
}
// Function under test — generic over the reader
fn check_overtemp(reader: &impl SensorReader, ids: &[u32], threshold: f64) ->
Vec<u32> {
    ids.iter()
        .filter(|&&id| reader.read_temperature(id).unwrap_or(0.0) > threshold)
        .copied()
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detect_overtemp_sensors() {
        let mut mock = MockSensorReader { temperatures: Default::default() };
        mock.temperatures.insert(0, 72.5);
        mock.temperatures.insert(1, 91.0);  // Over threshold
        mock.temperatures.insert(2, 65.0);
        let hot = check_overtemp(&mock, &[0, 1, 2], 80.0);
        assert_eq!(hot, vec![1]);
```

<a id="page-96"></a>

```
    }
}
```

<a id="dest-96-802"></a>

### [Temporary files and directories in tests](<#dest-96-802>)

C++ tests often use platform-specific temp directories. Rust has `tempfile` :

```
// Cargo.toml: [dev-dependencies]
// tempfile = "3"
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    #[test]
    fn parse_config_from_file() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temp file that's auto-deleted when dropped
        let mut file = NamedTempFile::new()?;
        writeln!(file, r#"{{"sku": "ServerNode", "level": "Quick"}}"#)?;
        let config = load_config(file.path().to_str().unwrap())?;
        assert_eq!(config.sku, "ServerNode");
        Ok(())
        // file is deleted here — no cleanup code needed
    }
}
```

<a id="dest-96-4770"></a>

### [Property-based testing with proptest](<#dest-96-4770>)

Instead of writing specific test cases, describe properties that should hold for all inputs.

`proptest` generates random inputs and finds minimal failing cases:

<a id="page-97"></a>

```
// Cargo.toml: [dev-dependencies]
// proptest = "1"
#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    fn parse_and_format(n: u32) -> String {
        format!("{n}")
    }
    proptest! {
        #[test]
        fn roundtrip_u32(n: u32) {
            let formatted = parse_and_format(n);
            let parsed: u32 = formatted.parse().unwrap();
            prop_assert_eq!(n, parsed);
        }
        #[test]
        fn string_contains_no_null(s in "[a-zA-Z0-9 ]{0,100}") {
            prop_assert!(!s.contains('\0'));
        }
    }
}
```

<a id="dest-97-3870"></a>

### [Snapshot testing with insta](<#dest-97-3870>)

For tests that produce complex output (JSON, formatted strings), `insta` auto-generates and manages reference snapshots:

<a id="page-98"></a>

```
// Cargo.toml: [dev-dependencies]
// insta = { version = "1", features = ["json"] }
#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;
    #[test]
    fn der_entry_format() {
        let entry = DerEntry {
            fault_code: 67956,
            component: "GPU".to_string(),
            message: "ECC error detected".to_string(),
        };
        // First run: creates a snapshot file in tests/snapshots/
        // Subsequent runs: compares against the saved snapshot
        assert_json_snapshot!(entry);
    }
}
```

```
cargo insta test              # Run tests and review new/changed snapshots
cargo insta review            # Interactive review of snapshot changes
```

<a id="dest-98-3600"></a>

### [C++ vs Rust testing comparison](<#dest-98-3600>)

C++ (Google Test) Rust Notes

```
                                                                    No suite/class
TEST(Suite, Name) { }            #[test] fn name() { }
                                                          hierarchy needed
```

Built-in macro, no `ASSERT_EQ(a, b)` `assert_eq!(a, b)` framework needed

Or use `approx` crate

```
                                 assert!((a - b).abs() <
ASSERT_NEAR(a, b, eps)
                                 eps)
                                 #[should_panic(expected
EXPECT_THROW(expr, type)
                                 = "...")]
                                 #[should_panic(expected
EXPECT_DEATH(expr, "msg")
                                 = "msg")]
class Fixture : public
::testing::Test                  Default
```

Or `catch_unwind` for fine control

Builder functions +

No inheritance needed

More explicit, no Google Mock `MOCK_METHOD` Trait + test impl macro magic

`proptest!` or macro- generated tests

`INSTANTIATE_TEST_SUITE_P` (parameterized)

<a id="page-99"></a>

C++ (Google Test) Rust Notes

RAII via `Drop` — cleanup is `SetUp()` / `TearDown()` automatic

Variables dropped at end of test

Separate test binary + CMake `cargo test` — zero config

```
                                 cargo test -- --
ctest --output-on-failure
                                nocapture
```

<a id="dest-99-1628"></a>

### [Integration tests: the tests/ directory](<#dest-99-1628>)

Unit tests live inside `#[cfg(test)]` modules alongside your code. Integration tests live in a separate `tests/` directory at the crate root and test your library’s public API as an external consumer would:

```
my_crate/
├── src/
│   └── lib.rs          # Your library code
├── tests/
│   ├── smoke.rs        # Each .rs file is a separate test binary
│   ├── regression.rs
│   └── common/
│       └── mod.rs      # Shared test helpers (NOT a test itself)
└── Cargo.toml
```

```
// tests/smoke.rs — tests your crate as an external user would
use my_crate::DiagEngine;  // Only public API is accessible
#[test]
fn engine_starts_successfully() {
    let engine = DiagEngine::new("test_config.json");
    assert!(engine.is_ok());
}
#[test]
fn engine_rejects_invalid_config() {
    let engine = DiagEngine::new("nonexistent.json");
    assert!(engine.is_err());
}
```

<a id="page-100"></a>

```
// tests/common/mod.rs — shared helpers, NOT compiled as a test binary
pub fn setup_test_environment() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("config.json"), r#"{"log_level":
"debug"}"#).unwrap();
    dir
}
```

```
// tests/regression.rs — can use shared helpers
mod common;
#[test]
fn regression_issue_42() {
    let env = common::setup_test_environment();
    let engine = my_crate::DiagEngine::new(
        env.path().join("config.json").to_str().unwrap()
    );
    assert!(engine.is_ok());
}
```

#### **Running integration tests:**

```
cargo test                          # Runs unit AND integration tests
cargo test --test smoke             # Run only tests/smoke.rs
cargo test --test regression        # Run only tests/regression.rs
cargo test --lib                    # Run ONLY unit tests (skip integration)
```

Key difference from unit tests: Integration tests cannot access private functions or

`pub(crate)` items. This forces you to verify that your public API is sufficient — a valuable

design signal. In C++ terms, it’s like testing against only the public header with no `friend`

access.

<a id="page-101"></a>

9. [9. Error Handling](<#page-101>)

<a id="dest-101-930"></a>

## [Connecting enums to Option and Result](<#dest-101-930>)

What you’ll learn: How Rust replaces null pointers with `Option<T>` and exceptions with

`Result<T, E>` , and how the `?` operator makes error propagation concise. This is Rust’s most distinctive pattern — errors are values, not hidden control flow.

- Remember the `enum` type we learned earlier? Rust’s `Option` and `Result` are simply enums defined in the standard library:

```
// This is literally how Option is defined in std:
enum Option<T> {
    Some(T),  // Contains a value
    None,     // No value
}
// And Result:
enum Result<T, E> {
    Ok(T),    // Success with value
    Err(E),   // Error with details
}
```

- This means everything you learned about pattern matching with `match` works directly with `Option` and `Result`
- There is no null pointer in Rust – `Option<T>` is the replacement, and the compiler forces you to handle the `None` case

<a id="dest-101-5580"></a>

### [C++ Comparison: Exceptions vs Result](<#dest-101-5580>)

C++ Pattern Rust Equivalent Advantage

Error in return type — can’t forget to handle

```
throw
                              Err(MyError::Runtime(msg))
std::runtime_error(msg)
                              match result { Ok(v) => ...,
try { } catch (...) { }
                             Err(e) => ... }
```

No hidden control flow

<a id="page-102"></a>

C++ Pattern Rust Equivalent Advantage

Exhaustive match required — can’t forget None

```
std::optional<T>             Option<T>
```

Default — all Rust functions are `noexcept` annotation “noexcept”

Exceptions don’t exist

Type-safe, can’t `errno` / return codes `Result<T, E>` ignore

<a id="dest-102-1860"></a>

## [Rust Option type](<#dest-102-1860>)

- The Rust `Option` type is an `enum` with only two variants: `Some<T>` and `None`
    - The idea is that this represents a `nullable` type, i.e., it either contains a valid value of that type ( `Some<T>` ), or has no valid value ( `None` )
    - The `Option` type is used in APIs where the result of an operation either succeeds and returns a valid value or it fails (but the specific error is irrelevant). For example, consider parsing a string for an integer value

```
fn main() {
    // Returns Option<usize>
    let a = "1234".find("1");
    match a {
        Some(a) => println!("Found 1 at index {a}"),
        None => println!("Couldn't find 1")
    }
}
```

<a id="dest-102-4928"></a>

## [Rust Option type](<#dest-102-4928>)

- Rust `Option` can be processed in various ways
    - `unwrap()` panics if the `Option<T>` is `None` and returns `T` otherwise and it is the least preferred approach
    - `or()` can be used to return an alternative value
    - `if let` lets us test for `Some<T>`

Production patterns: See [Safe value extraction with unwrap\_or](<#dest-234-5340>) and [Functional](<#dest-236-1942>) [transforms: map, map\_err, find\_map](<#dest-236-1942>) for real-world examples from production Rust code.

<a id="page-103"></a>

```
fn main() {
  // This return an Option<usize>
  let a = "1234".find("1");
  println!("{a:?} {}", a.unwrap());
  let a = "1234".find("5").or(Some(42));
  println!("{a:?}");
  if let Some(a) = "1234".find("1") {
      println!("{a}");
  } else {
    println!("Not found in string");
  }
  // This will panic
  // "1234".find("5").unwrap();
}
```

<a id="dest-103-2198"></a>

## [Rust Result type](<#dest-103-2198>)

- Result is an `enum` type similar to `Option` with two variants: `Ok<T>` or `Err<E>`
    - `Result` is used extensively in Rust APIs that can fail. The idea is that on success, functions will return a `Ok<T>` , or they will return a specific error `Err<T>`

```
  use std::num::ParseIntError;
  fn main() {
  let a : Result<i32, ParseIntError>  = "1234z".parse();
  match a {
      Ok(n) => println!("Parsed {n}"),
      Err(e) => println!("Parsing failed {e:?}"),
  }
  let a : Result<i32, ParseIntError>  = "1234z".parse().or(Ok(-1));
  println!("{a:?}");
  if let Ok(a) = "1234".parse::<i32>() {
    println!("Let OK {a}");
  }
  // This will panic
  //"1234z".parse().unwrap();
}
```

<a id="dest-103-5970"></a>

## [Option and Result: Two Sides of the Same Coin](<#dest-103-5970>)

`Option` and `Result` are deeply related — `Option<T>` is essentially `Result<T, ()>` (a result

where the error carries no information):

![Source region, PDF page 103](Rust-for-C_CPP-Programmers-images/page-0103-region-0040.png)

> PDF page 103: Complex table retained as a source image.

<a id="page-104"></a>

![Source region, PDF page 104](Rust-for-C_CPP-Programmers-images/page-0104-region-0041.png)

> PDF page 104: Complex table retained as a source image.

#### **Converting between them:**

```
fn main() {
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("value was None");  // Option → Result
    let res: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = res.ok();  // Result → Option (discards error)
    // They share many of the same methods:
    // .map(), .and_then(), .unwrap_or(), .unwrap_or_else(), .is_some()/is_ok()
}
```

Rule of thumb: Use `Option` when absence is normal (e.g., looking up a key). Use `Result` when failure needs explanation (e.g., file I/O, parsing).

<a id="dest-104-3278"></a>

## [Exercise: log() function implementation with](<#dest-104-3278>) [Option](<#dest-104-3278>)

#### 🟢 **Starter**

    - Implement a `log()` function that accepts an `Option<&str>` parameter. If the parameter is `None` , it should print a default string
    - The function should return a `Result` with `()` for both success and error (in this case we’ll never have an error)
- [ ] Solution (click to expand) [Rust error handling](<#page-104>)
    - Rust errors can be irrecoverable (fatal) or recoverable. Fatal errors result in a \`\`panic\`\`\`
        - In general, situations that result in `panics` should be avoided. `panics` are caused by bugs in the program, including exceeding index bounds, calling `unwrap()` on an `Option<None>` , etc.
        - It is OK to have explicit `panics` for conditions that should be impossible. The `panic!` or `assert!` macros can be used for sanity checks

<a id="page-105"></a>

```
fn main() {
   let x : Option<u32> = None;
   // println!("{x}", x.unwrap()); // Will panic
   println!("{}", x.unwrap_or(0));  // OK -- prints 0
   let x = 41;
   //assert!(x == 42); // Will panic
   //panic!("Something went wrong"); // Unconditional panic
   let _a = vec![0, 1];
   // println!("{}", a[2]); // Out of bounds panic; use a.get(2) which will return
Option<T>
}
```

<a id="dest-105-2085"></a>

## [Error Handling: C++ vs Rust](<#dest-105-2085>)

<a id="dest-105-2678"></a>

### [C++ Exception-Based Error Handling Problems](<#dest-105-2678>)

```
// C++ error handling - exceptions create hidden control flow
#include <fstream>
#include <stdexcept>
std::string read_config(const std::string& path) {
    std::ifstream file(path);
    if (!file.is_open()) {
        throw std::runtime_error("Cannot open: " + path);
    }
    std::string content;
    // What if getline throws? Is file properly closed?
    // With RAII yes, but what about other resources?
    std::getline(file, content);
    return content;  // What if caller doesn't try/catch?
}
int main() {
    // ERROR: Forgot to wrap in try/catch!
    auto config = read_config("nonexistent.txt");
    // Exception propagates silently, program crashes
    // Nothing in the function signature warned us
    return 0;
}
```

<a id="page-106"></a>

![Source region, PDF page 106](Rust-for-C_CPP-Programmers-images/page-0106-region-0042.png)

<a id="dest-106-3802"></a>

### [Result&lt;T, E&gt; Visualization](<#dest-106-3802>)

```
// Rust error handling - comprehensive and forced
use std::fs::File;
use std::io::Read;
fn read_file_content(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;  // ? automatically propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)  // Success case
}
fn main() {
    match read_file_content("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Failed to read file: {}", error),
        // Compiler forces us to handle both cases!
    }
}
```

<a id="page-107"></a>

Result Flow

Function starts

File::open()

Result check

Ok(file)

Pattern Matching Options

file.read\_to\_string()

result.expect(msg) \[WARNING\] Panics with

result.unwrap\_or(default)

result? \[OK\] Early return

result.unwrap() \[WARNING\] Panics on error

match result

![Source region, PDF page 107](Rust-for-C_CPP-Programmers-images/page-0107-region-0043.png)

> PDF page 107: Mathematics retained as a source image.

\[OK\] Safe fallback

message

Err(e)

Result check

\[OK\] Handles both cases

\[OK\] Handles error case

\[OK\] Always returns value

\[OK\] Propagates to caller

\[ERROR\] Can panic

\[ERROR\] Can panic

Ok(()) Err(e)

Err(io::Error)

Ok(contents) Err(io::Error)

? operator propagates error

Caller must handle error

<a id="dest-107-2632"></a>

## [Rust error handling](<#dest-107-2632>)

- Rust uses the `enum Result<T, E>` enum for recoverable error handling
    - The `Ok<T>` variant contains the result in case of success and `Err<E>` contains the error

```
fn main() {
    let x = "1234x".parse::<u32>();
    match x {
        Ok(x) => println!("Parsed number {x}"),
        Err(e) => println!("Parsing error {e:?}"),
    }
    let x  = "1234".parse::<u32>();
    // Same as above, but with valid number
    if let Ok(x) = &x {
        println!("Parsed number {x}")
    } else if let Err(e) = &x {
        println!("Error: {e:?}");
    }
}
```

<a id="dest-107-5970"></a>

## [Rust error handling](<#dest-107-5970>)

- The try-operator `?` is a convenient short hand for the `match` `Ok` / `Err` pattern
    - Note the method must return `Result<T, E>` to enable use of `?`
    - The type for `Result<T, E>` can be changed. In the example below, we return the same error type ( `std::num::ParseIntError` ) returned by `str::parse()`

<a id="page-108"></a>

```
fn double_string_number(s : &str) -> Result<u32, std::num::ParseIntError> {
   let x = s.parse::<u32>()?; // Returns immediately in case of an error
   Ok(x*2)
}
fn main() {
    let result = double_string_number("1234");
    println!("{result:?}");
    let result = double_string_number("1234x");
    println!("{result:?}");
}
```

<a id="dest-108-1658"></a>

## [Rust error handling](<#dest-108-1658>)

- Errors can be mapped to other types, or to default values (https://doc.rust- lang.org/std/result/enum.Result.html\#method.unwrap\_or\_default)

```
// Changes the error type to () in case of error
fn double_string_number(s : &str) -> Result<u32, ()> {
   let x = s.parse::<u32>().map_err(|_|())?; // Returns immediately in case of an
error
   Ok(x*2)
}
```

```
fn double_string_number(s : &str) -> Result<u32, ()> {
   let x = s.parse::<u32>().unwrap_or_default(); // Defaults to 0 in case of parse
error
   Ok(x*2)
}
```

```
fn double_optional_number(x : Option<u32>) -> Result<u32, ()> {
    // ok_or converts Option<None> to Result<u32, ()> in the below
    x.ok_or(()).map(|x|x*2) // .map() is applied only on Ok(u32)
}
```

<a id="dest-108-5482"></a>

## [Exercise: error handling](<#dest-108-5482>)

#### 🟡 **Intermediate**

- Implement a `log()` function with a single u32 parameter. If the parameter is not 42, return an error. The `Result<>` for success and error type is `()`
- Invoke `log()` function that exits with the same `Result<>` type if `log()` return an error. Otherwise print a message saying that log was successfully called

<a id="page-109"></a>

```
fn log(x: u32) -> ?? {
}
fn call_log(x: u32) -> ?? {
    // Call log(x), then exit immediately if it return an error
    println!("log was successfully called");
}
fn main() {
    call_log(42);
    call_log(43);
}
```

- [ ] Solution (click to expand)

<a id="page-110"></a>

<a id="dest-110-158"></a>

## [Rust Option and Result key takeaways](<#dest-110-158>)

What you’ll learn: Idiomatic error handling patterns — safe alternatives to `unwrap()` , the

`?` operator for propagation, custom error types, and when to use `anyhow` vs `thiserror` in production code.

- `Option` and `Result` are an integral part of idiomatic Rust
- Safe alternatives to unwrap() :

```
// Option<T> safe alternatives
let value = opt.unwrap_or(default);              // Provide fallback value
let value = opt.unwrap_or_else(|| compute());    // Lazy computation for fallback
let value = opt.unwrap_or_default();             // Use Default trait
implementation
let value = opt.expect("descriptive message");   // Only when panic is acceptable
// Result<T, E> safe alternatives
let value = result.unwrap_or(fallback);          // Ignore error, use fallback
let value = result.unwrap_or_else(|e| handle(e)); // Handle error, return fallback
let value = result.unwrap_or_default();          // Use Default trait
```

- Pattern matching for explicit control:

```
match some_option {
    Some(value) => println!("Got: {}", value),
    None => println!("No value found"),
}
match some_result {
    Ok(value) => process(value),
    Err(error) => log_error(error),
}
```

- Use ? operator for error propagation: Short-circuit and bubble up errors

```
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?; // Automatically returns error
    Ok(content.to_uppercase())
}
```

- Transformation methods:
    - `map()` : Transform the success value `Ok(T)` -&gt; `Ok(U)` or `Some(T)` -&gt; `Some(U)`
    - `map_err()` : Transform the error type `Err(E)` -&gt; `Err(F)`
    - <a id="page-111"></a> `and_then()` : Chain operations that can fail
- Use in your own APIs: Prefer `Result<T, E>` over exceptions or error codes
- References: [Option docs](<https://doc.rust-lang.org/std/option/enum.Option.html>) \| [Result docs](<https://doc.rust-lang.org/std/result/enum.Result.html>)

<a id="dest-111-698"></a>

## [Rust Common Pitfalls and Debugging Tips](<#dest-111-698>)

- Borrowing issues: Most common beginner mistake
    - “cannot borrow as mutable” -&gt; Only one mutable reference allowed at a time
    - “borrowed value does not live long enough” -&gt; Reference outlives the data it points to
    - Fix: Use scopes `{}` to limit reference lifetimes, or clone data when needed
- Missing trait implementations: “method not found” errors
    - Fix: Add `#[derive(Debug, Clone, PartialEq)]` for common traits
    - Use `cargo check` to get better error messages than `cargo run`
- Integer overflow in debug mode: Rust panics on overflow
    - Fix: Use `wrapping_add()` , `saturating_add()` , or `checked_add()` for explicit behavior
- String vs &amp;str confusion: Different types for different use cases
    - Use `&str` for string slices (borrowed), `String` for owned strings
    - Fix: Use `.to_string()` or `String::from()` to convert `&str` to `String`
- Fighting the borrow checker: Don’t try to outsmart it
    - Fix: Restructure code to work with ownership rules rather than against them
    - Consider using `Rc<RefCell<T>>` for complex sharing scenarios (sparingly)

<a id="page-112"></a>

<a id="dest-112-0"></a>

## [Error Handling Examples: Good vs Bad](<#dest-112-0>)

```
// [ERROR] BAD: Can panic unexpectedly
fn bad_config_reader() -> String {
    let config = std::env::var("CONFIG_FILE").unwrap(); // Panic if not set!
    std::fs::read_to_string(config).unwrap()           // Panic if file missing!
}
// [OK] GOOD: Handles errors gracefully
fn good_config_reader() -> Result<String, ConfigError> {
    let config_path = std::env::var("CONFIG_FILE")
        .unwrap_or_else(|_| "default.conf".to_string()); // Fallback to default
    let content = std::fs::read_to_string(config_path)
        .map_err(ConfigError::FileRead)?;                // Convert and propagate
error
    Ok(content)
}
// [OK] EVEN BETTER: With proper error types
use thiserror::Error;
#[derive(Error, Debug)]
enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Invalid configuration: {message}")]
    Invalid { message: String },
}
```

Let’s break down what’s happening here. `ConfigError` has just two variants — one for I/O errors and one for validation errors. This is the right starting point for most modules:

ConfigError variant Holds Created by

`#[from]` auto-converts via The original I/O error `FileRead(io::Error)`

```
?
```

A human-readable Your validation code `Invalid { message }` explanation

Now you can write functions that return `Result<T, ConfigError>` :

<a id="page-113"></a>

```
fn read_config(path: &str) -> Result<String, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // io::Error →
ConfigError::FileRead
    if content.is_empty() {
        return Err(ConfigError::Invalid {
            message: "config file is empty".to_string(),
        });
    }
    Ok(content)
}
```

🟢 Self-study checkpoint: Before continuing, make sure you can answer:

1. Why does `?` on the `read_to_string` call work? (Because `#[from]` generates `impl` `From<io::Error> for ConfigError` )
2. What happens if you add a third variant `MissingKey(String)` — what code changes? (Just add the variant; existing code still compiles)

<a id="dest-113-3368"></a>

## [Crate-Level Error Types and Result Aliases](<#dest-113-3368>)

As your project grows beyond a single file, you’ll combine multiple module-level errors into a crate-level error type. This is the standard pattern in production Rust. Let’s build up from the

`ConfigError` above.

In real-world Rust projects, every crate (or significant module) defines its own `Error` enum and a `Result` type alias. This is the idiomatic pattern — analogous to how in C++ you’d define a per-library exception hierarchy and `using Result = std::expected<T, Error>` .

<a id="page-114"></a>

<a id="dest-114-0"></a>

### [The pattern](<#dest-114-0>)

```
// src/error.rs  (or at the top of lib.rs)
use thiserror::Error;
/// Every error this crate can produce.
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),          // auto-converts via From
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),     // auto-converts via From
    #[error("Invalid sensor id: {0}")]
    InvalidSensor(u32),                  // domain-specific variant
    #[error("Timeout after {ms} ms")]
    Timeout { ms: u64 },
}
/// Crate-wide Result alias — saves typing throughout the crate.
pub type Result<T> = core::result::Result<T, Error>;
```

<a id="dest-114-3660"></a>

### [How it simplifies every function](<#dest-114-3660>)

#### Without the alias you’d write:

```
// Verbose — error type repeated everywhere
fn read_sensor(id: u32) -> Result<f64, crate::Error> { ... }
fn parse_config(path: &str) -> Result<Config, crate::Error> { ... }
```

#### With the alias:

```
// Clean — just `Result<T>`
use crate::{Error, Result};
fn read_sensor(id: u32) -> Result<f64> {
    if id > 128 {
        return Err(Error::InvalidSensor(id));
    }
    let raw = std::fs::read_to_string(format!("/dev/sensor/{id}"))?; // io::Error
→ Error::Io
    let value: f64 = raw.trim().parse()
        .map_err(|_| Error::InvalidSensor(id))?;
    Ok(value)
}
```

<a id="page-115"></a>

The `#[from]` attribute on `Io` generates this `impl` for free:

```
// Auto-generated by thiserror's #[from]
impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io(source)
    }
}
```

That’s what makes `?` work: when a function returns `std::io::Error` and your function returns `Result<T>` (your alias), the compiler calls `From::from()` to convert it automatically.

<a id="dest-115-2092"></a>

### [Composing module-level errors](<#dest-115-2092>)

Larger crates split errors by module, then compose them at the crate root:

```
// src/config/error.rs
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Missing key: {0}")]
    MissingKey(String),
    #[error("Invalid value for '{key}': {reason}")]
    InvalidValue { key: String, reason: String },
}
// src/error.rs  (crate-level)
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]               // delegates Display to inner error
    Config(#[from] crate::config::ConfigError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
pub type Result<T> = core::result::Result<T, Error>;
```

Callers can still match on specific config errors:

```
match result {
    Err(Error::Config(ConfigError::MissingKey(k))) => eprintln!("Add '{k}' to
config"),
    Err(e) => eprintln!("Other error: {e}"),
    Ok(v) => use_value(v),
}
```

<a id="page-116"></a>

<a id="dest-116-0"></a>

[C++ comparison](<#dest-116-0>)

Concept C++ Rust

Error hierarchy

```
                                 #[derive(thiserror::Error)] enum
std::runtime_error               Error { ... }
                                 fn foo() -> Result<T>
throw
```

`class AppError : public`

Return error

`std::expected<T, Error>` or

Convert Manual `try/catch` + rethrow `#[from]` + `?` — zero boilerplate error

```
template<class T> using
                                 pub type Result<T> =
Result = std::expected<T,
                                 core::result::Result<T, Error>;
Error>;
```

Result alias

Error `#[error("...")]` — compiled into Override `what()` message

`Display` impl

<a id="page-117"></a>

<a id="dest-117-158"></a>

## [Rust traits](<#dest-117-158>)

What you’ll learn: Traits — Rust’s answer to interfaces, abstract base classes, and operator overloading. You’ll learn how to define traits, implement them for your types, and use dynamic dispatch ( `dyn Trait` ) vs static dispatch (generics). For C++ developers: traits replace virtual functions, CRTP, and concepts. For C developers: traits are the structured way Rust does polymorphism.

- Rust traits are similar to interfaces in other languages
    - Traits define methods that must be defined by types that implement the trait.

```
fn main() {
    trait Pet {
        fn speak(&self);
    }
    struct Cat;
    struct Dog;
    impl Pet for Cat {
        fn speak(&self) {
            println!("Meow");
        }
    }
    impl Pet for Dog {
        fn speak(&self) {
            println!("Woof!")
        }
    }
    let c = Cat{};
    let d = Dog{};
    c.speak();  // There is no "is a" relationship between Cat and Dog
    d.speak(); // There is no "is a" relationship between Cat and Dog
}
```

<a id="page-118"></a>

<a id="dest-118-0"></a>

## [Traits vs C++ Concepts and Interfaces](<#dest-118-0>)

<a id="dest-118-592"></a>

### [Traditional C++ Inheritance vs Rust Traits](<#dest-118-592>)

```
// C++ - Inheritance-based polymorphism
class Animal {
public:
    virtual void speak() = 0;  // Pure virtual function
    virtual ~Animal() = default;
};
class Cat : public Animal {  // "Cat IS-A Animal"
public:
    void speak() override {
        std::cout << "Meow" << std::endl;
    }
};
void make_sound(Animal* animal) {  // Runtime polymorphism
    animal->speak();  // Virtual function call
}
```

```
// Rust - Composition over inheritance with traits
trait Animal {
    fn speak(&self);
}
struct Cat;  // Cat is NOT an Animal, but IMPLEMENTS Animal behavior
impl Animal for Cat {  // "Cat CAN-DO Animal behavior"
    fn speak(&self) {
        println!("Meow");
    }
}
fn make_sound<T: Animal>(animal: &T) {  // Static polymorphism
    animal.speak();  // Direct function call (zero cost)
}
```

![Source region, PDF page 118](Rust-for-C_CPP-Programmers-images/page-0118-region-0044.png)

<a id="page-119"></a>

<a id="dest-119-0"></a>

### [Trait Bounds and Generic Constraints](<#dest-119-0>)

```
use std::fmt::Display;
use std::ops::Add;
// C++ template equivalent (less constrained)
// template<typename T>
// T add_and_print(T a, T b) {
//     // No guarantee T supports + or printing
//     return a + b;  // Might fail at compile time
// }
// Rust - explicit trait bounds
fn add_and_print<T>(a: T, b: T) -> T
where
    T: Display + Add<Output = T> + Copy,
{
    println!("Adding {} + {}", a, b);  // Display trait
    a + b  // Add trait
}
```

![Source region, PDF page 119](Rust-for-C_CPP-Programmers-images/page-0119-region-0045.png)

<a id="dest-119-5168"></a>

### [C++ Operator Overloading → Rust std::ops Traits](<#dest-119-5168>)

In C++, you overload operators by writing free functions or member functions with special names ( `operator+` , `operator<<` , `operator[]` , etc.). In Rust, every operator maps to a trait in

`std::ops` (or `std::fmt` for output). You implement the trait instead of writing a magic- named function.

<a id="page-120"></a>

<a id="dest-120-0"></a>

[Side-by-side: + operator](<#dest-120-0>)

```
// C++: operator overloading as a member or free function
struct Vec2 {
    double x, y;
    Vec2 operator+(const Vec2& rhs) const {
        return {x + rhs.x, y + rhs.y};
    }
};
Vec2 a{1.0, 2.0}, b{3.0, 4.0};
Vec2 c = a + b;  // calls a.operator+(b)
```

```
use std::ops::Add;
#[derive(Debug, Clone, Copy)]
struct Vec2 { x: f64, y: f64 }
impl Add for Vec2 {
    type Output = Vec2;                     // Associated type — the result of +
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
let a = Vec2 { x: 1.0, y: 2.0 };
let b = Vec2 { x: 3.0, y: 4.0 };
let c = a + b;  // calls <Vec2 as Add>::add(a, b)
println!("{c:?}"); // Vec2 { x: 4.0, y: 6.0 }
```

<a id="dest-120-4492"></a>

[Key differences from C++](<#dest-120-4492>)

![Source region, PDF page 120](Rust-for-C_CPP-Programmers-images/page-0120-region-0046.png)

> PDF page 120: Complex table retained as a source image.

<a id="page-121"></a>

![Source region, PDF page 121](Rust-for-C_CPP-Programmers-images/page-0121-region-0047.png)

> PDF page 121: Complex table retained as a source image.

<a id="dest-121-652"></a>

[The self by value gotcha](<#dest-121-652>)

In Rust, `Add::add(self, rhs)` takes `self` by value. For `Copy` types (like `Vec2` above, which derives `Copy` ) this is fine — the compiler copies. But for non- `Copy` types, `+` consumes the operands:

```
let s1 = String::from("hello ");
let s2 = String::from("world");
let s3 = s1 + &s2;  // s1 is MOVED into s3!
```

`// println!("{s1}");  //` ❌ `Compile error: value used after move` `println!("{s2}");     //` ✅ `s2 was only borrowed (&s2)`

This is why `String + &str` works but `&str + &str` does not — `Add` is only implemented for

`String + &str` , consuming the left-hand `String` to reuse its buffer. This has no C++ analogue:

`std::string::operator+` always creates a new string.

<a id="dest-121-3435"></a>

[Full mapping: C++ operators → Rust traits](<#dest-121-3435>)

C++ Operator Rust Trait Notes

`Output` associated type `operator+` `std::ops::Add`

```
operator-              std::ops::Sub
```

Not pointer deref — that’s `operator*` `std::ops::Mul`

```
                                                   Deref
operator/              std::ops::Div
operator%              std::ops::Rem
```

`operator-` (unary) `std::ops::Neg`

`operator!` /

Rust uses `!` for both logical and `std::ops::Not` bitwise NOT (no `~` operator) `operator~`

`operator&` , `|` , `^` `BitAnd` , `BitOr` , `BitXor`

`operator<<` , `>>` `Shl` , `Shr` NOT stream I/O\! (shift)

Takes `&mut self` (not `self` ) `operator+=` `std::ops::AddAssign`

`std::ops::Index` / `operator[]`

Returns `&Output` / `&mut`

```
IndexMut                   Output
```

<a id="page-122"></a>

C++ Operator Rust Trait Notes

Closures implement these; you `Fn` / `FnMut` / `FnOnce` `operator()` cannot `impl Fn` directly `PartialEq` (+ `Eq` ) In `std::cmp` , not `std::ops` `operator==` `PartialOrd` (+ `Ord` ) In `std::cmp` `operator<` `operator<<` `fmt::Display` `println!("{}", x)` (stream)

`operator<<` (debug) `fmt::Debug` `println!("{:?}", x)`

Use `impl From<T> for bool` or a named method like

No direct equivalent `operator bool`

```
.is_empty()
```

`operator T()` No implicit conversions Use `From` / `Into` traits (explicit) (implicit conversion)

<a id="dest-122-2812"></a>

[Guardrails: what Rust prevents](<#dest-122-2812>)

1. No implicit conversions: C++ `operator int()` can cause silent, surprising casts. Rust has no implicit conversion operators — use `From` / `Into` and call `.into()` explicitly.
2. No overloading &amp;&amp; / \|\| : C++ allows it (breaking short-circuit semantics\!). Rust does not.
3. No overloading = : Assignment is always a move or copy, never user-defined. Compound assignment ( `+=` ) IS overloadable via `AddAssign` , etc.
4. No overloading , : C++ allows `operator,()` — one of the most infamous C++ footguns. Rust does not.
5. No overloading &amp; (address-of): Another C++ footgun ( `std::addressof` exists to work around it). Rust’s `&` always means “borrow.”
6. Coherence rules: You can only implement `Add<Foreign>` for your own type, or `Add<YourType>` for a foreign type — never `Add<Foreign>` for `Foreign` . This prevents conflicting operator definitions across crates.

Bottom line: In C++, operator overloading is powerful but largely unregulated — you can overload almost anything, including comma and address-of, and implicit conversions can trigger silently. Rust gives you the same expressiveness for arithmetic and comparison operators via traits, but blocks the historically dangerous overloads and forces all conversions to be explicit.

<a id="page-123"></a>

<a id="dest-123-0"></a>

## [Rust traits](<#dest-123-0>)

- Rust allows implementing a user defined trait on even built-in types like u32 in this example. However, either the trait or the type must belong to the crate

```
trait IsSecret {
  fn is_secret(&self);
}
// The IsSecret trait belongs to the crate, so we are OK
impl IsSecret for u32 {
  fn is_secret(&self) {
      if *self == 42 {
          println!("Is secret of life");
      }
  }
}
fn main() {
  42u32.is_secret();
  43u32.is_secret();
}
```

<a id="dest-123-3420"></a>

## [Rust traits](<#dest-123-3420>)

- Traits support interface inheritance and default implementations

<a id="page-124"></a>

```
trait Animal {
  // Default implementation
  fn is_mammal(&self) -> bool {
    true
  }
}
trait Feline : Animal {
  // Default implementation
  fn is_feline(&self) -> bool {
    true
  }
}
struct Cat;
// Use default implementations. Note that all traits for the supertrait must be
individually implemented
impl Feline for Cat {}
impl Animal for Cat {}
fn main() {
  let c = Cat{};
  println!("{} {}", c.is_mammal(), c.is_feline());
}
```

<a id="dest-124-3412"></a>

## [Exercise: Logger trait implementation](<#dest-124-3412>)

#### 🟡 **Intermediate**

    - Implement a `Log trait` with a single method called log() that accepts a u64
        - Implement two different loggers `SimpleLogger` and `ComplexLogger` that implement the `Log trait` . One should output “Simple logger” with the `u64` and the other should output “Complex logger” with the `u64`
- [ ] Solution (click to expand)

<a id="page-125"></a>

<a id="dest-125-0"></a>

## [Rust trait associated types](<#dest-125-0>)

```
#[derive(Debug)]
struct Small(u32);
#[derive(Debug)]
struct Big(u32);
trait Double {
    type T;
    fn double(&self) -> Self::T;
}
impl Double for Small {
    type T = Big;
    fn double(&self) -> Self::T {
        Big(self.0 * 2)
    }
}
fn main() {
    let a = Small(42);
    println!("{:?}", a.double());
}
```

<a id="dest-125-3352"></a>

## [Rust trait impl](<#dest-125-3352>)

- `impl` can be used with traits to accept any type that implements a trait

```
trait Pet {
    fn speak(&self);
}
struct Dog {}
struct Cat {}
impl Pet for Dog {
    fn speak(&self) {println!("Woof!")}
}
impl Pet for Cat {
    fn speak(&self) {println!("Meow")}
}
fn pet_speak(p: &impl Pet) {
    p.speak();
}
fn main() {
    let c = Cat {};
    let d = Dog {};
    pet_speak(&c);
    pet_speak(&d);
}
```

<a id="page-126"></a>

<a id="dest-126-0"></a>

## [Rust trait impl](<#dest-126-0>)

- `impl` can be also be used be used in a return value

```
trait Pet {}
struct Dog;
struct Cat;
impl Pet for Cat {}
impl Pet for Dog {}
fn cat_as_pet() -> impl Pet {
    let c = Cat {};
    c
}
fn dog_as_pet() -> impl Pet {
    let d = Dog {};
    d
}
fn main() {
    let p = cat_as_pet();
    let d = dog_as_pet();
}
```

<a id="dest-126-3525"></a>

## [Rust dynamic traits](<#dest-126-3525>)

- Dynamic traits can be used to invoke the trait functionality without knowing the underlying type. This is known as `type erasure`

```
trait Pet {
    fn speak(&self);
}
struct Dog {}
struct Cat {x: u32}
impl Pet for Dog {
    fn speak(&self) {println!("Woof!")}
}
impl Pet for Cat {
    fn speak(&self) {println!("Meow")}
}
fn pet_speak(p: &dyn Pet) {
    p.speak();
}
fn main() {
    let c = Cat {x: 42};
    let d = Dog {};
    pet_speak(&c);
    pet_speak(&d);
}
```

<a id="page-127"></a>

<a id="dest-127-465"></a>

### [Choosing Between impl Trait, dyn Trait, and Enums](<#dest-127-465>)

These three approaches all achieve polymorphism but with different trade-offs:

Heterogeneous Approach Dispatch Performance

When to

collections?

use

Default choice. Function arguments, return types

Zero-cost — inlined at compile time

No — each slot has one concrete type

```
impl
```

Static (monomorphized)

`Trait` / generics

When you need mixed types in a collection, or plugin- style extensibility

Small overhead per call (~1 pointer indirection)

Yes —

```
dyn
                                                    Vec<Box<dyn
                                                   Trait>>
```

Dynamic (vtable) `Trait`

When the set of variants is closed and known at compile time

Zero-cost — known variants at compile time

Yes — but only known variants

Match `enum`

<a id="page-128"></a>

```
trait Shape {
    fn area(&self) -> f64;
}
struct Circle { radius: f64 }
struct Rect { w: f64, h: f64 }
impl Shape for Circle { fn area(&self) -> f64 { std::f64::consts::PI * self.radius
* self.radius } }
impl Shape for Rect   { fn area(&self) -> f64 { self.w * self.h } }
// Static dispatch — compiler generates separate code for each type
fn print_area(s: &impl Shape) { println!("{}", s.area()); }
// Dynamic dispatch — one function, works with any Shape behind a pointer
fn print_area_dyn(s: &dyn Shape) { println!("{}", s.area()); }
// Enum — closed set, no trait needed
enum ShapeEnum { Circle(f64), Rect(f64, f64) }
impl ShapeEnum {
    fn area(&self) -> f64 {
        match self {
            ShapeEnum::Circle(r) => std::f64::consts::PI * r * r,
            ShapeEnum::Rect(w, h) => w * h,
        }
    }
}
```

For C++ developers: `impl Trait` is like C++ templates (monomorphized, zero-cost). `dyn`

`Trait` is like C++ virtual functions (vtable dispatch). Rust enums with `match` are like

`std::variant` with `std::visit` — but exhaustive matching is enforced by the compiler.

Rule of thumb: Start with `impl Trait` (static dispatch). Reach for `dyn Trait` only when you need heterogeneous collections or can’t know the concrete type at compile time. Use

`enum` when you own all the variants.

<a id="page-129"></a>

<a id="dest-129-158"></a>

## [Rust generics](<#dest-129-158>)

**What you’ll learn:** Generic type parameters, monomorphization (zero-cost generics), trait bounds, and how Rust generics compare to C++ templates — with better error messages and no SFINAE.

- Generics allow the same algorithm or data structure to be reused across data types
    - The generic parameter appears as an identifier within `<>` , e.g.: `<T>` . The parameter can have any legal identifier name, but is typically kept short for brevity
    - The compiler performs monomorphization at compile time, i.e., it generates a new type for every variation of `T` that is encountered

```
// Returns a tuple of type <T> composed of left and right of type <T>
fn pick<T>(x: u32, left: T, right: T) -> (T, T) {
   if x == 42 {
    (left, right)
   } else {
    (right, left)
   }
}
fn main() {
    let a = pick(42, true, false);
    let b = pick(42, "hello", "world");
    println!("{a:?}, {b:?}");
}
```

<a id="dest-129-4635"></a>

## [Rust generics](<#dest-129-4635>)

- Generics can also be applied to data types and associated methods. It is possible to specialize the implementation for a specific `<T>` (example: `f32` vs. `u32` )

<a id="page-130"></a>

```
#[derive(Debug)] // We will discuss this later
struct Point<T> {
    x : T,
    y : T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point {x, y}
    }
    fn set_x(&mut self, x: T) {
         self.x = x;
    }
    fn set_y(&mut self, y: T) {
         self.y = y;
    }
}
impl Point<f32> {
    fn is_secret(&self) -> bool {
        self.x == 42.0
    }
}
fn main() {
    let mut p = Point::new(2, 4); // i32
    let q = Point::new(2.0, 4.0); // f32
    p.set_x(42);
    p.set_y(43);
    println!("{p:?} {q:?} {}", q.is_secret());
}
```

<a id="dest-130-4088"></a>

## [Exercise: Generics](<#dest-130-4088>)

#### 🟢 **Starter**

    - Modify the `Point` type to use two different types ( `T` and `U` ) for x and y
- [ ] Solution (click to expand)

<a id="dest-130-5685"></a>

### [Combining Rust traits and generics](<#dest-130-5685>)

- Traits can be used to place restrictions on generic types (constraints)
- The constraint can be specified using a `:` after the generic type parameter, or using `where` . The following defines a generic function `get_area` that takes any type `T` as long as it implements the `ComputeArea` `trait`

<a id="page-131"></a>

```
    trait ComputeArea {
        fn area(&self) -> u64;
    }
    fn get_area<T: ComputeArea>(t: &T) -> u64 {
        t.area()
    }
```

- [▶ Try it in the Rust Playground](<https://play.rust-lang.org/>)

<a id="dest-131-1605"></a>

### [Combining Rust traits and generics](<#dest-131-1605>)

- It is possible to have multiple trait constraints

```
trait Fish {}
trait Mammal {}
struct Shark;
struct Whale;
impl Fish for Shark {}
impl Fish for Whale {}
impl Mammal for Whale {}
fn only_fish_and_mammals<T: Fish + Mammal>(_t: &T) {}
fn main() {
    let w = Whale {};
    only_fish_and_mammals(&w);
    let _s = Shark {};
    // Won't compile
    only_fish_and_mammals(&_s);
}
```

<a id="dest-131-4748"></a>

### [Rust traits constraints in data types](<#dest-131-4748>)

- Trait constraints can be combined with generics in data types
- In the following example, we define the `PrintDescription` `trait` and a generic `struct` `Shape` with a member constrained by the trait

<a id="page-132"></a>

```
trait PrintDescription {
    fn print_description(&self);
}
struct Shape<S: PrintDescription> {
    shape: S,
}
// Generic Shape implementation for any type that implements PrintDescription
impl<S: PrintDescription> Shape<S> {
    fn print(&self) {
        self.shape.print_description();
    }
}
```

- [▶ Try it in the Rust Playground](<https://play.rust-lang.org/>)

<a id="dest-132-2220"></a>

## [Exercise: Trait constraints and generics](<#dest-132-2220>)

#### 🟡 **Intermediate**

- Implement a `struct` with a generic member `cipher` that implements `CipherText`

```
trait CipherText {
    fn encrypt(&self);
}
// TO DO
//struct Cipher<>
```

- Next, implement a method called `encrypt` on the `struct` `impl` that invokes `encrypt` on

```
    cipher
// TO DO
impl for Cipher<> {}
```

    - Next, implement `CipherText` on two structs called `CipherOne` and `CipherTwo` (just `println()` is fine). Create `CipherOne` and `CipherTwo` , and use `Cipher` to invoke them
- [ ] Solution (click to expand)

<a id="dest-132-6405"></a>

### [Rust type state pattern and generics](<#dest-132-6405>)

- Rust types can be used to enforce state machine transitions at compile time
    - <a id="page-133"></a> Consider a `Drone` with say two states: `Idle` and `Flying` . In the `Idle` state, the only permitted method is `takeoff()` . In the `Flying` state, we permit `land()`
- One approach is to model the state machine using something like the following

```
enum DroneState {
    Idle,
    Flying
}
struct Drone {x: u64, y: u64, z: u64, state: DroneState}  // x, y, z are
coordinates
```

- This requires a lot of runtime checks to enforce the state machine semantics — [▶ try it](<https://play.rust-lang.org/>) to see why

<a id="dest-133-2550"></a>

### [Rust type state pattern generics](<#dest-133-2550>)

- Generics allows us to enforce the state machine at compile time. This requires using a special generic called `PhantomData<T>`
- The `PhantomData<T>` is a `zero-sized` marker data type. In this case, we use it to represent the `Idle` and `Flying` states, but it has `zero` runtime size
- Notice that the `takeoff` and `land` methods take `self` as a parameter. This is referred to as `consuming` (contrast with `&self` which uses borrowing). Basically, once we call the `takeoff()` on `Drone<Idle>` , we can only get back a `Drone<Flying>` and viceversa

```
struct Drone<T> {x: u64, y: u64, z: u64, state: PhantomData<T> }
impl Drone<Idle> {
    fn takeoff(self) -> Drone<Flying> {...}
}
impl Drone<Flying> {
    fn land(self) -> Drone<Idle> { ...}
}
```

```
- [▶ Try it in the Rust Playground](https://play.rust-lang.org/)
```

<a id="dest-133-6112"></a>

### [Rust type state pattern generics](<#dest-133-6112>)

- Key takeaways:
    - States can be represented using structs (zero-size)
    - We can combine the state `T` with `PhantomData<T>` (zero-size)
    - Implementing the methods for a particular stage of the state machine is now just a matter of `impl State<T>`
- <a id="page-134"></a> Use a method that consumes `self` to transition from one state to another
- This gives us `zero cost` abstractions. The compiler can enforce the state machine at compile time and it’s impossible to call methods unless the state is right

<a id="dest-134-885"></a>

### [Rust builder pattern](<#dest-134-885>)

- The consume `self` can be useful for builder patterns
- Consider a GPIO configuration with several dozen pins. The pins can be configured to high or low (default is low)

```
#[derive(default)]
enum PinState {
    #[default]
    Low,
    High,
}
#[derive(default)]
struct GPIOConfig {
    pin0: PinState,
    pin1: PinState
    ...
}
```

- The builder pattern can be used to construct a GPIO configuration by chaining — [▶ Try it](<https://play.rust-lang.org/>)

<a id="page-135"></a>

<a id="dest-135-158"></a>

## [Rust From and Into traits](<#dest-135-158>)

What you’ll learn: Rust’s type conversion traits — `From<T>` and `Into<T>` for infallible conversions, `TryFrom` and `TryInto` for fallible ones. Implement `From` and get `Into` for free. Replaces C++ conversion operators and constructors.

- `From` and `Into` are complementary traits to facilitate type conversion
- Types normally implement the `From` trait. `String::from("Rust")` converts a `&str` into a `String` . When `From<T> for U` exists, Rust also provides `Into<U> for T` , so `let s:` `String = "Rust".into();` works too.

```
struct Point {x: u32, y: u32}
// Construct a Point from a tuple
impl From<(u32, u32)> for Point {
    fn from(xy : (u32, u32)) -> Self {
        Point {x : xy.0, y: xy.1}       // Construct Point using the tuple
elements
    }
}
fn main() {
    let s = String::from("Rust");
    let x = u32::from(true);
    let p = Point::from((40, 42));
    // let p : Point = (40,42).into(); // Alternate form of the above
    println!("s: {s} x:{x} p.x:{} p.y {}", p.x, p.y);
}
```

<a id="dest-135-4762"></a>

## [Exercise: From and Into](<#dest-135-4762>)

    - Implement a `From` trait for `Point` to convert into a type called `TransposePoint` . `TransposePoint` swaps the `x` and `y` elements of `Point`
- [ ] Solution (click to expand) [Rust Default trait](<#page-135>)
    - `Default` can be used to implement default values for a type
        - Types can use the `Derive` macro with `Default` or provide a custom implementation

<a id="page-136"></a>

```
#[derive(Default, Debug)]
struct Point {x: u32, y: u32}
#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}
impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}
fn main() {
    let x = Point::default();   // Creates a Point{0, 0}
    println!("{x:?}");
    let y = CustomPoint::default();
    println!("{y:?}");
}
```

<a id="dest-136-2520"></a>

### [Rust Default trait](<#dest-136-2520>)

- `Default` trait has several use cases including
    - Performing a partial copy and using default initialization for rest
    - Default alternative for `Option` types in methods like `unwrap_or_default()`

```
#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}
impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}
fn main() {
    let x = CustomPoint::default();
    // Override y, but leave rest of elements as the default
    let y = CustomPoint {y: 43, ..CustomPoint::default()};
    println!("{x:?} {y:?}");
    let z : Option<CustomPoint> = None;
    // Try changing the unwrap_or_default() to unwrap()
    println!("{:?}", z.unwrap_or_default());
}
```

<a id="dest-136-6165"></a>

### [Other Rust type conversions](<#dest-136-6165>)

- Rust doesn’t support implicit type conversions and `as` can be used for `explicit` conversions
- `as` should be sparingly used because it’s subject to loss of data by narrowing and so forth. In general, it’s preferable to use `into()` or `from()` where possible

<a id="page-137"></a>

```
fn main() {
    let f = 42u8;
    // let g : u32 = f;    // Will not compile
    let g = f as u32;      // Ok, but not preferred. Subject to rules around
narrowing
    let g : u32 = f.into(); // Most preferred form; infallible and checked by the
compiler
    // let k : u8 = g.into();  // Fails to compile; narrowing can result in loss
of data
    // Attempting a narrowing operation requires use of try_into
    if let Ok(k) = TryInto::<u8>::try_into(g) {
        println!("{k}");
    }
}
```

<a id="page-138"></a>

12. [12. Closures](<#page-138>)

<a id="dest-138-930"></a>

## [Rust closures](<#dest-138-930>)

What you’ll learn: Closures as anonymous functions, the three capture traits ( `Fn` ,

`FnMut` , `FnOnce` ), `move` closures, and how Rust closures compare to C++ lambdas — with automatic capture analysis instead of manual `[&]` / `[=]` specifications.

- Closures are anonymous functions that can capture their environment
    - C++ equivalent: lambdas ( `[&](int x) { return x + 1; }` )
    - Key difference: Rust closures have three capture traits ( `Fn` , `FnMut` , `FnOnce` ) that the compiler selects automatically
    - C++ capture modes ( `[=]` , `[&]` , `[this]` ) are manual and error-prone (dangling `[&]` \!)
    - Rust’s borrow checker prevents dangling captures at compile time
- Closures can be identified by the `||` symbol. The parameters for the types are enclosed within the `||` and can use type inference
- Closures are frequently used in conjunction with iterators (next topic)

```
fn add_one(x: u32) -> u32 {
    x + 1
}
fn main() {
    let add_one_v1 = |x : u32| {x + 1}; // Explicitly specified type
    let add_one_v2 = |x| {x + 1};   // Type is inferred from call site
    let add_one_v3 = |x| x+1;   // Permitted for single line functions
    println!("{} {} {} {}", add_one(42), add_one_v1(42), add_one_v2(42),
add_one_v3(42) );
}
```

<a id="dest-138-5842"></a>

## [Exercise: Closures and capturing](<#dest-138-5842>)

#### 🟡 **Intermediate**

- Create a closure that captures a `String` from the enclosing scope and appends to it (hint: use `move` )
- Create a vector of closures: `Vec<Box<dyn Fn(i32) -> i32>>` containing closures that add 1, multiply by 2, and square the input. Iterate over the vector and apply each closure to 138/289

<a id="page-139"></a>

the number 5

- [ ] Solution (click to expand) [Rust iterators](<#dest-139-615>)
    - Iterators are one of the most powerful features of Rust. They enable very elegant methods for performing operations on collections, including filtering ( `filter()` ), transformation ( `map()` ), filter and map ( `filter_map()` ), searching ( `find()` ) and much more
    - In the example below, the `|&x| *x >= 42` is a closure that performs the same comparison. The `|x| println!("{x}")` is another closure

```
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    for x in &a {
        if *x >= 42 {
            println!("{x}");
        }
    }
    // Same as above
    a.iter().filter(|&x| *x >= 42).for_each(|x| println!("{x}"))
}
```

<a id="dest-139-3952"></a>

<a id="dest-139-615"></a>

## [Rust iterators](<#dest-139-3952>)

- A key feature of iterators is that most of them are `lazy` , i.e., they do not do anything until they are evaluated. For example, `a.iter().filter(|&x| *x >= 42);` wouldn’t have done anything without the `for_each` . The Rust compiler emits an explicit warning when it detects such a situation

```
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    // Add one to each element and print it
    let _ = a.iter().map(|x|x + 1).for_each(|x|println!("{x}"));
    let found = a.iter().find(|&x|*x == 42);
    println!("{found:?}");
    // Count elements
    let count = a.iter().count();
    println!("{count}");
}
```

<a id="page-140"></a>

<a id="dest-140-0"></a>

## [Rust iterators](<#dest-140-0>)

- The `collect()` method can be used to gather the results into a separate collection
    - In the below the `_` in `Vec<_>` is the equivalent of a wildcard character for the type returned by the `map` . For example, we can even return a `String` from `map`

```
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    let squared_a : Vec<_> = a.iter().map(|x|x*x).collect();
    for x in &squared_a {
        println!("{x}");
    }
    let squared_a_strings : Vec<_> = a.iter().map(|x|(x*x).to_string()).collect();
    // These are actually string representations
    for x in &squared_a_strings {
        println!("{x}");
    }
}
```

<a id="dest-140-3075"></a>

## [Exercise: Rust iterators](<#dest-140-3075>)

#### 🟢 **Starter**

    - Create an integer array composed of odd and even elements. Iterate over the array and split it into two different vectors with even and odd elements in each
    - Can this be done in a single pass (hint: use `partition()` )?
- [ ] Solution (click to expand)

Production patterns: See [Collapsing assignment pyramids with closures](<#dest-242-930>) for real iterator chains ( `.map().collect()` , `.filter().collect()` , `.find_map()` ) from production Rust code.

<a id="dest-140-5955"></a>

### [Iterator power tools: the methods that replace C++ loops](<#dest-140-5955>)

The following iterator adapters are used extensively in production Rust code. C++ has

`<algorithm>` and C++20 ranges, but Rust’s iterator chains are more composable and more commonly used.

<a id="page-141"></a>

<a id="dest-141-0"></a>

[enumerate — index + value (replaces for (int i = 0; ...))](<#dest-141-0>)

```
let sensors = vec!["temp0", "temp1", "temp2"];
for (idx, name) in sensors.iter().enumerate() {
    println!("Sensor {idx}: {name}");
}
// Sensor 0: temp0
// Sensor 1: temp1
// Sensor 2: temp2
```

C++ equivalent: `for (size_t i = 0; i < sensors.size(); ++i) { auto& name = sensors[i];`

```
... }
```

<a id="dest-141-2138"></a>

[zip — pair elements from two iterators (replaces parallel index loops)](<#dest-141-2138>)

```
let names = ["gpu0", "gpu1", "gpu2"];
let temps = [72.5, 68.0, 75.3];
let report: Vec<String> = names.iter()
    .zip(temps.iter())
    .map(|(name, temp)| format!("{name}: {temp}°C"))
    .collect();
println!("{report:?}");
// ["gpu0: 72.5°C", "gpu1: 68.0°C", "gpu2: 75.3°C"]
// Stops at the shorter iterator — no out-of-bounds risk
```

C++ equivalent: `for (size_t i = 0; i < std::min(names.size(), temps.size()); ++i) {`

```
... }
```

<a id="dest-141-4822"></a>

[flat\_map — map + flatten nested collections](<#dest-141-4822>)

```
// Each GPU has multiple PCIe BDFs; collect all BDFs across all GPUs
let gpu_bdfs = vec![
    vec!["0000:01:00.0", "0000:02:00.0"],
    vec!["0000:41:00.0"],
    vec!["0000:81:00.0", "0000:82:00.0"],
];
let all_bdfs: Vec<&str> = gpu_bdfs.iter()
    .flat_map(|bdfs| bdfs.iter().copied())
    .collect();
println!("{all_bdfs:?}");
// ["0000:01:00.0", "0000:02:00.0", "0000:41:00.0", "0000:81:00.0",
"0000:82:00.0"]
```

C++ equivalent: nested `for` loop pushing into a single vector.

<a id="page-142"></a>

<a id="dest-142-0"></a>

[chain — concatenate two iterators](<#dest-142-0>)

```
let critical_gpus = vec!["gpu0", "gpu3"];
let warning_gpus = vec!["gpu1", "gpu5"];
// Process all flagged GPUs, critical first
for gpu in critical_gpus.iter().chain(warning_gpus.iter()) {
    println!("Flagged: {gpu}");
}
```

<a id="dest-142-1658"></a>

[windows and chunks — sliding/fixed-size views over slices](<#dest-142-1658>)

```
let temps = [70, 72, 75, 73, 71, 68, 65];
// windows(3): sliding window of size 3 — detect trends
let rising = temps.windows(3)
    .any(|w| w[0] < w[1] && w[1] < w[2]);
println!("Rising trend detected: {rising}"); // true (70 < 72 < 75)
// chunks(2): fixed-size groups — process in pairs
for pair in temps.chunks(2) {
    println!("Pair: {pair:?}");
}
// Pair: [70, 72]
// Pair: [75, 73]
// Pair: [71, 68]
// Pair: [65]       ← last chunk can be smaller
```

C++ equivalent: manual index arithmetic with `i` and `i+1` / `i+2` .

<a id="page-143"></a>

<a id="dest-143-0"></a>

[fold — accumulate into a single value (replaces std::accumulate)](<#dest-143-0>)

```
let errors = vec![
    ("gpu0", 3u32),
    ("gpu1", 0),
    ("gpu2", 7),
    ("gpu3", 1),
];
// Count total errors and build summary in one pass
let (total, summary) = errors.iter().fold(
    (0u32, String::new()),
    |(count, mut s), (name, errs)| {
        if *errs > 0 {
            s.push_str(&format!("{name}:{errs} "));
        }
        (count + errs, s)
    },
);
println!("Total errors: {total}, details: {summary}");
// Total errors: 11, details: gpu0:3 gpu2:7 gpu3:1
```

<a id="dest-143-3278"></a>

[scan — stateful transform (running total, delta detection)](<#dest-143-3278>)

```
let readings = [100, 105, 103, 110, 108];
// Compute deltas between consecutive readings
let deltas: Vec<i32> = readings.iter()
    .scan(None::<i32>, |prev, &val| {
        let delta = prev.map(|p| val - p);
        *prev = Some(val);
        Some(delta)
    })
    .flatten()  // Remove the initial None
    .collect();
println!("Deltas: {deltas:?}"); // [5, -2, 7, -2]
```

<a id="dest-143-5610"></a>

[Quick reference: C++ loop → Rust iterator](<#dest-143-5610>)

C++ Pattern Rust Iterator Example

```
for (int i = 0; i <
                              .enumerate()           v.iter().enumerate()
v.size(); i++)
```

Parallel iteration with index `.zip()` `a.iter().zip(b.iter())`

`vecs.iter().flat_map(|v|` Nested loop → flat result `.flat_map()`

```
v.iter())
```

<a id="page-144"></a>

C++ Pattern Rust Iterator Example

Concatenate two `.chain()` `a.iter().chain(b.iter())` containers

Sliding window `v[i..i+n]` `.windows(n)` `v.windows(3)`

Process in fixed-size `.chunks(n)` `v.chunks(4)` groups

`std::accumulate` / manual `.fold()` `.fold(init, |acc, x| ...)` accumulator

Running total / delta `.scan()` `.scan(state, |s, x| ...)` tracking

```
while (it != end &&
count < n) { ++it;            .take(n)               .iter().take(5)
++count; }
while (it != end &&                                  .skip_while(|x| x <
                              .skip_while()
!pred(*it)) { ++it; }                               &threshold)
                                                     .iter().any(|x| x >
std::any_of                   .any()
                                                    &limit)
                                                     .iter().all(|x|
std::all_of                   .all()
                                                    x.is_valid())
std::none_of                  !.any()                !iter.any(|x| x.failed())
                                                     .filter(|x| x >
std::count_if                 .filter().count()
                                                    &0).count()
```

`std::min_element` /

`.iter().max()` → `.min()` / `.max()` `std::max_element`

```
Option<&T>
```

`.dedup()` (on `v.dedup()` (in-place on Vec) `std::unique` sorted)

<a id="dest-144-5378"></a>

### [Exercise: Iterator chains](<#dest-144-5378>)

Given sensor data as `Vec<(String, f64)>` (name, temperature), write a single iterator chain that:

1. Filters sensors with temp &gt; 80.0
2. Sorts them by temperature (descending)
3. Formats each as `"{name}: {temp}°C [ALARM]"`
4. Collects into `Vec<String>`

Hint: you’ll need `.collect()` before `.sort_by()` , since sorting requires a `Vec` .

<a id="page-145"></a>

- [ ] Solution (click to expand) [Rust iterators](<#page-145>)
    - The `Iterator` trait is used to implement iteration over user-defined types (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
        - In the example, we’ll implement an iterator for the Fibonacci sequence, which starts with 1, 1, 2, … and the successor is the sum of the previous two numbers
        - The `associated type` in the `Iterator` ( `type Item = u32;` ) defines the output type from our iterator ( `u32` )
        - The `next()` method simply contains the logic for implementing our iterator. In this case, all state information is available in the `Fibonacci` structure
        - We could have implemented another trait called `IntoIterator` to implement the `into_iter()` method for more specialized iterators
        - [▶ Try it in the Rust Playground](<https://play.rust-lang.org/>)

<a id="page-146"></a>

<a id="dest-146-158"></a>

## [Iterator Power Tools](<#dest-146-158>)

<a id="dest-146-930"></a>

### [Iterator Power Tools Reference](<#dest-146-930>)

What you’ll learn: Advanced iterator combinators beyond `filter` / `map` / `collect` —

`enumerate` , `zip` , `chain` , `flat_map` , `scan` , `windows` , and `chunks` . Essential for replacing C-style indexed `for` loops with safe, expressive Rust iterators.

The basic `filter` / `map` / `collect` chain covers many cases, but Rust’s iterator library is far richer. This section covers the tools you’ll reach for daily — especially when translating C loops that manually track indices, accumulate results, or process data in fixed-size chunks.

<a id="dest-146-3158"></a>

[Quick Reference Table](<#dest-146-3158>)

Method C Equivalent What it does Returns

Pairs each element with its index

```
                      for (int i=0;
enumerate()                                                       (usize, T)
                     ...)
```

Parallel arrays with `zip(other)` same index

Pairs elements `(A, B)` from two iterators

Process array1 then `chain(other)` array2

Concatenates two `T` iterators

Maps then Nested loops `flat_map(f)` `U` flattens one level

```
                      for (int i=0;
windows(n)           i<len-n+1; i++)
                     &arr[i..i+n]
```

Overlapping slices `&[T]` of size `n`

Process `n` `chunks(n)` elements at a time

Non-overlapping `&[T]` slices of size `n`

```
                      int acc = init;
fold(init, f)        for (...) acc =
                     f(acc, x);
                                                                  Option<B>
```

Reduce to single `Acc` value

Running `scan(init, f)` accumulator with

Like `fold` but yields

<a id="page-147"></a>

Method C Equivalent What it does Returns

output intermediate results

`take(n)` /

Start loop at offset / limit

First `n` / skip first

```
                                                                  T
skip(n)
                      while (pred)
                                                                  T
skip_while(f)        {...}
```

`n` elements

`take_while(f)` /

Take/skip while predicate holds

Allows `.peek()` without consuming

Lookahead with `peekable()`

```
                                                                  T
                      arr[i+1]
                      for (i=0; i<len;
step_by(n)                                                        T
                     i+=n)
                                                                  (A, B)
```

Take every nth element

Collect pairs into `unzip()` Split parallel arrays two collections

`sum()` /

Accumulate sum/product

Reduce with `+` or

```
                                                                  T
product()                                   *
                                                                  Option<T>
                                            Option<T>
                      bool found =
                     false; for (...)                             bool
                     ...
                      for (i=0; ...) if
position(f)                                                       Option<usize>
                     (pred) return i;
```

Return `min()` / `max()` Find extremes

Short-circuit boolean search

`any(f)` / `all(f)`

Index of first match

<a id="dest-147-4702"></a>

### [enumerate — Index + Value (replaces C index loops)](<#dest-147-4702>)

```
fn main() {
    let sensors = ["GPU_TEMP", "CPU_TEMP", "FAN_RPM", "PSU_WATT"];
    // C style: for (int i = 0; i < 4; i++) printf("[%d] %s\n", i, sensors[i]);
    for (i, name) in sensors.iter().enumerate() {
        println!("[{i}] {name}");
    }
    // Find the index of a specific sensor
    let gpu_idx = sensors.iter().position(|&s| s == "GPU_TEMP");
    println!("GPU sensor at index: {gpu_idx:?}");  // Some(0)
}
```

<a id="page-148"></a>

<a id="dest-148-0"></a>

### [zip — Parallel Iteration (replaces parallel array loops)](<#dest-148-0>)

```
fn main() {
    let names = ["accel_diag", "nic_diag", "cpu_diag"];
    let statuses = [true, false, true];
    let durations_ms = [1200, 850, 3400];
    // C: for (int i=0; i<3; i++) printf("%s: %s (%d ms)\n", names[i], ...);
    for ((name, passed), ms) in names.iter().zip(&statuses).zip(&durations_ms) {
        let status = if *passed { "PASS" } else { "FAIL" };
        println!("{name}: {status} ({ms} ms)");
    }
}
```

<a id="dest-148-2310"></a>

### [chain — Concatenate Iterators](<#dest-148-2310>)

```
fn main() {
    let critical = vec!["ECC error", "Thermal shutdown"];
    let warnings = vec!["Link degraded", "Fan slow"];
    // Process all events in priority order
    let all_events: Vec<_> = critical.iter().chain(warnings.iter()).collect();
    println!("{all_events:?}");
    // ["ECC error", "Thermal shutdown", "Link degraded", "Fan slow"]
}
```

<a id="dest-148-4358"></a>

### [flat\_map — Flatten Nested Results](<#dest-148-4358>)

```
fn main() {
    let lines = vec!["gpu:42:ok", "nic:99:fail", "cpu:7:ok"];
    // Extract all numeric values from colon-separated lines
    let numbers: Vec<u32> = lines.iter()
        .flat_map(|line| line.split(':'))
        .filter_map(|token| token.parse::<u32>().ok())
        .collect();
    println!("{numbers:?}");  // [42, 99, 7]
}
```

<a id="page-149"></a>

<a id="dest-149-0"></a>

### [windows and chunks — Sliding and Fixed-Size Groups](<#dest-149-0>)

```
fn main() {
    let temps = [65, 68, 72, 71, 75, 80, 78, 76];
    // windows(3): overlapping groups of 3 (like a sliding average)
    // C: for (int i = 0; i <= len-3; i++) avg(arr[i], arr[i+1], arr[i+2]);
    let moving_avg: Vec<f64> = temps.windows(3)
        .map(|w| w.iter().sum::<i32>() as f64 / 3.0)
        .collect();
    println!("Moving avg: {moving_avg:.1?}");
    // chunks(2): non-overlapping groups of 2
    // C: for (int i = 0; i < len; i += 2) process(arr[i], arr[i+1]);
    for pair in temps.chunks(2) {
        println!("Chunk: {pair:?}");
    }
    // chunks_exact(2): same but panics if remainder exists
    // Also: .remainder() gives leftover elements
}
```

<a id="dest-149-3390"></a>

### [fold and scan — Accumulation](<#dest-149-3390>)

```
fn main() {
    let values = [10, 20, 30, 40, 50];
    // fold: single final result (like C's accumulator loop)
    let sum = values.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {sum}");  // 150
    // Build a string with fold
    let csv = values.iter()
        .fold(String::new(), |acc, x| {
            if acc.is_empty() { format!("{x}") }
            else { format!("{acc},{x}") }
        });
    println!("CSV: {csv}");  // "10,20,30,40,50"
    // scan: like fold but yields intermediate results
    let running_sum: Vec<i32> = values.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("Running sum: {running_sum:?}");  // [10, 30, 60, 100, 150]
}
```

<a id="page-150"></a>

<a id="dest-150-0"></a>

### [Exercise: Sensor Data Pipeline](<#dest-150-0>)

Given raw sensor readings (one per line, format `"sensor_name:value:unit"` ), write an iterator pipeline that:

1. Parses each line into `(name, f64, unit)`
2. Filters out readings below a threshold
3. Groups by sensor name using `fold` into a `HashMap`
4. Prints the average reading per sensor

```
// Starter code
fn main() {
    let raw_data = vec![
        "gpu_temp:72.5:C",
        "cpu_temp:65.0:C",
        "gpu_temp:74.2:C",
        "fan_rpm:1200.0:RPM",
        "cpu_temp:63.8:C",
        "gpu_temp:80.1:C",
        "fan_rpm:1150.0:RPM",
    ];
    let threshold = 70.0;
    // TODO: Parse, filter values >= threshold, group by name, compute averages
}
```

- [ ] Solution (click to expand) [Rust iterators](<#page-150>)
    - The `Iterator` trait is used to implement iteration over user-defined types (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
        - In the example, we’ll implement an iterator for the Fibonacci sequence, which starts with 1, 1, 2, … and the successor is the sum of the previous two numbers
        - The `associated type` in the `Iterator` ( `type Item = u32;` ) defines the output type from our iterator ( `u32` )
        - The `next()` method simply contains the logic for implementing our iterator. In this case, all state information is available in the `Fibonacci` structure
        - We could have implemented another trait called `IntoIterator` to implement the `into_iter()` method for more specialized iterators
        - https://play.rust-lang.org/? version=stable&amp;mode=debug&amp;edition=2021&amp;gist=ab367dc2611e1b5a0bf98f1185b3 8f3f

<a id="page-151"></a>

<a id="dest-151-158"></a>

## [Rust concurrency](<#dest-151-158>)

What you’ll learn: Rust’s concurrency model — threads, `Send` / `Sync` marker traits,

`Mutex<T>` , `Arc<T>` , channels, and how the compiler prevents data races at compile time. No runtime overhead for thread safety you don’t use.

- Rust has built-in support for concurrency, similar to `std::thread` in C++
    - Key difference: Rust prevents data races at compile time through `Send` and `Sync` marker traits
    - In C++, sharing a `std::vector` across threads without a mutex is UB but compiles fine. In Rust, it won’t compile.
    - `Mutex<T>` in Rust wraps the data, not just the access — you literally cannot read the data without locking
- The `thread::spawn()` can be used to create a separate thread that executes the closure `||` in parallel

```
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });
    for i in 0..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
    handle.join().unwrap(); // The handle.join() ensures that the spawned thread
exits
}
```

<a id="dest-151-6052"></a>

## [Rust concurrency](<#dest-151-6052>)

- `thread::scope()` can be used in cases where it is necessary to borrow from the environment. This works because `thread::scope` waits until the internal thread returns
- Try executing this exercise without `thread::scope` to see the issue

<a id="page-152"></a>

```
use std::thread;
fn main() {
  let a = [0, 1, 2];
  thread::scope(|scope| {
      scope.spawn(|| {
          for x in &a {
            println!("{x}");
          }
      });
  });
}
```

<a id="dest-152-1928"></a>

## [Rust concurrency](<#dest-152-1928>)

- We can also use `move` to transfer ownership to the thread. For `Copy` types like `[i32; 3]` , the `move` keyword copies the data into the closure, and the original remains usable

```
use std::thread;
fn main() {
  let mut a = [0, 1, 2];
  let handle = thread::spawn(move || {
      for x in a {
        println!("{x}");
      }
  });
  a[0] = 42;    // Doesn't affect the copy sent to the thread
  handle.join().unwrap();
}
```

<a id="dest-152-4688"></a>

## [Rust concurrency](<#dest-152-4688>)

- `Arc<T>` can be used to share read-only references between multiple threads
    - `Arc` stands for Atomic Reference Counted. The reference isn’t released until the reference count reaches 0
    - `Arc::clone()` simply increases the reference count without cloning the data

<a id="page-153"></a>

```
use std::sync::Arc;
use std::thread;
fn main() {
    let a = Arc::new([0, 1, 2]);
    let mut handles = Vec::new();
    for i in 0..2 {
        let arc = Arc::clone(&a);
        handles.push(thread::spawn(move || {
            println!("Thread: {i} {arc:?}");
        }));
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
}
```

<a id="dest-153-2062"></a>

## [Rust concurrency](<#dest-153-2062>)

- `Arc<T>` can be combined with `Mutex<T>` to provide mutable references.
    - `Mutex` guards the protected data and ensures that only the thread holding the lock has access.
    - The `MutexGuard` is automatically released when it goes out of scope (RAII). Note: `std::mem::forget` can still leak a guard — so “impossible to forget to unlock” is more accurate than “impossible to leak.”

```
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // MutexGuard dropped here — lock released automatically
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap());
    // Output: Final count: 5
}
```

<a id="page-154"></a>

<a id="dest-154-0"></a>

## [Rust concurrency: RwLock](<#dest-154-0>)

- `RwLock<T>` allows multiple concurrent readers or one exclusive writer — the read/write lock pattern from C++ ( `std::shared_mutex` )
    - Use `RwLock` when reads far outnumber writes (e.g., configuration, caches)
    - Use `Mutex` when read/write frequency is similar or critical sections are short

```
use std::sync::{Arc, RwLock};
use std::thread;
fn main() {
    let config = Arc::new(RwLock::new(String::from("v1.0")));
    let mut handles = Vec::new();
    // Spawn 5 readers — all can run concurrently
    for i in 0..5 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let val = config.read().unwrap();  // Multiple readers OK
            println!("Reader {i}: {val}");
        }));
    }
    // One writer — blocks until all readers finish
    {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let mut val = config.write().unwrap();  // Exclusive access
            *val = String::from("v2.0");
            println!("Writer: updated to {val}");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
```

<a id="dest-154-5685"></a>

## [Rust concurrency: Mutex poisoning](<#dest-154-5685>)

- If a thread panics while holding a `Mutex` or `RwLock` , the lock becomes poisoned
    - Subsequent calls to `.lock()` return `Err(PoisonError)` — the data may be in an inconsistent state
    - You can recover with `.into_inner()` if you’re confident the data is still valid
    - This has no C++ equivalent — `std::mutex` has no poisoning concept; a panicking thread just leaves the lock held

<a id="page-155"></a>

```
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data2 = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut guard = data2.lock().unwrap();
        guard.push(4);
        panic!("oops!");  // Lock is now poisoned
    });
    let _ = handle.join();  // Thread panicked
    // Subsequent lock attempts return Err(PoisonError)
    match data.lock() {
        Ok(guard) => println!("Data: {guard:?}"),
        Err(poisoned) => {
            println!("Lock was poisoned! Recovering...");
            let guard = poisoned.into_inner();  // Access data anyway
            println!("Recovered data: {guard:?}");  // [1, 2, 3, 4] — push
succeeded before panic
        }
    }
}
```

<a id="dest-155-3818"></a>

## [Rust concurrency: Atomics](<#dest-155-3818>)

- For simple counters and flags, `std::sync::atomic` types avoid the overhead of a `Mutex`
    - `AtomicBool` , `AtomicI32` , `AtomicU64` , `AtomicUsize` , etc.
    - Equivalent to C++ `std::atomic<T>` — same memory ordering model ( `Relaxed` , `Acquire` , `Release` , `SeqCst` )

<a id="page-156"></a>

```
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter: {}", counter.load(Ordering::SeqCst));
    // Output: Counter: 10000
}
```

![Source region, PDF page 156](Rust-for-C_CPP-Programmers-images/page-0156-region-0048.png)

> PDF page 156: Complex table retained as a source image.

<a id="dest-156-5258"></a>

## [Rust concurrency: Condvar](<#dest-156-5258>)

- `Condvar` (condition variable) lets a thread sleep until another thread signals that a condition has changed
    - Always paired with a `Mutex` — the pattern is: lock, check condition, wait if not ready, act when ready
    - Equivalent to C++ `std::condition_variable` / `std::condition_variable::wait`
    - Handles spurious wakeups — always re-check the condition in a loop (or use `wait_while` / `wait_until` )

<a id="page-157"></a>

```
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    // Spawn a worker that waits for a signal
    let pair2 = Arc::clone(&pair);
    let worker = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        // wait: sleeps until signaled (always re-check in a loop for spurious
wakeups)
        while !*ready {
            ready = cvar.wait(ready).unwrap();
        }
        println!("Worker: condition met, proceeding!");
    });
    // Main thread does some work, then signals the worker
    thread::sleep(std::time::Duration::from_millis(100));
    {
        let (lock, cvar) = &*pair;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();  // Wake one waiting thread (notify_all() wakes all)
    }
    worker.join().unwrap();
}
```

When to use Condvar vs channels: Use `Condvar` when threads share mutable state and need to wait for a condition on that state (e.g., “buffer not empty”). Use channels ( `mpsc` ) when threads need to pass messages. Channels are generally easier to reason about.

<a id="dest-157-5302"></a>

## [Rust concurrency](<#dest-157-5302>)

- Rust channels can be used to exchange messages between `Sender` and `Receiver`
    - This uses a paradigm called `mpsc` or `Multi-producer, Single-Consumer`
    - Both `send()` and `recv()` can block the thread

<a id="page-158"></a>

```
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel();
    tx.send(10).unwrap();
    tx.send(20).unwrap();
    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());
    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());
}
```

<a id="dest-158-2332"></a>

## [Rust concurrency](<#dest-158-2332>)

- Channels can be combined with threads

```
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();
    for _ in 0..2 {
        let tx2 = tx.clone();
        thread::spawn(move || {
            let thread_id = thread::current().id();
            for i in 0..10 {
                tx2.send(format!("Message {i}")).unwrap();
                println!("{thread_id:?}: sent Message {i}");
            }
            println!("{thread_id:?}: done");
        });
    }
        // Drop the original sender so rx.iter() terminates when all cloned
senders are dropped
    drop(tx);
    thread::sleep(Duration::from_millis(100));
    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}
```

<a id="page-159"></a>

<a id="dest-159-0"></a>

### [Why Rust prevents data races: Send and Sync](<#dest-159-0>)

- Rust uses two marker traits to enforce thread safety at compile time:
    - `Send` : A type is `Send` if it can be safely transferred to another thread
    - `Sync` : A type is `Sync` if it can be safely shared (via `&T` ) between threads
- Most types are automatically `Send + Sync` . Notable exceptions:
    - `Rc<T>` is neither Send nor Sync (use `Arc<T>` for threads)
    - `Cell<T>` and `RefCell<T>` are not Sync (use `Mutex<T>` or `RwLock<T>` )
    - Raw pointers ( `*const T` , `*mut T` ) are neither Send nor Sync
- This is why the compiler stops you from using `Rc<T>` across threads – it literally doesn’t implement `Send`
- `Arc<Mutex<T>>` is the thread-safe equivalent of `Rc<RefCell<T>>`

Intuition (Jon Gjengset): Think of values as toys. Send = you can give your toy away to another child (thread) — transferring ownership is safe. Sync = you can let others play with your toy at the same time — sharing a reference is safe. An `Rc<T>` has a fragile

(non-atomic) reference counter; handing it off or sharing it would corrupt the count, so it is neither `Send` nor `Sync` .

<a id="dest-159-3668"></a>

## [Exercise: Multi-threaded word count](<#dest-159-3668>)

🔴 Challenge — combines threads, Arc, Mutex, and HashMap

    - Given a `Vec<String>` of text lines, spawn one thread per line to count the words in that line
    - Use `Arc<Mutex<HashMap<String, usize>>>` to collect results
    - Print the total word count across all lines
    - Bonus: Try implementing this with channels ( `mpsc` ) instead of shared state
- [ ] Solution (click to expand)
14. <a id="page-160"></a> [14. Unsafe Rust and FFI](<#page-160>)

<a id="dest-160-832"></a>

[Unsafe Rust](<#dest-160-832>)

What you’ll learn: When and how to use `unsafe` — raw pointer dereferencing, FFI (Foreign Function Interface) for calling C from Rust and vice versa, `CString` / `CStr` for string interop, and how to write safe wrappers around unsafe code.

- `unsafe` unlocks access to features that are normally disallowed by the Rust compiler
    - Dereferencing raw pointers
    - Accessing mutable static variables
    - https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
- With great power comes great responsibility
    - `unsafe` tells the compiler “I, the programmer, take responsibility for upholding the invariants that the compiler normally guarantees”
    - Must guarantee no aliased mutable and immutable references, no dangling pointers, no invalid references, …
    - The use of `unsafe` should be limited to the smallest possible scope
    - All code using `unsafe` should have a “safety” comment describing the assumptions

<a id="page-161"></a>

<a id="dest-161-0"></a>

### [Unsafe Rust examples](<#dest-161-0>)

```
unsafe fn harmless() {}
fn main() {
    // Safety: We are calling a harmless unsafe function
    unsafe {
        harmless();
    }
    let a = 42u32;
    let p = &a as *const u32;
    // Safety: p is a valid pointer to a variable that will remain in scope
    unsafe {
        println!("{}", *p);
    }
    // Safety: Not safe; for illustration purposes only
    let dangerous_buffer = 0xb8000 as *mut u32;
    unsafe {
        println!("About to go kaboom!!!");
        *dangerous_buffer = 0; // This will SEGV on most modern machines
    }
}
```

<a id="dest-161-3390"></a>

### [Simple FFI example (Rust library function consumed by C)](<#dest-161-3390>)

<a id="dest-161-4028"></a>

## [FFI Strings: CString and CStr](<#dest-161-4028>)

FFI stands for _Foreign Function Interface_ — the mechanism Rust uses to call functions written in other languages (such as C) and vice versa.

When interfacing with C code, Rust’s `String` and `&str` types (which are UTF-8 without null terminators) aren’t directly compatible with C strings (which are null-terminated byte arrays). Rust provides `CString` (owned) and `CStr` (borrowed) from `std::ffi` for this purpose:

![Source region, PDF page 161](Rust-for-C_CPP-Programmers-images/page-0161-region-0049.png)

> PDF page 161: Complex table retained as a source image.

<a id="page-162"></a>

```
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
fn demo_ffi_strings() {
    // Creating a C-compatible string (adds null terminator)
    let c_string = CString::new("Hello from Rust").expect("CString::new failed");
    let ptr: *const c_char = c_string.as_ptr();
    // Converting a C string back to Rust (unsafe because we trust the pointer)
    // Safety: ptr is valid and null-terminated (we just created it above)
    let back_to_rust: &CStr = unsafe { CStr::from_ptr(ptr) };
    let rust_str: &str = back_to_rust.to_str().expect("Invalid UTF-8");
    println!("{}", rust_str);
}
```

Warning: `CString::new()` will return an error if the input contains interior null bytes ( `\0` ). Always handle the `Result` . You’ll see `CStr` used extensively in the FFI examples below.

- `FFI` methods must be marked with `#[no_mangle]` to ensure that the compiler doesn’t mangle the name
- We’ll compile the crate as a static library

```
#[no_mangle]
pub extern "C" fn add(left: u64, right: u64) -> u64 {
    left + right
}
```

- We’ll compile the following C-code and link it against our static library.

```
#include <stdio.h>
#include <stdint.h>
extern uint64_t add(uint64_t, uint64_t);
int main() {
    printf("Add returned %llu\n", add(21, 21));
}
```

<a id="dest-162-6585"></a>

### [Complex FFI example](<#dest-162-6585>)

- In the following examples, we’ll create a Rust logging interface and expose it to \[PYTHON\] and `C`
- <a id="page-163"></a> We’ll see how the same interface can be used natively from Rust and C
- We will explore the use of tools like `cbindgen` to generate header files for `C`
- We will see how `unsafe` wrappers can act as a bridge to safe Rust code

<a id="dest-163-982"></a>

## [Logger helper functions](<#dest-163-982>)

```
fn create_or_open_log_file(log_file: &str, overwrite: bool) -> Result<File,
String> {
    if overwrite {
        File::create(log_file).map_err(|e| e.to_string())
    } else {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_file)
            .map_err(|e| e.to_string())
    }
}
fn log_to_file(file_handle: &mut File, message: &str) -> Result<(), String> {
    file_handle
        .write_all(message.as_bytes())
        .map_err(|e| e.to_string())
}
```

<a id="page-164"></a>

<a id="dest-164-0"></a>

## [Logger struct](<#dest-164-0>)

```
struct SimpleLogger {
    log_level: LogLevel,
    file_handle: File,
}
impl SimpleLogger {
    fn new(log_file: &str, overwrite: bool, log_level: LogLevel) -> Result<Self,
String> {
        let file_handle = create_or_open_log_file(log_file, overwrite)?;
        Ok(Self {
            file_handle,
            log_level,
        })
    }
    fn log_message(&mut self, log_level: LogLevel, message: &str) -> Result<(),
String> {
        if log_level as u32 <= self.log_level as u32 {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let message = format!("Simple: {timestamp} {log_level} {message}\n");
            log_to_file(&mut self.file_handle, &message)
        } else {
            Ok(())
        }
    }
}
```

<a id="dest-164-4500"></a>

## [Testing](<#dest-164-4500>)

- Testing functionality with Rust is trivial
    - Test methods are decorated with `#[test]` , and aren’t part of the compiled binary
    - It’s easy to create mock methods for testing purposes

```
#[test]
fn testfunc() -> Result<(), String> {
    let mut logger = SimpleLogger::new("test.log", false, LogLevel::INFO)?;
    logger.log_message(LogLevel::TRACELEVEL1, "Hello world")?;
    logger.log_message(LogLevel::CRITICAL, "Critical message")?;
    Ok(()) // The compiler automatically drops logger here
}
```

```
cargo test
```

<a id="page-165"></a>

<a id="dest-165-0"></a>

## [(C)-Rust FFI](<#dest-165-0>)

- cbindgen is a great tool for generating header files for exported Rust functions
    - Can be installed using cargo

```
cargo install cbindgen
cbindgen
```

- Function and structures can be exported using `#[no_mangle]` and `#[repr(C)]`
    - We’ll assume the common interface pattern passing in a `**` to the actual implementation and returning 0 on success and non-zero on error
    - Opaque vs transparent structs: Our `SimpleLogger` is passed as an opaque pointer ( `*mut SimpleLogger` ) — the C side never accesses its fields, so `#[repr(C)]` is not needed. Use `#[repr(C)]` when C code needs to read/write struct fields directly:

```
// Opaque — C only holds a pointer, never inspects fields. No #[repr(C)] needed.
struct SimpleLogger { /* Rust-only fields */ }
// Transparent — C reads/writes fields directly. MUST use #[repr(C)].
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```

```
typedef struct SimpleLogger SimpleLogger;
uint32_t create_simple_logger(const char *file_name, struct SimpleLogger
**out_logger);
uint32_t log_entry(struct SimpleLogger *logger, const char *message);
uint32_t drop_logger(struct SimpleLogger *logger);
```

- Note that we need to a lot of sanity checks
- We have to explicitly leak memory to prevent Rust from automatically deallocating

<a id="page-166"></a>

```
#[no_mangle]
pub extern "C" fn create_simple_logger(file_name: *const std::os::raw::c_char,
out_logger: *mut *mut SimpleLogger) -> u32 {
    use std::ffi::CStr;
    // Make sure pointer isn't NULL
    if file_name.is_null() || out_logger.is_null() {
        return 1;
    }
    // Safety: The passed in pointer is either NULL or 0-terminated by contract
    let file_name = unsafe {
        CStr::from_ptr(file_name)
    };
    let file_name = file_name.to_str();
    // Make sure that file_name doesn't have garbage characters
    if file_name.is_err() {
        return 1;
    }
    let file_name = file_name.unwrap();
    // Assume some defaults; we'll pass them in real life
    let new_logger = SimpleLogger::new(file_name, true, LogLevel::CRITICAL);
    // Check that we were able to construct the logger
    if new_logger.is_err() {
        return 1;
    }
    let new_logger = Box::new(new_logger.unwrap());
    // This prevents the Box from being dropped when if goes out of scope
    let logger_ptr: *mut SimpleLogger = Box::leak(new_logger);
    // Safety: logger is non-null and logger_ptr is valid
    unsafe {
        *out_logger = logger_ptr;
    }
    return 0;
}
```

- We have similar error checks in `log_entry()`

<a id="page-167"></a>

```
#[no_mangle]
pub extern "C" fn log_entry(logger: *mut SimpleLogger, message: *const
std::os::raw::c_char) -> u32 {
    use std::ffi::CStr;
    if message.is_null() || logger.is_null() {
        return 1;
    }
    // Safety: message is non-null
    let message = unsafe {
        CStr::from_ptr(message)
    };
    let message = message.to_str();
    // Make sure that file_name doesn't have garbage characters
    if message.is_err() {
        return 1;
    }
    // Safety: logger is valid pointer previously constructed by
create_simple_logger()
    unsafe {
        (*logger).log_message(LogLevel::CRITICAL, message.unwrap()).is_err() as
u32
    }
}
#[no_mangle]
pub extern "C" fn drop_logger(logger: *mut SimpleLogger) -> u32 {
    if logger.is_null() {
        return 1;
    }
    // Safety: logger is valid pointer previously constructed by
create_simple_logger()
    unsafe {
        // This constructs a Box<SimpleLogger>, which is dropped when it goes out
of scope
        let _ = Box::from_raw(logger);
    }
    0
}
```

- We can test our (C)-FFI using Rust, or by writing a (C)-program

```
#[test]
fn test_c_logger() {
    // The c".." creates a NULL terminated string
    let file_name = c"test.log".as_ptr() as *const std::os::raw::c_char;
    let mut c_logger: *mut SimpleLogger = std::ptr::null_mut();
    assert_eq!(create_simple_logger(file_name, &mut c_logger), 0);
    // This is the manual way to create c"..." strings
    let message = b"message from C\0".as_ptr() as *const std::os::raw::c_char;
    assert_eq!(log_entry(c_logger, message), 0);
    drop_logger(c_logger);
}
```

<a id="page-168"></a>

```
#include "logger.h"
...
int main() {
    SimpleLogger *logger = NULL;
    if (create_simple_logger("test.log", &logger) == 0) {
        log_entry(logger, "Hello from C");
        drop_logger(logger); /*Needed to close handle, etc.*/
    }
    ...
}
```

<a id="dest-168-1950"></a>

### [Ensuring correctness of unsafe code](<#dest-168-1950>)

- The TL;DR version is that using `unsafe` requires deliberate thought
    - Always document the safety assumptions made by the code and review it with experts
    - Use tools like cbindgen, Miri, Valgrind that can help verify correctness
    - Never let a panic unwind across an FFI boundary — this is UB. Use `std::panic::catch_unwind` at FFI entry points, or configure `panic = "abort"` in your profile
    - If a struct is shared across FFI, mark it `#[repr(C)]` to guarantee C-compatible memory layout
    - Consult https://doc.rust-lang.org/nomicon/intro.html (the “Rustonomicon” — the dark arts of unsafe Rust)
    - Seek help of internal experts

<a id="dest-168-4808"></a>

[Verification tools: Miri vs Valgrind](<#dest-168-4808>)

C++ developers are familiar with Valgrind and sanitizers. Rust has those **plus** Miri, which is far more precise for Rust-specific UB:

![Source region, PDF page 168](Rust-for-C_CPP-Programmers-images/page-0168-region-0050.png)

> PDF page 168: Complex table retained as a source image.

<a id="page-169"></a>

![Source region, PDF page 169](Rust-for-C_CPP-Programmers-images/page-0169-region-0051.png)

> PDF page 169: Complex table retained as a source image.

Recommendation: Use both — Miri for pure Rust unsafe, Valgrind for FFI integration:

- Miri — catches Rust-specific UB that Valgrind cannot see (aliasing violations, invalid enum values, stacked borrows):

```
rustup +nightly component add miri
cargo +nightly miri test                    # Run all tests under Miri
cargo +nightly miri test -- test_name       # Run a specific test
```

⚠️ Miri requires nightly and cannot execute FFI calls. Isolate unsafe Rust logic into testable units.

- Valgrind — the tool you already know, works on the compiled binary including FFI:

```
sudo apt install valgrind
cargo install cargo-valgrind
cargo valgrind test                         # Run all tests under Valgrind
```

Catches leaks in `Box::leak` / `Box::from_raw` patterns common in FFI code.

<a id="page-170"></a>

- cargo-careful — runs tests with extra runtime checks enabled (between regular tests and Miri):

```
cargo install cargo-careful
cargo +nightly careful test
```

<a id="dest-170-1410"></a>

### [Unsafe Rust summary](<#dest-170-1410>)

- `cbindgen` is a great tool for (C) FFI to Rust
    - Use `bindgen` for FFI-interfaces in the other direction (consult the extensive documentation)
- Do not assume that your unsafe code is correct, or that it’s fine to use from safe Rust. It’s really easy to make mistakes, and even code that seemingly works correctly can be wrong for subtle reasons
    - Use tools to verify correctness
    - If still in doubt, reach out for expert advice
- Make sure that your `unsafe` code has comments with an explicit documentation about assumptions and why it’s correct
    - Callers of `unsafe` code should have corresponding comments on safety as well, and observe restrictions

<a id="dest-170-4088"></a>

## [Exercise: Writing a safe FFI wrapper](<#dest-170-4088>)

🔴 Challenge — requires understanding unsafe blocks, raw pointers, and safe API design

- Write a safe Rust wrapper around an `unsafe` FFI-style function. The exercise simulates calling a C function that writes a formatted string into a caller-provided buffer.
- Step 1: Implement the unsafe function `unsafe_greet` that writes a greeting into a raw `*mut u8` buffer
- Step 2: Write a safe wrapper `safe_greet` that allocates a `Vec<u8>` , calls the unsafe function, and returns a `String`
- Step 3: Add proper `// Safety:` comments to every unsafe block

#### **Starter code:**

<a id="page-171"></a>

```
use std::fmt::Write as _;
/// Simulates a C function: writes "Hello, <name>!" into buffer.
/// Returns the number of bytes written (excluding null terminator).
/// # Safety
/// - `buf` must point to at least `buf_len` writable bytes
/// - `name` must be a valid pointer to a null-terminated C string
unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
    // TODO: Build greeting, copy bytes into buf, return length
    // Hint: use std::ffi::CStr::from_ptr or iterate bytes manually
    todo!()
}
/// Safe wrapper — no unsafe in the public API
fn safe_greet(name: &str) -> Result<String, String> {
    // TODO: Allocate a Vec<u8> buffer, create a null-terminated name,
    // call unsafe_greet inside an unsafe block with Safety comment,
    // convert the result back to a String
    todo!()
}
fn main() {
    match safe_greet("Rustacean") {
        Ok(msg) => println!("{msg}"),
        Err(e) => eprintln!("Error: {e}"),
    }
    // Expected output: Hello, Rustacean!
}
```

- [ ] Solution (click to expand)

<a id="page-172"></a>

<a id="dest-172-158"></a>

## [no\_std — Rust Without the Standard](<#dest-172-158>) [Library](<#dest-172-158>)

What you’ll learn: How to write Rust for bare-metal and embedded targets using `#!`

`[no_std]` — the `core` and `alloc` crate split, panic handlers, and how this compares to embedded C without `libc` .

If you come from embedded C, you’re already used to working without `libc` or with a minimal runtime. Rust has a first-class equivalent: the \#\!\[no\_std\] attribute.

<a id="dest-172-2722"></a>

### [What is no\_std?](<#dest-172-2722>)

When you add `#![no_std]` to the crate root, the compiler removes the implicit `extern crate` `std;` and links only against core (and optionally alloc ).

![Source region, PDF page 172](Rust-for-C_CPP-Programmers-images/page-0172-region-0052.png)

> PDF page 172: Complex table retained as a source image.

Rule of thumb for embedded devs: if your C project links against `-lc` and uses `malloc` , you can probably use `core` + `alloc` . If it runs on bare metal without `malloc` , stick with `core` only.

<a id="page-173"></a>

<a id="dest-173-0"></a>

## [Declaring no\_std](<#dest-173-0>)

```
// src/lib.rs  (or src/main.rs for a binary with #![no_main])
#![no_std]
// You still get everything in `core`:
use core::fmt;
use core::result::Result;
use core::option::Option;
// If you have an allocator, opt in to heap types:
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
```

For a bare-metal binary you also need `#![no_main]` and a panic handler:

```
#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // hang on panic — replace with your board's reset/LED blink
}
// Entry point depends on your HAL / linker script
```

<a id="dest-173-4672"></a>

## [What you lose (and alternatives)](<#dest-173-4672>)

std feature no\_std alternative

`core::write!` to a UART / `defmt` `println!`

`heapless::FnvIndexMap` (fixed capacity) or `BTreeMap` (with `HashMap`

`alloc` )

`heapless::Vec` (stack-allocated, fixed capacity) `Vec`

`heapless::String` or `&str` `String`

```
std::io::Read/Write      embedded_io::Read/Write
```

Interrupt handlers, RTIC tasks `thread::spawn`

Hardware timer peripherals `std::time`

Flash / EEPROM drivers `std::fs`

<a id="page-174"></a>

<a id="dest-174-0"></a>

### [Notable no\_std crates for embedded](<#dest-174-0>)

Crate Purpose Notes

Fixed-capacity `Vec` , `String` , [`heapless`](<https://crates.io/crates/heapless>)

No allocator needed — all on the stack

`Queue` , `Map`

Like `printf` but deferred Efficient logging over probe/ITM [`defmt`](<https://crates.io/crates/defmt>) formatting on the host

Hardware abstraction traits Implement once, run on any MCU (SPI, I²C, GPIO, UART)

[`embedded-`](<https://crates.io/crates/embedded-hal>)

```
hal
```

ARM Cortex-M intrinsics &amp; Low-level, like CMSIS [`cortex-m`](<https://crates.io/crates/cortex-m>) register access

Runtime / startup code for Replaces your `startup.s` Cortex-M

[`cortex-m-`](<https://crates.io/crates/cortex-m-rt>)

```
rt
```

Real-Time Interrupt-driven [`rtic`](<https://crates.io/crates/rtic>) Concurrency

Compile-time task scheduling, zero overhead

Async executor for embedded `async/await` on bare metal [`embassy`](<https://crates.io/crates/embassy-executor>) `no_std` serde serialization [`postcard`](<https://crates.io/crates/postcard>) (binary)

Replaces `serde_json` when you can’t afford strings

Works in `no_std` since v2; prefer Derive macro for `Error` trait [`thiserror`](<https://crates.io/crates/thiserror>) over `anyhow`

When you need networking `no_std` TCP/IP stack [`smoltcp`](<https://crates.io/crates/smoltcp>) without an OS

<a id="dest-174-4725"></a>

### [C vs Rust: bare-metal comparison](<#dest-174-4725>)

A typical embedded C blinky:

<a id="page-175"></a>

```
// C — bare metal, vendor HAL
#include "stm32f4xx_hal.h"
void SysTick_Handler(void) {
    HAL_GPIO_TogglePin(GPIOA, GPIO_PIN_5);
}
int main(void) {
    HAL_Init();
    __HAL_RCC_GPIOA_CLK_ENABLE();
    GPIO_InitTypeDef gpio = { .Pin = GPIO_PIN_5, .Mode = GPIO_MODE_OUTPUT_PP };
    HAL_GPIO_Init(GPIOA, &gpio);
    HAL_SYSTICK_Config(HAL_RCC_GetHCLKFreq() / 1000);
    while (1) {}
}
```

The Rust equivalent (using `embedded-hal` + a board crate):

```
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_halt as _; // panic handler: infinite loop
use stm32f4xx_hal::{pac, prelude::*};
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delay = dp.TIM2.delay_ms(&clocks);
    loop {
        led.toggle();
        delay.delay_ms(500u32);
    }
}
```

#### **Key differences for C devs:**

- `Peripherals::take()` returns `Option` — ensures the singleton pattern at compile time (no double-init bugs)
- `.split()` moves ownership of individual pins — no risk of two modules driving the same pin
- All register access is type-checked — you can’t accidentally write to a read-only register
- The borrow checker prevents data races between `main` and interrupt handlers (with RTIC)

<a id="page-176"></a>

<a id="dest-176-0"></a>

### [When to use no\_std vs std](<#dest-176-0>)

![Source region, PDF page 176](Rust-for-C_CPP-Programmers-images/page-0176-region-0053.png)

<a id="dest-176-3668"></a>

## [Exercise: no\_std ring buffer](<#dest-176-3668>)

🔴 Challenge — combines generics, `MaybeUninit` , and `#[cfg(test)]` in a `no_std` context

In embedded systems you often need a fixed-size ring buffer (circular buffer) that never allocates. Implement one using only `core` (no `alloc` , no `std` ).

#### **Requirements:**

- Generic over element type `T: Copy`
- Fixed capacity `N` (const generic)
- `push(&mut self, item: T)` — overwrites oldest element when full
- `pop(&mut self) -> Option<T>` — returns oldest element

```
len(&self) -> usize
```

- `is_empty(&self) -> bool`
- Must compile with `#![no_std]`

<a id="page-177"></a>

```
// Starter code
#![no_std]
use core::mem::MaybeUninit;
pub struct RingBuffer<T: Copy, const N: usize> {
    buf: [MaybeUninit<T>; N],
    head: usize,  // next write position
    tail: usize,  // next read position
    count: usize,
}
impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub const fn new() -> Self {
        todo!()
    }
    pub fn push(&mut self, item: T) {
        todo!()
    }
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }
    pub fn len(&self) -> usize {
        todo!()
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
```

- [ ] Solution

<a id="page-178"></a>

<a id="dest-178-158"></a>

## [Embedded Deep Dive](<#dest-178-158>)

<a id="dest-178-930"></a>

## [MMIO and Volatile Register Access](<#dest-178-930>)

What you’ll learn: Type-safe hardware register access in embedded Rust — volatile MMIO patterns, register abstraction crates, and how Rust’s type system can encode register permissions that C’s `volatile` keyword cannot.

In C firmware, you access hardware registers via `volatile` pointers to specific memory addresses. Rust has equivalent mechanisms — but with type safety.

<a id="dest-178-2970"></a>

### [C volatile vs Rust volatile](<#dest-178-2970>)

```
// C — typical MMIO register access
#define GPIO_BASE     0x40020000
#define GPIO_MODER    (*(volatile uint32_t*)(GPIO_BASE + 0x00))
#define GPIO_ODR      (*(volatile uint32_t*)(GPIO_BASE + 0x14))
void toggle_led(void) {
    GPIO_ODR ^= (1 << 5);  // Toggle pin 5
}
```

```
// Rust — raw volatile (low-level, rarely used directly)
use core::ptr;
const GPIO_BASE: usize = 0x4002_0000;
const GPIO_ODR: *mut u32 = (GPIO_BASE + 0x14) as *mut u32;
/// # Safety
/// Caller must ensure GPIO_BASE is a valid mapped peripheral address.
unsafe fn toggle_led() {
    // SAFETY: GPIO_ODR is a valid memory-mapped register address.
    let current = unsafe { ptr::read_volatile(GPIO_ODR) };
    unsafe { ptr::write_volatile(GPIO_ODR, current ^ (1 << 5)) };
}
```

<a id="page-179"></a>

<a id="dest-179-0"></a>

### [svd2rust — Type-Safe Register Access (the Rust way)](<#dest-179-0>)

In practice, you never write raw volatile pointers. Instead, `svd2rust` generates a Peripheral Access Crate (PAC) from the chip’s SVD file (the same XML file used by your IDE’s debug view):

```
// Generated PAC code (you don't write this — svd2rust does)
// The PAC makes invalid register access a compile error
// Usage with PAC:
use stm32f4::stm32f401;  // PAC crate for your chip
fn configure_gpio(dp: stm32f401::Peripherals) {
    // Enable GPIOA clock — type-safe, no magic numbers
    dp.RCC.ahb1enr.modify(|_, w| w.gpioaen().enabled());
    // Set pin 5 to output — can't accidentally write to a read-only field
    dp.GPIOA.moder.modify(|_, w| w.moder5().output());
    // Toggle pin 5 — type-checked field access
    dp.GPIOA.odr.modify(|r, w| {
        // SAFETY: toggling a single bit in a valid register field.
        unsafe { w.bits(r.bits() ^ (1 << 5)) }
    });
}
```

C register access Rust PAC equivalent

```
#define REG (*(volatile
```

PAC crate generated by `svd2rust` `uint32_t*)ADDR)`

![Source region, PDF page 179](Rust-for-C_CPP-Programmers-images/page-0179-region-0054.png)

> PDF page 179: Mathematics retained as a source image.

```
                                       let val =
value = REG;
                                       periph.reg.read().field().bits()
```

Wrong register field → silent UB Compile error — field doesn’t exist

Wrong register width → silent UB Type-checked — u8 vs u16 vs u32

<a id="dest-179-5730"></a>

## [Interrupt Handling and Critical Sections](<#dest-179-5730>)

C firmware uses `__disable_irq()` / `__enable_irq()` and ISR functions with `void` signatures. Rust provides type-safe equivalents.

<a id="page-180"></a>

<a id="dest-180-0"></a>

### [C vs Rust Interrupt Patterns](<#dest-180-0>)

```
// C — traditional interrupt handler
volatile uint32_t tick_count = 0;
void SysTick_Handler(void) {   // Naming convention is critical — get it wrong →
HardFault
    tick_count++;
}
uint32_t get_ticks(void) {
    __disable_irq();
    uint32_t t = tick_count;   // Read inside critical section
    __enable_irq();
    return t;
}
```

```
// Rust — using cortex-m and critical sections
use core::cell::Cell;
use cortex_m::interrupt::{self, Mutex};
// Shared state protected by a critical-section Mutex
static TICK_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
#[cortex_m_rt::exception]     // Attribute ensures correct vector table placement
fn SysTick() {                // Compile error if name doesn't match a valid
exception
    interrupt::free(|cs| {    // cs = critical section token (proof IRQs disabled)
        let count = TICK_COUNT.borrow(cs).get();
        TICK_COUNT.borrow(cs).set(count + 1);
    });
}
fn get_ticks() -> u32 {
    interrupt::free(|cs| TICK_COUNT.borrow(cs).get())
}
```

<a id="dest-180-5550"></a>

### [RTIC — Real-Time Interrupt-driven Concurrency](<#dest-180-5550>)

For complex firmware with multiple interrupt priorities, RTIC (formerly RTFM) provides compile-time task scheduling with zero overhead:

<a id="page-181"></a>

```
#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use stm32f4xx_hal::prelude::*;
    #[shared]
    struct Shared {
        temperature: f32,   // Shared between tasks — RTIC manages locking
    }
    #[local]
    struct Local {
        led: stm32f4xx_hal::gpio::Pin<'A', 5, stm32f4xx_hal::gpio::Output>,
    }
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let dp = cx.device;
        let gpioa = dp.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();
        (Shared { temperature: 25.0 }, Local { led })
    }
    // Hardware task: runs on SysTick interrupt
    #[task(binds = SysTick, shared = [temperature], local = [led])]
    fn tick(mut cx: tick::Context) {
        cx.local.led.toggle();
        cx.shared.temperature.lock(|temp| {
            // RTIC guarantees exclusive access here — no manual locking needed
            *temp += 0.1;
        });
    }
}
```

#### **Why RTIC matters for C firmware devs:**

- The `#[shared]` annotation replaces manual mutex management
- Priority-based preemption is configured at compile time — no runtime overhead
- Deadlock-free by construction (the framework proves it at compile time)
- ISR naming errors are compile errors, not runtime HardFaults

<a id="dest-181-6038"></a>

## [Panic Handler Strategies](<#dest-181-6038>)

In C, when something goes wrong in firmware, you typically reset or blink an LED. Rust’s panic handler gives you structured control:

<a id="page-182"></a>

```
// Strategy 1: Halt (for debugging — attach debugger, inspect state)
use panic_halt as _;  // Infinite loop on panic
// Strategy 2: Reset the MCU
use panic_reset as _;  // Triggers system reset
// Strategy 3: Log via probe (development)
use panic_probe as _;  // Sends panic info over debug probe (with defmt)
// Strategy 4: Log over defmt then halt
use defmt_panic as _;  // Rich panic messages over ITM/RTT
// Strategy 5: Custom handler (production firmware)
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 1. Disable interrupts to prevent further damage
    cortex_m::interrupt::disable();
    // 2. Write panic info to a reserved RAM region (survives reset)
    // SAFETY: PANIC_LOG is a reserved memory region defined in linker script.
    unsafe {
        let log = 0x2000_0000 as *mut [u8; 256];
        // Write truncated panic message
        use core::fmt::Write;
        let mut writer = FixedWriter::new(&mut *log);
        let _ = write!(writer, "{}", info);
    }
    // 3. Trigger watchdog reset (or blink error LED)
    loop {
        cortex_m::asm::wfi();  // Wait for interrupt (low power while halted)
    }
}
```

<a id="dest-182-5325"></a>

## [Linker Scripts and Memory Layout](<#dest-182-5325>)

C firmware devs write linker scripts to define FLASH/RAM regions. Rust embedded uses the same concept via `memory.x` :

<a id="page-183"></a>

```
/* memory.x — placed at crate root, consumed by cortex-m-rt */
MEMORY
{
  /* Adjust for your MCU — these are STM32F401 values */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 96K
}
/* Optional: reserve space for panic log (see panic handler above) */
_panic_log_start = ORIGIN(RAM);
_panic_log_size  = 256;
```

```
# .cargo/config.toml — set the target and linker flags
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32F401RE"  # flash and run via debug probe
rustflags = [
    "-C", "link-arg=-Tlink.x",              # cortex-m-rt linker script
]
[build]
target = "thumbv7em-none-eabihf"            # Cortex-M4F with hardware FPU
```

C linker script Rust equivalent

`memory.x` at crate root `MEMORY { FLASH ..., RAM ... }`

```
__attribute__((section(".data")))      #[link_section = ".data"]
```

`-C link-arg=-Tlink.x` in `-T linker.ld` in Makefile

```
.cargo/config.toml
```

`__bss_start__` , `__bss_end__` Handled by `cortex-m-rt` automatically

Startup assembly ( `startup.s` ) `cortex-m-rt` `#[entry]` macro

<a id="dest-183-5168"></a>

## [Writing embedded-hal Drivers](<#dest-183-5168>)

The `embedded-hal` crate defines traits for SPI, I2C, GPIO, UART, etc. Drivers written against these traits work on any MCU — this is Rust’s killer feature for embedded reuse.

<a id="page-184"></a>

<a id="dest-184-0"></a>

### [C vs Rust: A Temperature Sensor Driver](<#dest-184-0>)

```
// C — driver tightly coupled to STM32 HAL
#include "stm32f4xx_hal.h"
float read_temperature(I2C_HandleTypeDef* hi2c, uint8_t addr) {
    uint8_t buf[2];
    HAL_I2C_Mem_Read(hi2c, addr << 1, 0x00, I2C_MEMADD_SIZE_8BIT,
                     buf, 2, HAL_MAX_DELAY);
    int16_t raw = ((int16_t)buf[0] << 4) | (buf[1] >> 4);
    return raw * 0.0625;
}
// Problem: This driver ONLY works with STM32 HAL. Porting to Nordic = rewrite.
```

```
// Rust — driver works on ANY MCU that implements embedded-hal
use embedded_hal::i2c::I2c;
pub struct Tmp102<I2C> {
    i2c: I2C,
    address: u8,
}
impl<I2C: I2c> Tmp102<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }
    pub fn read_temperature(&mut self) -> Result<f32, I2C::Error> {
        let mut buf = [0u8; 2];
        self.i2c.write_read(self.address, &[0x00], &mut buf)?;
        let raw = ((buf[0] as i16) << 4) | ((buf[1] as i16) >> 4);
        Ok(raw as f32 * 0.0625)
    }
}
// Works on STM32, Nordic nRF, ESP32, RP2040 — any chip with an embedded-hal I2C
impl
```

![Source region, PDF page 184](Rust-for-C_CPP-Programmers-images/page-0184-region-0055.png)

<a id="page-185"></a>

<a id="dest-185-0"></a>

## [Global Allocator Setup](<#dest-185-0>)

The `alloc` crate gives you `Vec` , `String` , `Box` — but you need to tell Rust where heap memory comes from. This is the equivalent of implementing `malloc()` for your platform:

```
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use embedded_alloc::LlffHeap as Heap;
#[global_allocator]
static HEAP: Heap = Heap::empty();
#[cortex_m_rt::entry]
fn main() -> ! {
    // Initialize the allocator with a memory region
    // (typically a portion of RAM not used by stack or static data)
    {
        const HEAP_SIZE: usize = 4096;
        static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
        // SAFETY: HEAP_MEM is only accessed here during init, before any
allocation.
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    // Now you can use heap types!
    let mut log_buffer: Vec<u8> = Vec::with_capacity(256);
    let name: String = String::from("sensor_01");
    // ...
    loop {}
}
```

![Source region, PDF page 185](Rust-for-C_CPP-Programmers-images/page-0185-region-0056.png)

> PDF page 185: Complex table retained as a source image.

<a id="dest-185-6608"></a>

## [Mixed no\_std + std Workspaces](<#dest-185-6608>)

Real projects (like a large Rust workspace) often have:

<a id="page-186"></a>

- `no_std` library crates for hardware-portable logic
- `std` binary crates for the Linux application layer

```
workspace_root/
├── Cargo.toml              # [workspace] members = [...]
├── protocol/               # no_std — wire protocol, parsing
│   ├── Cargo.toml          # no default-features, no std
│   └── src/lib.rs          # #![no_std]
├── driver/                 # no_std — hardware abstraction
│   ├── Cargo.toml
│   └── src/lib.rs          # #![no_std], uses embedded-hal traits
├── firmware/               # no_std — MCU binary
│   ├── Cargo.toml          # depends on protocol, driver
│   └── src/main.rs         # #![no_std] #![no_main]
└── host_tool/              # std — Linux CLI tool
    ├── Cargo.toml          # depends on protocol (same crate!)
    └── src/main.rs         # Uses std::fs, std::net, etc.
```

The key pattern: the `protocol` crate uses `#![no_std]` so it compiles for both the MCU

firmware and the Linux host tool. Shared code, zero duplication.

```
# protocol/Cargo.toml
[package]
name = "protocol"
[features]
default = []
std = []  # Optional: enable std-specific features when building for host
[dependencies]
serde = { version = "1", default-features = false, features = ["derive"] }
# Note: default-features = false drops serde's std dependency
```

<a id="page-187"></a>

```
// protocol/src/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
extern crate std;
extern crate alloc;
use alloc::vec::Vec;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct DiagPacket {
    pub sensor_id: u16,
    pub value: i32,
    pub fault_code: u16,
}
// This function works in both no_std and std contexts
pub fn parse_packet(data: &[u8]) -> Result<DiagPacket, &'static str> {
    if data.len() < 8 {
        return Err("packet too short");
    }
    Ok(DiagPacket {
        sensor_id: u16::from_le_bytes([data[0], data[1]]),
        value: i32::from_le_bytes([data[2], data[3], data[4], data[5]]),
        fault_code: u16::from_le_bytes([data[6], data[7]]),
    })
}
```

<a id="dest-187-4380"></a>

## [Exercise: Hardware Abstraction Layer Driver](<#dest-187-4380>)

Write a `no_std` driver for a hypothetical LED controller that communicates over SPI. The driver should be generic over any SPI implementation using `embedded-hal` .

#### **Requirements:**

1. Define a `LedController<SPI>` struct
2. Implement `new()` , `set_brightness(led: u8, brightness: u8)` , and `all_off()`
3. SPI protocol: send `[led_index, brightness_value]` as 2-byte transaction
4. Write tests using a mock SPI implementation

<a id="page-188"></a>

```
// Starter code
#![no_std]
use embedded_hal::spi::SpiDevice;
pub struct LedController<SPI> {
    spi: SPI,
    num_leds: u8,
}
// TODO: Implement new(), set_brightness(), all_off()
// TODO: Create MockSpi for testing
```

- [ ] Solution (click to expand)

<a id="dest-188-2370"></a>

## [Debugging Embedded Rust — probe-rs, defmt, and VS Code](<#dest-188-2370>)

C firmware developers typically debug with OpenOCD + GDB or vendor-specific IDEs (Keil, IAR, Segger Ozone). Rust’s embedded ecosystem has converged on **probe-rs** as the unified debug probe interface, replacing the OpenOCD + GDB stack with a single, Rust-native tool.

<a id="dest-188-3638"></a>

### [probe-rs — The All-in-One Debug Probe Tool](<#dest-188-3638>)

`probe-rs` replaces the OpenOCD + GDB combination. It supports CMSIS-DAP, ST-Link, J-Link, and other debug probes out of the box:

```
# Install probe-rs (includes cargo-flash and cargo-embed)
cargo install probe-rs-tools
# Flash and run your firmware
cargo flash --chip STM32F401RE --release
# Flash, run, and open RTT (Real-Time Transfer) console
cargo embed --chip STM32F401RE
```

probe-rs vs OpenOCD + GDB:

![Source region, PDF page 188](Rust-for-C_CPP-Programmers-images/page-0188-region-0057.png)

> PDF page 188: Complex table retained as a source image.

<a id="page-189"></a>

![Source region, PDF page 189](Rust-for-C_CPP-Programmers-images/page-0189-region-0058.png)

> PDF page 189: Complex table retained as a source image.

<a id="dest-189-1005"></a>

### [Embed.toml — Project Configuration](<#dest-189-1005>)

Instead of juggling `.cfg` and `.gdbinit` files, probe-rs uses a single config:

```
# Embed.toml — placed in your project root
[default.general]
chip = "STM32F401RETx"
[default.rtt]
enabled = true           # Enable Real-Time Transfer console
channels = [
    { up = 0, mode = "BlockIfFull", name = "Terminal" },
]
[default.flashing]
enabled = true           # Flash before running
restore_unwritten_bytes = false
[default.reset]
halt_afterwards = false  # Start running after flash + reset
[default.gdb]
enabled = false          # Set true to expose GDB server on :1337
gdb_connection_string = "127.0.0.1:1337"
```

```
# With Embed.toml, just run:
cargo embed              # Flash + RTT console — zero flags needed
cargo embed --release    # Release build
```

<a id="dest-189-5512"></a>

### [defmt — Deferred Formatting for Embedded Logging](<#dest-189-5512>)

`defmt` (deferred formatting) replaces `printf` debugging. Format strings are stored in the ELF file, not in flash — so log calls on the target send only an index + argument bytes. This makes logging 10–100× faster than `printf` and uses a fraction of the flash space:

<a id="page-190"></a>

```
#![no_std]
#![no_main]
use defmt::{info, warn, error, debug, trace};
use defmt_rtt as _; // RTT transport — links the defmt output to probe-rs
#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Boot complete, firmware v{}", env!("CARGO_PKG_VERSION"));
    let sensor_id: u16 = 0x4A;
    let temperature: f32 = 23.5;
    // Format strings stay in ELF, not flash — near-zero overhead
    debug!("Sensor {:#06X}: {:.1}°C", sensor_id, temperature);
    if temperature > 80.0 {
        warn!("Overtemp on sensor {:#06X}: {:.1}°C", sensor_id, temperature);
    }
    loop {
        cortex_m::asm::wfi(); // Wait for interrupt
    }
}
// Custom types — derive defmt::Format instead of Debug
#[derive(defmt::Format)]
struct SensorReading {
    id: u16,
    value: i32,
    status: SensorStatus,
}
#[derive(defmt::Format)]
enum SensorStatus {
    Ok,
    Warning,
    Fault(u8),
}
// Usage:
// info!("Reading: {:?}", reading);  // <-- uses defmt::Format, NOT std Debug
```

#### **defmt vs** printf **vs** log :

![Source region, PDF page 190](Rust-for-C_CPP-Programmers-images/page-0190-region-0059.png)

> PDF page 190: Complex table retained as a source image.

<a id="page-191"></a>

![Source region, PDF page 191](Rust-for-C_CPP-Programmers-images/page-0191-region-0060.png)

> PDF page 191: Complex table retained as a source image.

<a id="dest-191-2265"></a>

### [VS Code Debug Configuration](<#dest-191-2265>)

With the `probe-rs` VS Code extension, you get full graphical debugging — breakpoints, variable inspection, call stack, and register view:

```
// .vscode/launch.json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "Flash & Debug (probe-rs)",
            "chip": "STM32F401RETx",
            "coreConfigs": [
                {
                    "programBinary": "target/thumbv7em-none-
eabihf/debug/${workspaceFolderBasename}",
                    "rttEnabled": true,
                    "rttChannelFormats": [
                        {
                            "channelNumber": 0,
                            "dataFormat": "Defmt",
                            "showTimestamps": true
                        }
                    ]
                }
            ],
            "connectUnderReset": true,
            "speed": 4000
        }
    ]
}
```

Install the extension:

<a id="page-192"></a>

10/25/26, 2:04 AM Rust for C/C++ Programmers

```
ext install probe-rs.probe-rs-debugger
```

https://microsoft.github.io/RustTraining/c-cpp-book/print.html 192/289

<a id="page-193"></a>

10/25/26, 2:04 AM Rust for C/C++ Programmers

![Source region, PDF page 193](Rust-for-C_CPP-Programmers-images/page-0193-region-0061.png)

https://microsoft.github.io/RustTraining/c-cpp-book/print.html 193/289

<a id="page-194"></a>

![Source region, PDF page 194](Rust-for-C_CPP-Programmers-images/page-0194-region-0062.png)

![Source region, PDF page 194](Rust-for-C_CPP-Programmers-images/page-0194-region-0063.png)

<a id="page-195"></a>

![Source region, PDF page 195](Rust-for-C_CPP-Programmers-images/page-0195-region-0064.png)

![Source region, PDF page 195](Rust-for-C_CPP-Programmers-images/page-0195-region-0065.png)

C Debug Action Rust Equivalent

`probe-rs info` (auto-detects probe + `openocd -f board/st_nucleo_f4.cfg` chip)

```
arm-none-eabi-gdb -x .gdbinit              probe-rs gdb --chip STM32F401RE
```

GDB connects to `localhost:1337` `target remote :3333`

```
monitor reset halt                         probe-rs reset --chip ...
load firmware.elf                          cargo flash --chip ...
```

`printf("debug: %d\n", val)` `defmt::info!("debug: {}", val)` (RTT) (semihosting)

Keil/IAR GUI debugger VS Code + `probe-rs-debugger` extension

Segger SystemView `defmt` + `probe-rs` RTT viewer

Cross-reference: For advanced unsafe patterns used in embedded drivers (pin projections, custom arena/slab allocators), see the companion Rust Patterns guide, sections “Pin Projections — Structural Pinning” and “Custom Allocators — Arena and Slab Patterns.”

<a id="page-196"></a>

<a id="dest-196-158"></a>

## [Case Study Overview: C++ to Rust](<#dest-196-158>) [Translation](<#dest-196-158>)

**What you’ll learn:** Lessons from a real-world translation of ~100K lines of C++ to ~90K lines of Rust across ~20 crates. Five key transformation patterns and the architectural decisions behind them.

- We translated a large C++ diagnostic system (~100K lines of C++) into a Rust implementation (~20 Rust crates, ~90K lines)
- This section shows the actual patterns used — not toy examples, but real production code
- The five key transformations:

\# C++ Pattern Rust Pattern Impact

Class hierarchy + 1

Enum dispatch +

~400 → 0 dynamic\_casts

```
dynamic_cast                     match
```

`shared_ptr` / 2

No reference Arena + index linkage `enable_shared_from_this` tree cycles

`Framework*` raw pointer in 3 every module

`DiagContext<'a>` with lifetime borrowing

Compile-time validity

Composable state 4 God object structs

Testable, modular

Trait objects only where needed (~25 uses)

Static dispatch default

`vector<unique_ptr<Base>>` 5 everywhere

<a id="dest-196-5535"></a>

[Before and After Metrics](<#dest-196-5535>)

![Source region, PDF page 196](Rust-for-C_CPP-Programmers-images/page-0196-region-0066.png)

> PDF page 196: Complex table retained as a source image.

<a id="page-197"></a>

![Source region, PDF page 197](Rust-for-C_CPP-Programmers-images/page-0197-region-0067.png)

> PDF page 197: Complex table retained as a source image.

<a id="dest-197-892"></a>

## [Case Study 1: Inheritance hierarchy → Enum](<#dest-197-892>) [dispatch](<#dest-197-892>)

<a id="dest-197-1988"></a>

## [The C++ Pattern: Event Class Hierarchy](<#dest-197-1988>)

```
// C++ original: Every GPU event type is a class inheriting from GpuEventBase
class GpuEventBase {
public:
    virtual ~GpuEventBase() = default;
    virtual void Process(DiagFramework* fw) = 0;
    uint16_t m_recordId;
    uint8_t  m_sensorType;
    // ... common fields
};
class GpuPcieDegradeEvent : public GpuEventBase {
public:
    void Process(DiagFramework* fw) override;
    uint8_t m_linkSpeed;
    uint8_t m_linkWidth;
};
class GpuPcieFatalEvent : public GpuEventBase { /* ... */ };
class GpuBootEvent : public GpuEventBase { /* ... */ };
// ... 10+ event classes inheriting from GpuEventBase
// Processing requires dynamic_cast:
void ProcessEvents(std::vector<std::unique_ptr<GpuEventBase>>& events,
                   DiagFramework* fw) {
    for (auto& event : events) {
        if (auto* degrade = dynamic_cast<GpuPcieDegradeEvent*>(event.get())) {
            // handle degrade...
        } else if (auto* fatal = dynamic_cast<GpuPcieFatalEvent*>(event.get())) {
            // handle fatal...
        }
        // ... 10 more branches
    }
}
```

<a id="page-198"></a>

<a id="dest-198-0"></a>

## [The Rust Solution: Enum Dispatch](<#dest-198-0>)

```
// Example: types.rs — No inheritance, no vtable, no dynamic_cast
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpuEventKind {
    PcieDegrade,
    PcieFatal,
    PcieUncorr,
    Boot,
    BaseboardState,
    EccError,
    OverTemp,
    PowerRail,
    ErotStatus,
    Unknown,
}
```

```
// Example: manager.rs — Separate typed Vecs, no downcasting needed
pub struct GpuEventManager {
    sku: SkuVariant,
    degrade_events: Vec<GpuPcieDegradeEvent>,   // Concrete type, not Box<dyn>
    fatal_events: Vec<GpuPcieFatalEvent>,
    uncorr_events: Vec<GpuPcieUncorrEvent>,
    boot_events: Vec<GpuBootEvent>,
    baseboard_events: Vec<GpuBaseboardEvent>,
    ecc_events: Vec<GpuEccEvent>,
    // ... each event type gets its own Vec
}
// Accessors return typed slices — zero ambiguity
impl GpuEventManager {
    pub fn degrade_events(&self) -> &[GpuPcieDegradeEvent] {
        &self.degrade_events
    }
    pub fn fatal_events(&self) -> &[GpuPcieFatalEvent] {
        &self.fatal_events
    }
}
```

<a id="dest-198-5888"></a>

### [Why Not Vec&lt;Box&lt;dyn GpuEvent&gt;&gt;?](<#dest-198-5888>)

- The Wrong Approach (literal translation): Put all events in one heterogeneous collection, then downcast — this is what C++ does with `vector<unique_ptr<Base>>`
- The Right Approach: Separate typed Vecs eliminate all downcasting. Each consumer asks for exactly the event type it needs
- Performance: Separate Vecs give better cache locality (all degrade events are contiguous in memory)

<a id="page-199"></a>

<a id="dest-199-172"></a>

## [Case Study 2: shared\_ptr tree → Arena/index](<#dest-199-172>) [pattern](<#dest-199-172>)

<a id="dest-199-1268"></a>

## [The C++ Pattern: Reference-Counted Tree](<#dest-199-1268>)

```
// C++ topology library: PcieDevice uses enable_shared_from_this
// because parent and child nodes both need to reference each other
class PcieDevice : public std::enable_shared_from_this<PcieDevice> {
public:
    std::shared_ptr<PcieDevice> m_upstream;
    std::vector<std::shared_ptr<PcieDevice>> m_downstream;
    // ... device data
    void AddChild(std::shared_ptr<PcieDevice> child) {
        child->m_upstream = shared_from_this();  // Parent ↔ child cycle!
        m_downstream.push_back(child);
    }
};
// Problem: parent→child and child→parent create reference cycles
// Need weak_ptr to break cycles, but easy to forget
```

<a id="page-200"></a>

<a id="dest-200-0"></a>

## [The Rust Solution: Arena with Index Linkage](<#dest-200-0>)

```
// Example: components.rs — Flat Vec owns all devices
pub struct PcieDevice {
    pub base: PcieDeviceBase,
    pub kind: PcieDeviceKind,
    // Tree linkage via indices — no reference counting, no cycles
    pub upstream_idx: Option<usize>,      // Index into the arena Vec
    pub downstream_idxs: Vec<usize>,      // Indices into the arena Vec
}
// The "arena" is simply a Vec<PcieDevice> owned by the tree:
pub struct DeviceTree {
    devices: Vec<PcieDevice>,  // Flat ownership — one Vec owns everything
}
impl DeviceTree {
    pub fn parent(&self, device_idx: usize) -> Option<&PcieDevice> {
        self.devices[device_idx].upstream_idx
            .map(|idx| &self.devices[idx])
    }
    pub fn children(&self, device_idx: usize) -> Vec<&PcieDevice> {
        self.devices[device_idx].downstream_idxs
            .iter()
            .map(|&idx| &self.devices[idx])
            .collect()
    }
}
```

<a id="dest-200-4672"></a>

### [Key Insight](<#dest-200-4672>)

- No shared\_ptr , no weak\_ptr , no enable\_shared\_from\_this
- No reference cycles possible — indices are just `usize` values
- Better cache performance — all devices in contiguous memory
- Simpler reasoning — one owner (the Vec), many viewers (indices)

<a id="page-201"></a>

![Source region, PDF page 201](Rust-for-C_CPP-Programmers-images/page-0201-region-0068.png)

<a id="page-202"></a>

<a id="dest-202-158"></a>

## [Case Study 3: Framework communication →](<#dest-202-158>) [Lifetime borrowing](<#dest-202-158>)

**What you’ll learn:** How to convert C++ raw-pointer framework communication patterns to Rust’s lifetime-based borrowing system, eliminating dangling pointer risks while maintaining zero-cost abstractions.

<a id="dest-202-2190"></a>

## [The C++ Pattern: Raw Pointer to Framework](<#dest-202-2190>)

```
// C++ original: Every diagnostic module stores a raw pointer to the framework
class DiagBase {
protected:
    DiagFramework* m_pFramework;  // Raw pointer — who owns this?
public:
    DiagBase(DiagFramework* fw) : m_pFramework(fw) {}
    void LogEvent(uint32_t code, const std::string& msg) {
        m_pFramework->GetEventLog()->Record(code, msg);  // Hope it's still alive!
    }
};
// Problem: m_pFramework is a raw pointer with no lifetime guarantee
// If framework is destroyed while modules still reference it → UB
```

<a id="page-203"></a>

<a id="dest-203-0"></a>

## [The Rust Solution: DiagContext with Lifetime Borrowing](<#dest-203-0>)

```
// Example: module.rs — Borrow, don't store
/// Context passed to diagnostic modules during execution.
/// The lifetime 'a guarantees the framework outlives the context.
pub struct DiagContext<'a> {
    pub der_log: &'a mut EventLogManager,
    pub config: &'a ModuleConfig,
    pub framework_opts: &'a HashMap<String, String>,
}
/// Modules receive context as a parameter — never store framework pointers
pub trait DiagModule {
    fn id(&self) -> &str;
    fn execute(&mut self, ctx: &mut DiagContext) -> DiagResult<()>;
    fn pre_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> {
        Ok(())
    }
    fn post_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> {
        Ok(())
    }
}
```

<a id="dest-203-3728"></a>

### [Key Insight](<#dest-203-3728>)

- C++ modules store a pointer to the framework (danger: what if the framework is destroyed first?)
- Rust modules receive a context as a function parameter — the borrow checker guarantees the framework is alive during the call
- No raw pointers, no lifetime ambiguity, no “hope it’s still alive”

<a id="page-204"></a>

<a id="dest-204-0"></a>

## [Case Study 4: God object → Composable](<#dest-204-0>) [state](<#dest-204-0>)

<a id="dest-204-1095"></a>

## [The C++ Pattern: Monolithic Framework Class](<#dest-204-1095>)

```
// C++ original: The framework is god object
class DiagFramework {
    // Health-monitor trap processing
    std::vector<AlertTriggerInfo> m_alertTriggers;
    std::vector<WarnTriggerInfo> m_warnTriggers;
    bool m_healthMonHasBootTimeError;
    uint32_t m_healthMonActionCounter;
    // GPU diagnostics
    std::map<uint32_t, GpuPcieInfo> m_gpuPcieMap;
    bool m_isRecoveryContext;
    bool m_healthcheckDetectedDevices;
    // ... 30+ more GPU-related fields
    // PCIe tree
    std::shared_ptr<CPcieTreeLinux> m_pPcieTree;
    // Event logging
    CEventLogMgr* m_pEventLogMgr;
    // ... several other methods
    void HandleGpuEvents();
    void HandleNicEvents();
    void RunGpuDiag();
    // Everything depends on everything
};
```

<a id="page-205"></a>

<a id="dest-205-0"></a>

## [The Rust Solution: Composable State Structs](<#dest-205-0>)

```
// Example: main.rs — State decomposed into focused structs
#[derive(Default)]
struct HealthMonitorState {
    alert_triggers: Vec<AlertTriggerInfo>,
    warn_triggers: Vec<WarnTriggerInfo>,
    health_monitor_action_counter: u32,
    health_monitor_has_boot_time_error: bool,
    // Only health-monitor-related fields
}
#[derive(Default)]
struct GpuDiagState {
    gpu_pcie_map: HashMap<u32, GpuPcieInfo>,
    is_recovery_context: bool,
    healthcheck_detected_devices: bool,
    // Only GPU-related fields
}
/// The framework composes these states rather than owning everything flat
struct DiagFramework {
    ctx: DiagContext,             // Execution context
    args: Args,                   // CLI arguments
    pcie_tree: Option<DeviceTree>,  // No shared_ptr needed
    event_log_mgr: EventLogManager,   // Owned, not raw pointer
    fc_manager: FcManager,        // Fault code management
    health: HealthMonitorState,   // Health-monitor state — its own struct
    gpu: GpuDiagState,           // GPU state — its own struct
}
```

<a id="dest-205-4808"></a>

### [Key Insight](<#dest-205-4808>)

- Testability: Each state struct can be unit-tested independently
- Readability: `self.health.alert_triggers` vs `m_alertTriggers` — clear ownership
- Fearless refactoring: Changing `GpuDiagState` can’t accidentally affect health-monitor processing
- No method soup: Functions that only need health-monitor state take `&mut` `HealthMonitorState` , not the entire framework

<a id="page-206"></a>

<a id="dest-206-0"></a>

## [Case Study 5: Trait objects — when they ARE](<#dest-206-0>) [right](<#dest-206-0>)

- Not everything should be an enum\! The diagnostic module plugin system is a genuine use case for trait objects
- Why? Because diagnostic modules are open for extension — new modules can be added without modifying the framework

```
// Example: framework.rs — Vec<Box<dyn DiagModule>> is correct here
pub struct DiagFramework {
    modules: Vec<Box<dyn DiagModule>>,        // Runtime polymorphism
    pre_diag_modules: Vec<Box<dyn DiagModule>>,
    event_log_mgr: EventLogManager,
    // ...
}
impl DiagFramework {
    /// Register a diagnostic module — any type implementing DiagModule
    pub fn register_module(&mut self, module: Box<dyn DiagModule>) {
        info!("Registering module: {}", module.id());
        self.modules.push(module);
    }
}
```

<a id="dest-206-4148"></a>

### [When to Use Each Pattern](<#dest-206-4148>)

Use Case Pattern Why

Fixed set of variants known at compile time

Exhaustive checking, no `enum` + `match` vtable

Hardware event types (Degrade, Fatal, Boot, …)

All variants known, performance matters

```
enum
GpuEventKind
```

PCIe device types (GPU, NIC, Switch, …)

Fixed set, each variant has different data

`enum`

```
PcieDeviceKind
```

Plugin/module system (open for extension)

New modules added without `Box<dyn Trait>` modifying framework

Test mocking `Box<dyn Trait>` Inject test doubles

<a id="dest-206-6795"></a>

### [Exercise: Think Before You Translate](<#dest-206-6795>)

Given this C++ code:

<a id="page-207"></a>

```
class Shape { public: virtual double area() = 0; };
class Circle : public Shape { double r; double area() override { return 3.14*r*r;
} };
class Rect : public Shape { double w, h; double area() override { return w*h; } };
std::vector<std::unique_ptr<Shape>> shapes;
```

Question: Should the Rust translation use `enum Shape` or `Vec<Box<dyn Shape>>` ?

- [ ] Solution (click to expand) [Translation metrics and lessons learned](<#page-207>)

<a id="dest-207-2415"></a>

## [What We Learned](<#dest-207-2415>)

1. Default to enum dispatch — In ~100K lines of C++, only ~25 uses of `Box<dyn Trait>` were genuinely needed (plugin systems, test mocks). The other ~900 virtual methods became enums with match
2. Arena pattern eliminates reference cycles — `shared_ptr` and `enable_shared_from_this` are symptoms of unclear ownership. Think about who owns the data first
3. Pass context, don’t store pointers — Lifetime-bounded `DiagContext<'a>` is safer and clearer than storing `Framework*` in every module
4. Decompose god objects — If a struct has 30+ fields, it’s probably 3-4 structs wearing a trenchcoat
5. The compiler is your pair programmer — ~400 `dynamic_cast` calls meant ~400 potential runtime failures. Zero `dynamic_cast` equivalents in Rust means zero runtime type errors

<a id="dest-207-5580"></a>

## [The Hardest Parts](<#dest-207-5580>)

- Lifetime annotations: Getting borrows right takes time when you’re used to raw pointers — but once it compiles, it’s correct
- Fighting the borrow checker: Wanting `&mut self` in two places at once. Solution: decompose state into separate structs
- Resisting literal translation: The temptation to write `Vec<Box<dyn Base>>` everywhere. Ask: “Is this set of variants closed?” → If yes, use enum

<a id="page-208"></a>

<a id="dest-208-0"></a>

### [Recommendation for C++ Teams](<#dest-208-0>)

1. Start with a small, self-contained module (not the god object)
2. Translate data structures first, then behavior
3. Let the compiler guide you — its error messages are excellent
4. Reach for `enum` before `dyn Trait`
5. Use the [Rust playground](<https://play.rust-lang.org/>) to prototype patterns before integrating

<a id="page-209"></a>

<a id="dest-209-158"></a>

## [Rust Best Practices Summary](<#dest-209-158>)

**What you’ll learn:** Practical guidelines for writing idiomatic Rust — code organization, naming conventions, error handling patterns, and documentation. A quick-reference chapter you’ll return to often.

<a id="dest-209-1868"></a>

## [Code Organization](<#dest-209-1868>)

- Prefer small functions: Easy to test and reason about
- Use descriptive names: `calculate_total_price()` vs `calc()`
- Group related functionality: Use modules and separate files
- Write documentation: Use `///` for public APIs

<a id="dest-209-3428"></a>

## [Error Handling](<#dest-209-3428>)

- Avoid unwrap() unless infallible: Only use when you’re 100% certain it won’t panic

```
// Bad: Can panic
let value = some_option.unwrap();
// Good: Handle the None case
let value = some_option.unwrap_or(default_value);
let value = some_option.unwrap_or_else(|| expensive_computation());
let value = some_option.unwrap_or_default(); // Uses Default trait
// For Result<T, E>
let value = some_result.unwrap_or(fallback_value);
let value = some_result.unwrap_or_else(|err| {
    eprintln!("Error occurred: {err}");
    default_value
});
```

- Use expect() with descriptive messages: When unwrap is justified, explain why

```
let config = std::env::var("CONFIG_PATH")
    .expect("CONFIG_PATH environment variable must be set");
```

- Return Result&lt;T, E&gt; for fallible operations: Let callers decide how to handle errors
- <a id="page-210"></a> Use thiserror for custom error types: More ergonomic than manual implementations

```
use thiserror::Error;
#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {message}")]
    Parse { message: String },
    #[error("Value {value} is out of range")]
    OutOfRange { value: i32 },
}
```

- Chain errors with ? operator: Propagate errors up the call stack
- Prefer thiserror over anyhow : Our team convention is to define explicit error enums with `#[derive(thiserror::Error)]` so callers can match on specific variants. `anyhow::Error` is convenient for quick prototyping but erases the error type, making it harder for callers to handle specific failures. Use `thiserror` for library and production code; reserve `anyhow` for throwaway scripts or top-level binaries where you only need to print the error.
- When unwrap() is acceptable:
    - Unit tests: `assert_eq!(result.unwrap(), expected)`
    - Prototyping: Quick and dirty code that you’ll replace
    - Infallible operations: When you can prove it won’t fail

```
let numbers = vec![1, 2, 3];
let first = numbers.get(0).unwrap(); // Safe: we just created the vec with
elements
// Better: Use expect() with explanation
let first = numbers.get(0).expect("numbers vec is non-empty by construction");
```

- Fail fast: Check preconditions early and return errors immediately

<a id="dest-210-6120"></a>

## [Memory Management](<#dest-210-6120>)

- Prefer borrowing over cloning: Use `&T` instead of cloning when possible
- Use Rc&lt;T&gt; sparingly: Only when you need shared ownership
- Limit lifetimes: Use scopes `{}` to control when values are dropped
- Avoid RefCell&lt;T&gt; in public APIs: Keep interior mutability internal

<a id="page-211"></a>

<a id="dest-211-0"></a>

## [Performance](<#dest-211-0>)

- Profile before optimizing: Use `cargo bench` and profiling tools
- Prefer iterators over loops: More readable and often faster
- Use &amp;str over String : When you don’t need ownership
- Consider Box&lt;T&gt; for large stack objects: Move them to heap if needed

<a id="dest-211-1560"></a>

## [Essential Traits to Implement](<#dest-211-1560>)

<a id="dest-211-2160"></a>

### [Core Traits Every Type Should Consider](<#dest-211-2160>)

When creating custom types, consider implementing these fundamental traits to make your types feel native to Rust:

<a id="dest-211-3075"></a>

[Debug and Display](<#dest-211-3075>)

```
use std::fmt;
#[derive(Debug)]  // Automatic implementation for debugging
struct Person {
    name: String,
    age: u32,
}
// Manual Display implementation for user-facing output
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}
// Usage:
let person = Person { name: "Alice".to_string(), age: 30 };
println!("{:?}", person);  // Debug: Person { name: "Alice", age: 30 }
println!("{}", person);    // Display: Alice (age 30)
```

<a id="page-212"></a>

<a id="dest-212-0"></a>

[Clone and Copy](<#dest-212-0>)

```
// Copy: Implicit duplication for small, simple types
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
// Clone: Explicit duplication for complex types
#[derive(Debug, Clone)]
struct Person {
    name: String,  // String doesn't implement Copy
    age: u32,
}
let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Copy (implicit)
let person1 = Person { name: "Bob".to_string(), age: 25 };
let person2 = person1.clone();  // Clone (explicit)
```

<a id="dest-212-3278"></a>

[PartialEq and Eq](<#dest-212-3278>)

```
#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);
#[derive(Debug, PartialEq)]
struct Temperature {
    celsius: f64,  // f64 doesn't implement Eq (due to NaN)
}
let id1 = UserId(123);
let id2 = UserId(123);
assert_eq!(id1, id2);  // Works because of PartialEq
let temp1 = Temperature { celsius: 20.0 };
let temp2 = Temperature { celsius: 20.0 };
assert_eq!(temp1, temp2);  // Works with PartialEq
```

<a id="page-213"></a>

<a id="dest-213-0"></a>

[PartialOrd and Ord](<#dest-213-0>)

```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u8);
let high = Priority(1);
let low = Priority(10);
assert!(high < low);  // Lower numbers = higher priority
// Use in collections
let mut priorities = vec![Priority(5), Priority(1), Priority(8)];
priorities.sort();  // Works because Priority implements Ord
```

<a id="dest-213-2062"></a>

[Default](<#dest-213-2062>)

```
#[derive(Debug, Default)]
struct Config {
    debug: bool,           // false (default)
    max_connections: u32,  // 0 (default)
    timeout: Option<u64>,  // None (default)
}
// Custom Default implementation
impl Default for Config {
    fn default() -> Self {
        Config {
            debug: false,
            max_connections: 100,  // Custom default
            timeout: Some(30),     // Custom default
        }
    }
}
let config = Config::default();
let config = Config { debug: true, ..Default::default() };  // Partial override
```

<a id="page-214"></a>

<a id="dest-214-0"></a>

[From and Into](<#dest-214-0>)

```
struct UserId(u64);
struct UserName(String);
// Implement From, and Into comes for free
impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}
impl From<String> for UserName {
    fn from(name: String) -> Self {
        UserName(name)
    }
}
impl From<&str> for UserName {
    fn from(name: &str) -> Self {
        UserName(name.to_string())
    }
}
// Usage:
let user_id: UserId = 123u64.into();         // Using Into
let user_id = UserId::from(123u64);          // Using From
let username = UserName::from("alice");      // &str -> UserName
let username: UserName = "bob".into();       // Using Into
```

<a id="page-215"></a>

<a id="dest-215-0"></a>

[TryFrom and TryInto](<#dest-215-0>)

```
use std::convert::TryFrom;
struct PositiveNumber(u32);
#[derive(Debug)]
struct NegativeNumberError;
impl TryFrom<i32> for PositiveNumber {
    type Error = NegativeNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(PositiveNumber(value as u32))
        } else {
            Err(NegativeNumberError)
        }
    }
}
// Usage:
let positive = PositiveNumber::try_from(42)?;     // Ok(PositiveNumber(42))
let error = PositiveNumber::try_from(-5);         // Err(NegativeNumberError)
```

<a id="dest-215-3682"></a>

[Serde (for serialization)](<#dest-215-3682>)

```
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}
// Automatic JSON serialization/deserialization
let user = User {
    id: 1,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};
let json = serde_json::to_string(&user)?;
let deserialized: User = serde_json::from_str(&json)?;
```

<a id="page-216"></a>

<a id="dest-216-0"></a>

### [Trait Implementation Checklist](<#dest-216-0>)

For any new type, consider this checklist:

```
#[derive(
    Debug,          // [OK] Always implement for debugging
    Clone,          // [OK] If the type should be duplicatable
    PartialEq,      // [OK] If the type should be comparable
    Eq,             // [OK] If comparison is reflexive/transitive
    PartialOrd,     // [OK] If the type has ordering
    Ord,            // [OK] If ordering is total
    Hash,           // [OK] If type will be used as HashMap key
    Default,        // [OK] If there's a sensible default value
)]
struct MyType {
    // fields...
}
// Manual implementations to consider:
impl Display for MyType { /* user-facing representation */ }
impl From<OtherType> for MyType { /* convenient conversion */ }
impl TryFrom<FallibleType> for MyType { /* fallible conversion */ }
```

<a id="dest-216-3548"></a>

### [When NOT to Implement Traits](<#dest-216-3548>)

- Don’t implement Copy for types with heap data: `String` , `Vec` , `HashMap` etc.
- Don’t implement Eq if values can be NaN: Types containing `f32` / `f64`
- Don’t implement Default if there’s no sensible default: File handles, network connections
- Don’t implement Clone if cloning is expensive: Large data structures (consider `Rc<T>` instead)

<a id="dest-216-5295"></a>

### [Summary: Trait Benefits](<#dest-216-5295>)

Trait Benefit When to Use

```
                     println!("{:?}",
Debug
```

Always (except rare cases) `value)`

User-facing types `Display` `println!("{}", value)`

When explicit duplication makes `Clone` `value.clone()` sense

Implicit duplication Small, simple types `Copy`

`==` and `!=` operators Most types `PartialEq`

<a id="page-217"></a>

![Source region, PDF page 217](Rust-for-C_CPP-Programmers-images/page-0217-region-0069.png)

> PDF page 217: Complex table retained as a source image.

<a id="page-218"></a>

<a id="dest-218-158"></a>

## [Avoiding Excessive clone()](<#dest-218-158>)

<a id="dest-218-930"></a>

## [Avoiding excessive clone()](<#dest-218-930>)

What you’ll learn: Why `.clone()` is a code smell in Rust, how to restructure ownership

to eliminate unnecessary copies, and the specific patterns that signal an ownership design problem.

- Coming from C++, `.clone()` feels like a safe default — “just copy it”. But excessive cloning hides ownership problems and hurts performance.
- Rule of thumb: If you’re cloning to satisfy the borrow checker, you probably need to restructure ownership instead.

<a id="dest-218-3315"></a>

### [When clone() is wrong](<#dest-218-3315>)

```
// BAD: Cloning a String just to pass it to a function that only reads it
fn log_message(msg: String) {  // Takes ownership unnecessarily
    println!("[LOG] {}", msg);
}
let message = String::from("GPU test passed");
log_message(message.clone());  // Wasteful: allocates a whole new String
log_message(message);           // Original consumed — clone was pointless
```

```
// GOOD: Accept a borrow — zero allocation
fn log_message(msg: &str) {    // Borrows, doesn't own
    println!("[LOG] {}", msg);
}
let message = String::from("GPU test passed");
log_message(&message);          // No clone, no allocation
log_message(&message);          // Can call again — message not consumed
```

<a id="page-219"></a>

<a id="dest-219-0"></a>

### [Real example: returning &amp;str instead of cloning](<#dest-219-0>)

```
// Example: healthcheck.rs — returns a borrowed view, zero allocation
pub fn serial_or_unknown(&self) -> &str {
    self.serial.as_deref().unwrap_or(UNKNOWN_VALUE)
}
pub fn model_or_unknown(&self) -> &str {
    self.model.as_deref().unwrap_or(UNKNOWN_VALUE)
}
```

The C++ equivalent would return `const std::string&` or `std::string_view` — but in C++ neither is lifetime-checked. In Rust, the borrow checker guarantees the returned `&str` can’t outlive `self` .

<a id="dest-219-2572"></a>

### [Real example: static string slices — no heap at all](<#dest-219-2572>)

```
// Example: healthcheck.rs — compile-time string tables
const HBM_SCREEN_RECIPES: &[&str] = &[
    "hbm_ds_ntd", "hbm_ds_ntd_gfx", "hbm_dt_ntd", "hbm_dt_ntd_gfx",
    "hbm_burnin_8h", "hbm_burnin_24h",
];
```

In C++ this would typically be `std::vector<std::string>` (heap-allocated on first use). Rust’s

`&'static [&'static str]` lives in read-only memory — zero runtime cost.

<a id="dest-219-4560"></a>

### [When clone() IS appropriate](<#dest-219-4560>)

Situation Why clone is OK Example

Bumps ref count (~1 ns), doesn’t copy data

`Arc::clone()` for threading

```
let flag = stop_flag.clone();
```

Moving data into a spawned thread

Thread needs its own copy

```
let ctx = ctx.clone();
thread::spawn(move || { ... })
```

Extracting from

Can’t move out of a borrow

`self.name.clone()` when returning owned `String`

`&self` fields

Small `Copy` types wrapped in `Option`

`.copied()` is clearer than `.clone()`

`opt.get(0).copied()` for

`Option<&u32>` → `Option<u32>`

<a id="page-220"></a>

<a id="dest-220-0"></a>

[Real example: Arc::clone for thread sharing](<#dest-220-0>)

```
// Example: workload.rs — Arc::clone is cheap (ref count bump)
let stop_flag = Arc::new(AtomicBool::new(false));
let stop_flag_clone = stop_flag.clone();   // ~1 ns, no data copied
let ctx_clone = ctx.clone();               // Clone context for move into thread
let sensor_handle = thread::spawn(move || {
    // ...uses stop_flag_clone and ctx_clone
});
```

<a id="dest-220-1905"></a>

[Checklist: Should I clone?](<#dest-220-1905>)

1. Can I accept &amp;str / &amp;T instead of String / T ? → Borrow, don’t clone
2. Can I restructure to avoid needing two owners? → Pass by reference or use scopes
3. Is this Arc::clone() ? → That’s fine, it’s O(1)
4. Am I moving data into a thread/closure? → Clone is necessary
5. Am I cloning in a hot loop? → Profile and consider borrowing or `Cow<T>`

### [Cow&lt;'a, T&gt;: Clone-on-Write — borrow when you can, clone](<#page-220>) [when you must](<#page-220>)

`Cow` (Clone on Write) is an enum that holds either a borrowed reference or an owned value. It’s the Rust equivalent of “avoid allocation when possible, but allocate if you need to modify.” C++ has no direct equivalent — the closest is a function that returns `const std::string&`

sometimes and `std::string` other times.

<a id="page-221"></a>

<a id="dest-221-0"></a>

### [Why Cow exists](<#dest-221-0>)

```
// Without Cow — you must choose: always borrow OR always clone
fn normalize(s: &str) -> String {          // Always allocates!
    if s.contains(' ') {
        s.replace(' ', "_")               // New String (allocation needed)
    } else {
        s.to_string()                     // Unnecessary allocation!
    }
}
// With Cow — borrow when unchanged, allocate only when modified
use std::borrow::Cow;
fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))    // Allocates (must modify)
    } else {
        Cow::Borrowed(s)                   // Zero allocation (passthrough)
    }
}
```

<a id="page-222"></a>

<a id="dest-222-0"></a>

### [How Cow works](<#dest-222-0>)

```
use std::borrow::Cow;
// Cow<'a, str> is essentially:
// enum Cow<'a, str> {
//     Borrowed(&'a str),     // Zero-cost reference
//     Owned(String),          // Heap-allocated owned value
// }
fn greet(name: &str) -> Cow<'_, str> {
    if name.is_empty() {
        Cow::Borrowed("stranger")         // Static string — no allocation
    } else if name.starts_with(' ') {
        Cow::Owned(name.trim().to_string()) // Modified — allocation needed
    } else {
        Cow::Borrowed(name)               // Passthrough — no allocation
    }
}
fn main() {
    let g1 = greet("Alice");     // Cow::Borrowed("Alice")
    let g2 = greet("");          // Cow::Borrowed("stranger")
    let g3 = greet(" Bob ");     // Cow::Owned("Bob")
    // Cow<str> implements Deref<Target = str>, so you can use it as &str:
    println!("Hello, {g1}!");    // Works — Cow auto-derefs to &str
    println!("Hello, {g2}!");
    println!("Hello, {g3}!");
}
```

<a id="page-223"></a>

<a id="dest-223-0"></a>

### [Real-world use case: config value normalization](<#dest-223-0>)

```
use std::borrow::Cow;
/// Normalize a SKU name: trim whitespace, lowercase.
/// Returns Cow::Borrowed if already normalized (zero allocation).
fn normalize_sku(sku: &str) -> Cow<'_, str> {
    let trimmed = sku.trim();
    if trimmed == sku && sku.chars().all(|c| c.is_lowercase() ||
!c.is_alphabetic()) {
        Cow::Borrowed(sku)   // Already normalized — no allocation
    } else {
        Cow::Owned(trimmed.to_lowercase())  // Needs modification — allocate
    }
}
fn main() {
    let s1 = normalize_sku("server-x1");   // Borrowed — zero alloc
    let s2 = normalize_sku("  Server-X1 "); // Owned — must allocate
    println!("{s1}, {s2}"); // "server-x1, server-x1"
}
```

<a id="dest-223-3390"></a>

### [When to use Cow](<#dest-223-3390>)

![Source region, PDF page 223](Rust-for-C_CPP-Programmers-images/page-0223-region-0070.png)

> PDF page 223: Complex table retained as a source image.

C++ comparison: `Cow<str>` is like a function that returns

`std::variant<std::string_view, std::string>` — except with automatic deref and no

boilerplate to access the value.

<a id="page-224"></a>

![Source region, PDF page 224](Rust-for-C_CPP-Programmers-images/page-0224-region-0071.png)

<a id="page-225"></a>

```
use std::rc::{Rc, Weak};
use std::cell::RefCell;
#[derive(Debug)]
struct Node {
    value: String,
    parent: RefCell<Weak<Node>>,      // Weak — doesn't prevent parent from
dropping
    children: RefCell<Vec<Rc<Node>>>,  // Strong — parent owns children
}
impl Node {
    fn new(value: &str) -> Rc<Node> {
        Rc::new(Node {
            value: value.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }
    fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        // Child gets a weak reference to parent (no cycle)
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        // Parent gets a strong reference to child
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}
fn main() {
    let root = Node::new("root");
    let child = Node::new("child");
    Node::add_child(&root, &child);
    // Access parent from child via upgrade()
    if let Some(parent) = child.parent.borrow().upgrade() {
        println!("Child's parent: {}", parent.value); // "root"
    }
    println!("Root strong count: {}", Rc::strong_count(&root));  // 1
    println!("Root weak count: {}", Rc::weak_count(&root));      // 1
}
```

<a id="page-226"></a>

<a id="dest-226-0"></a>

### [C++ comparison](<#dest-226-0>)

```
// C++ — weak_ptr to break shared_ptr cycle
struct Node {
    std::string value;
    std::weak_ptr<Node> parent;                  // Weak — no ownership
    std::vector<std::shared_ptr<Node>> children;  // Strong — owns children
    static auto create(const std::string& v) {
        return std::make_shared<Node>(Node{v, {}, {}});
    }
};
auto root = Node::create("root");
auto child = Node::create("child");
child->parent = root;          // weak_ptr assignment
root->children.push_back(child);
if (auto p = child->parent.lock()) {   // lock() → shared_ptr or null
    std::cout << "Parent: " << p->value << std::endl;
}
```

C++ Rust Notes

`Rc<T>` (single-thread) / `Arc<T>` `shared_ptr<T>` (multi-thread)

Same semantics

`Weak<T>` from `Rc::downgrade()` `weak_ptr<T>` / `Arc::downgrade()`

Same semantics

`weak_ptr::lock()` →

`Weak::upgrade()` →

`None` if dropped

`shared_ptr` or null

```
Option<Rc<T>>
```

Same `shared_ptr::use_count()` `Rc::strong_count()` meaning

<a id="dest-226-5265"></a>

### [When to use Weak](<#dest-226-5265>)

![Source region, PDF page 226](Rust-for-C_CPP-Programmers-images/page-0226-region-0072.png)

> PDF page 226: Complex table retained as a source image.

<a id="page-227"></a>

![Source region, PDF page 227](Rust-for-C_CPP-Programmers-images/page-0227-region-0073.png)

> PDF page 227: Complex table retained as a source image.

Prefer the arena pattern (Case Study 2) over `Rc/Weak` for tree structures in new code.

`Vec<T>` + indices is simpler, faster, and has zero reference-counting overhead. Use `Rc/Weak` when you need shared ownership with dynamic lifetimes.

<a id="dest-227-2168"></a>

### [Copy vs Clone, PartialEq vs Eq — when to derive what](<#dest-227-2168>)

- Copy ≈ C++ trivially copyable (no custom copy ctor/dtor). Types like `int` , `enum` , and simple POD structs — the compiler generates a bitwise `memcpy` automatically. In Rust, `Copy` is the same idea: assignment `let b = a;` does an implicit bitwise copy and both variables remain valid.
- Clone ≈ C++ copy constructor / operator= deep-copy. When a C++ class has a custom copy constructor (e.g., to deep-copy a `std::vector` member), the equivalent in Rust is implementing `Clone` . You must call `.clone()` explicitly — Rust never hides an expensive copy behind `=` .
- Key distinction: In C++, both trivial copies and deep copies happen implicitly via the same `=` syntax. Rust forces you to choose: `Copy` types copy silently (cheap), non- `Copy` types move by default, and you must opt in to an expensive duplicate with `.clone()` .
- Similarly, C++ `operator==` doesn’t distinguish between types where `a == a` always holds (like integers) and types where it doesn’t (like `float` with NaN). Rust encodes this in `PartialEq` vs `Eq` .

<a id="dest-227-5445"></a>

[Copy vs Clone](<#dest-227-5445>)

Copy Clone

Custom logic (explicit How it works Bitwise memcpy (implicit)

`.clone()` )

When it Only when you call On assignment: `let b = a;` happens

```
.clone()
```

After Both `a` and `b` are valid Both `a` and `b` are valid copy/clone

<a id="page-228"></a>

Copy Clone

```
Without                                                 let b = a; moves a (a
                   let b = a; moves a (a is gone)
either                                               is gone)
```

Allowed for Types with no heap data Any type

Trivially copyable / POD types (no C++ analogy custom copy ctor)

Custom copy constructor (deep copy)

<a id="dest-228-1552"></a>

### [Real example: Copy — simple enums](<#dest-228-1552>)

```
// From fan_diag/src/sensor.rs — all unit variants, fits in 1 byte
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FanStatus {
    #[default]
    Normal,
    Low,
    High,
    Missing,
    Failed,
    Unknown,
}
let status = FanStatus::Normal;
let copy = status;   // Implicit copy — status is still valid
println!("{:?} {:?}", status, copy);  // Both work
```

<a id="dest-228-4410"></a>

### [Real example: Copy — enum with integer payloads](<#dest-228-4410>)

```
// Example: healthcheck.rs — u32 payloads are Copy, so the whole enum is too
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthcheckStatus {
    Pass,
    ProgramError(u32),
    DmesgError(u32),
    RasError(u32),
    OtherError(u32),
    Unknown,
}
```

<a id="page-229"></a>

<a id="dest-229-0"></a>

### [Real example: Clone only — struct with heap data](<#dest-229-0>)

```
// Example: components.rs — String prevents Copy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FruData {
    pub technology: DeviceTechnology,
    pub physical_location: String,      // ← String: heap-allocated, can't Copy
    pub expected: bool,
    pub removable: bool,
}
// let a = fru_data;   → MOVES (a is gone)
// let a = fru_data.clone();  → CLONES (fru_data still valid, new heap allocation)
```

<a id="dest-229-2175"></a>

### [The rule: Can it be Copy?](<#dest-229-2175>)

```
Does the type contain String, Vec, Box, HashMap,
Rc, Arc, or any other heap-owning type?
    YES → Clone only (cannot be Copy)
    NO  → You CAN derive Copy (and should, if the type is small)
```

<a id="dest-229-3548"></a>

### [PartialEq vs Eq](<#dest-229-3548>)

PartialEq Eq

What it gives you `==` and `!=` operators Marker: “equality is reflexive”

Reflexive? (a == Not guaranteed Guaranteed a)

```
f32::NAN !=
```

Why it matters

`HashMap` keys require `Eq` `f32::NAN`

When the type has no `f32` / `f64` When to derive Almost always fields

No direct equivalent (C++ doesn’t C++ analogy `operator==` check)

<a id="page-230"></a>

<a id="dest-230-0"></a>

### [Real example: Eq — used as HashMap key](<#dest-230-0>)

```
// From hms_trap/src/cpu_handler.rs — Hash requires Eq
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuFaultType {
    InvalidFaultType,
    CpuCperFatalErr,
    CpuLpddr5UceErr,
    CpuC2CUceFatalErr,
    // ...
}
// Used as: HashMap<CpuFaultType, FaultHandler>
// HashMap keys must be Eq + Hash — PartialEq alone won't compile
```

<a id="dest-230-2310"></a>

### [Real example: No Eq possible — type contains f32](<#dest-230-2310>)

```
// Example: types.rs — f32 prevents Eq
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemperatureSensors {
    pub warning_threshold: Option<f32>,   // ← f32 has NaN ≠ NaN
    pub critical_threshold: Option<f32>,  // ← can't derive Eq
    pub sensor_names: Vec<String>,
}
// Cannot be used as HashMap key. Cannot derive Eq.
// Because: f32::NAN == f32::NAN is false, violating reflexivity.
```

<a id="dest-230-4358"></a>

### [PartialOrd vs Ord](<#dest-230-4358>)

PartialOrd Ord

What it gives you

![Source region, PDF page 230](Rust-for-C_CPP-Programmers-images/page-0230-region-0074.png)

> PDF page 230: Mathematics retained as a source image.

No (some pairs may be Total ordering? incomparable)

Yes (every pair is comparable)

PartialOrd only (NaN breaks f32/f64? Cannot derive Ord ordering)

<a id="page-231"></a>

<a id="dest-231-0"></a>

### [Real example: Ord — severity ranking](<#dest-231-0>)

```
// From hms_trap/src/fault.rs — variant order defines severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FaultSeverity {
    Info,      // lowest  (discriminant 0)
    Warning,   //         (discriminant 1)
    Error,     //         (discriminant 2)
    Critical,  // highest (discriminant 3)
}
// FaultSeverity::Info < FaultSeverity::Critical → true
// Enables: if severity >= FaultSeverity::Error { escalate(); }
```

<a id="dest-231-2175"></a>

### [Real example: Ord — diagnostic levels for comparison](<#dest-231-2175>)

```
// Example: orchestration.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum GpuDiagLevel {
    #[default]
    Quick,     // lowest
    Standard,
    Extended,
    Full,      // highest
}
// Enables: if requested_level >= GpuDiagLevel::Extended { run_extended_tests(); }
```

<a id="page-232"></a>

<a id="dest-232-0"></a>

### [Derive decision tree](<#dest-232-0>)

```
                        Your new type
                            │
                   Contains String/Vec/Box?
                      /              \
                    YES                NO
                     │                  │
              Clone only          Clone + Copy
                     │                  │
              Contains f32/f64?    Contains f32/f64?
                /          \         /          \
              YES           NO     YES           NO
               │             │      │             │
         PartialEq       PartialEq  PartialEq  PartialEq
         only            + Eq       only       + Eq
                          │                      │
                    Need sorting?           Need sorting?
                      /       \               /       \
                    YES        NO            YES        NO
                     │          │              │          │
               PartialOrd    Done        PartialOrd    Done
               + Ord                     + Ord
                     │                        │
               Need as                  Need as
               map key?                 map key?
                  │                        │
                + Hash                   + Hash
```

<a id="dest-232-4335"></a>

### [Quick reference: common derive combos from production Rust code](<#dest-232-4335>)

Type category Typical derive Example

`Copy, Clone, PartialEq, Eq,` Simple status enum

```
                                   FanStatus
Default
Copy, Clone, PartialEq, Eq,
Hash                               SelComponent
Copy, Clone, PartialEq, Eq,
PartialOrd, Ord                    GpuDiagLevel
Clone, Debug, Serialize,
Deserialize                        OverallSummary
                                   DiagConfig
Serialize, Deserialize
```

Enum used as HashMap key

`CpuFaultType` ,

Sortable severity enum

`FaultSeverity` ,

Data struct with Strings

`FruData` ,

`Clone, Debug, Default,` Serializable config

<a id="page-233"></a>

<a id="dest-233-158"></a>

## [Avoiding Unchecked Indexing](<#dest-233-158>)

<a id="dest-233-930"></a>

## [Avoiding unchecked indexing](<#dest-233-930>)

What you’ll learn: Why `vec[i]` is dangerous in Rust (panics on out-of-bounds), and safe

alternatives like `.get()` , iterators, and `entry()` API for `HashMap` . Replaces C++’s undefined behavior with explicit handling.

- In C++, `vec[i]` and `map[key]` have undefined behavior / auto-insert on missing keys. Rust’s `[]` panics on out-of-bounds.
- Rule: Use `.get()` instead of `[]` unless you can prove the index is valid.

<a id="dest-233-3165"></a>

### [C++ → Rust comparison](<#dest-233-3165>)

```
// C++ — silent UB or insertion
std::vector<int> v = {1, 2, 3};
int x = v[10];        // UB! No bounds check with operator[]
std::map<std::string, int> m;
int y = m["missing"]; // Silently inserts key with value 0!
```

```
// Rust — safe alternatives
let v = vec![1, 2, 3];
// Bad: panics if index out of bounds
// let x = v[10];
// Good: returns Option<&i32>
let x = v.get(10);              // None — no panic
let x = v.get(1).copied().unwrap_or(0);  // 2, or 0 if missing
```

<a id="dest-233-6285"></a>

### [Real example: safe byte parsing from production Rust code](<#dest-233-6285>)

```
// Example: diagnostics.rs
// Parsing a binary SEL record — buffer might be shorter than expected
let sensor_num = bytes.get(7).copied().unwrap_or(0);
let ppin = cpu_ppin.get(i).map(|s| s.as_str()).unwrap_or("");
```

<a id="page-234"></a>

<a id="dest-234-0"></a>

### [Real example: chained safe lookups with .and\_then()](<#dest-234-0>)

```
// Example: profile.rs — double lookup: HashMap → Vec
pub fn get_processor(&self, location: &str) -> Option<&Processor> {
    self.processor_by_location
        .get(location)                              // HashMap → Option<&usize>
        .and_then(|&idx| self.processors.get(idx))   // Vec → Option<&Processor>
}
// Both lookups return Option — no panics, no UB
```

<a id="dest-234-1770"></a>

### [Real example: safe JSON navigation](<#dest-234-1770>)

```
// Example: framework.rs — every JSON key returns Option
let manufacturer = product_fru
    .get("Manufacturer")            // Option<&Value>
    .and_then(|v| v.as_str())       // Option<&str>
    .unwrap_or(UNKNOWN_VALUE)       // &str (safe fallback)
    .to_string();
```

Compare to the C++ pattern: `json["SystemInfo"]["ProductFru"]["Manufacturer"]` — any missing key throws `nlohmann::json::out_of_range` .

<a id="dest-234-3892"></a>

### [When \[\] is acceptable](<#dest-234-3892>)

```
After a bounds check: if i < v.len() { v[i] }
In tests: Where panicking is the desired behavior
With constants: let first = v[0]; right after assert!(!v.is_empty());
```

<a id="dest-234-5340"></a>

## [Safe value extraction with unwrap\_or](<#dest-234-5340>)

- `unwrap()` panics on `None` / `Err` . In production code, prefer the safe alternatives.

<a id="dest-234-6270"></a>

### [The unwrap family](<#dest-234-6270>)

![Source region, PDF page 234](Rust-for-C_CPP-Programmers-images/page-0234-region-0075.png)

> PDF page 234: Complex table retained as a source image.

<a id="page-235"></a>

Method Behavior on None/Err Use When

When panic is justified, Panics with message `.expect("msg")` explain why

You have a cheap Returns `default` `.unwrap_or(default)` constant fallback

Fallback is expensive to Calls closure compute `expr)`

```
.unwrap_or_else(||
```

Returns `.unwrap_or_default()`

Type implements

```
Default::default()          Default
```

<a id="dest-235-2100"></a>

### [Real example: parsing with safe defaults](<#dest-235-2100>)

```
// Example: peripherals.rs
// Regex capture groups might not match — provide safe fallbacks
let bus_hex = caps.get(1).map(|m| m.as_str()).unwrap_or("00");
let fw_status = caps.get(5).map(|m| m.as_str()).unwrap_or("0x0");
let bus = u8::from_str_radix(bus_hex, 16).unwrap_or(0);
```

<a id="dest-235-3608"></a>

### [Real example: unwrap\_or\_else with fallback struct](<#dest-235-3608>)

```
// Example: framework.rs
// Full function wraps logic in an Option-returning closure;
// if anything fails, return a default struct:
(|| -> Option<BaseboardFru> {
    let content = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    // ... extract fields with .get()? chains
    Some(baseboard_fru)
})()
.unwrap_or_else(|| BaseboardFru {
    manufacturer: String::new(),
    model: String::new(),
    product_part_number: String::new(),
    serial_number: String::new(),
    asset_tag: String::new(),
})
```

<a id="page-236"></a>

<a id="dest-236-0"></a>

### [Real example: unwrap\_or\_default on config deserialization](<#dest-236-0>)

```
// Example: framework.rs
// If JSON config parsing fails, fall back to Default — no crash
Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
```

The C++ equivalent would be a `try/catch` around `nlohmann::json::parse()` with manual default construction in the catch block.

<a id="dest-236-1942"></a>

## [Functional transforms: map, map\_err, find\_map](<#dest-236-1942>)

- These methods on `Option` and `Result` let you transform the contained value without unwrapping, replacing nested `if/else` with linear chains.

<a id="dest-236-3052"></a>

### [Quick reference](<#dest-236-3052>)

Method On Does C++ Equivalent

`Option` / `.map(|v| ...)`

Transform the

```
                                                        if (opt) { *opt =
                   Result                               transform(*opt); }
.map_err(|e|
```

`Some` / `Ok` value

Transform the `Err` `Result` value `...)`

Adding context to catch block

Chain operations that return

`Option` /

```
.and_then(|v|
...)                Result
.find_map(|v|
```

Nested if-checks

`Option` / `Result`

`find` + `map` in one Iterator Loop with `if + break` pass `...)`

`Option` / Iterator

Keep only values matching predicate

```
.filter(|v|                                              if (!predicate) return
...)                                                    nullopt;
                                                         if
.ok()?              Result                              (result.has_error())
                                                        return nullopt;
```

Convert `Result →`

`Option` and propagate `None`

<a id="page-237"></a>

<a id="dest-237-0"></a>

### [Real example: .and\_then() chain for JSON field extraction](<#dest-237-0>)

```
// Example: framework.rs — finding serial number with fallbacks
let sys_info = json.get("SystemInfo")?;
// Try BaseboardFru.BoardSerialNumber first
if let Some(serial) = sys_info
    .get("BaseboardFru")
    .and_then(|b| b.get("BoardSerialNumber"))
    .and_then(|v| v.as_str())
    .filter(valid_serial)     // Only accept non-empty, valid serials
{
    return Some(serial.to_string());
}
// Fallback to BoardFru.SerialNumber
sys_info
    .get("BoardFru")
    .and_then(|b| b.get("SerialNumber"))
    .and_then(|v| v.as_str())
    .filter(valid_serial)
    .map(|s| s.to_string())   // Convert &str → String only if Some
```

In C++ this would be a pyramid of `if (json.contains("BaseboardFru")) { if`

`(json["BaseboardFru"].contains("BoardSerialNumber")) { ... } }` .

<a id="dest-237-4005"></a>

### [Real example: find\_map — search + transform in one pass](<#dest-237-4005>)

```
// Example: context.rs — find SDR record matching sensor + owner
pub fn find_for_event(&self, sensor_number: u8, owner_id: u8) ->
Option<&SdrRecord> {
    self.by_sensor.get(&sensor_number).and_then(|indices| {
        indices.iter().find_map(|&i| {
            let record = &self.records[i];
            if record.sensor_owner_id() == Some(owner_id) {
                Some(record)
            } else {
                None
            }
        })
    })
}
```

`find_map` is `find` + `map` fused: it stops at the first match and transforms it. The C++ equivalent is a `for` loop with an `if` + `break` .

<a id="page-238"></a>

<a id="dest-238-0"></a>

### [Real example: map\_err for error context](<#dest-238-0>)

```
// Example: main.rs — add context to errors before propagating
let json_str = serde_json::to_string_pretty(&config)
    .map_err(|e| format!("Failed to serialize config: {}", e))?;
```

Transforms a `serde_json::Error` into a descriptive `String` error that includes context about what failed.

<a id="dest-238-1942"></a>

## [JSON handling: nlohmann::json → serde](<#dest-238-1942>)

- C++ teams typically use `nlohmann::json` for JSON parsing. Rust uses serde + serde\_json — which is more powerful because the JSON schema is encoded in the type system.

<a id="dest-238-3045"></a>

### [C++ (nlohmann) vs Rust (serde) comparison](<#dest-238-3045>)

```
// C++ with nlohmann::json — runtime field access
#include <nlohmann/json.hpp>
using json = nlohmann::json;
struct Fan {
    std::string logical_id;
    std::vector<std::string> sensor_ids;
};
Fan parse_fan(const json& j) {
    Fan f;
    f.logical_id = j.at("LogicalID").get<std::string>();    // throws if missing
    if (j.contains("SDRSensorIdHexes")) {                   // manual default
handling
        f.sensor_ids = j["SDRSensorIdHexes"].get<std::vector<std::string>>();
    }
    return f;
}
```

<a id="page-239"></a>

```
// Rust with serde — compile-time schema, automatic field mapping
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fan {
    pub logical_id: String,
    #[serde(rename = "SDRSensorIdHexes", default)]  // JSON key → Rust field
    pub sensor_ids: Vec<String>,                     // Missing → empty Vec
    #[serde(default)]
    pub sensor_names: Vec<String>,                   // Missing → empty Vec
}
// One line replaces the entire parse function:
let fan: Fan = serde_json::from_str(json_str)?;
```

<a id="dest-239-2385"></a>

### [Key serde attributes (real examples from production Rust code)](<#dest-239-2385>)

Attribute Purpose C++ Equivalent

Use `Default::default()` `#[serde(default)]` for missing fields

```
                                                      if (j.contains(key)) {
                                                      ... } else { default; }
#[serde(rename =
"Key")]
                                                      for (auto& [k,v] :
                        HashMap                       j.items()) { ... }
```

Map JSON key name to Rust Manual `j.at("Key")` access field name

Absorb unknown keys into `#[serde(flatten)]`

Don’t serialize/deserialize Not storing in JSON `#[serde(skip)]` this field

Internally tagged enum (discriminator field)

```
#[serde(tag =                                         if (j["type"] == "gpu") {
"type")]                                              ... }
```

<a id="page-240"></a>

<a id="dest-240-0"></a>

### [Real example: full config struct](<#dest-240-0>)

```
// Example: diag.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagConfig {
    pub sku: SkuConfig,
    #[serde(default)]
    pub level: DiagLevel,            // Missing → DiagLevel::default()
    #[serde(default)]
    pub modules: ModuleConfig,       // Missing → ModuleConfig::default()
    #[serde(default)]
    pub output_dir: String,          // Missing → ""
    #[serde(default, flatten)]
    pub options: HashMap<String, serde_json::Value>,  // Absorbs unknown keys
}
// Loading is 3 lines (vs ~20+ in C++ with nlohmann):
let content = std::fs::read_to_string(path)?;
let config: DiagConfig = serde_json::from_str(&content)?;
Ok(config)
```

<a id="dest-240-3255"></a>

### [Enum deserialization with \#\[serde(tag = "type")\]](<#dest-240-3255>)

```
// Example: components.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]                   // JSON: {"type": "Gpu", "product": ...}
pub enum PcieDeviceKind {
    Gpu { product: GpuProduct, manufacturer: GpuManufacturer },
    Nic { product: NicProduct, manufacturer: NicManufacturer },
    NvmeDrive { drive_type: StorageDriveType, capacity_gb: u32 },
    // ... 9 more variants
}
// serde automatically dispatches on the "type" field — no manual if/else chain
```

The C++ equivalent would be: `if (j["type"] == "Gpu") { parse_gpu(j); } else if`

```
(j["type"] == "Nic") { parse_nic(j); } ...
```

<a id="dest-240-5730"></a>

## [Exercise: JSON deserialization with serde](<#dest-240-5730>)

- Define a `ServerConfig` struct that can be deserialized from the following JSON:

<a id="page-241"></a>

```
{
    "hostname": "diag-node-01",
    "port": 8080,
    "debug": true,
    "modules": ["accel_diag", "nic_diag", "cpu_diag"]
}
```

- Use `#[derive(Deserialize)]` and `serde_json::from_str()` to parse it
- Add `#[serde(default)]` to `debug` so it defaults to `false` if missing
- Bonus: Add an `enum DiagLevel { Quick, Full, Extended }` field with `#` `[serde(default)]` that defaults to `Quick`

Starter code (requires `cargo add serde --features derive` and `cargo add serde_json` ):

```
use serde::Deserialize;
// TODO: Define DiagLevel enum with Default impl
// TODO: Define ServerConfig struct with serde attributes
fn main() {
    let json_input = r#"{
        "hostname": "diag-node-01",
        "port": 8080,
        "debug": true,
        "modules": ["accel_diag", "nic_diag", "cpu_diag"]
    }"#;
    // TODO: Deserialize and print the config
    // TODO: Try parsing JSON with "debug" field missing — verify it defaults to
false
}
```

- [ ] Solution (click to expand)

<a id="page-242"></a>

<a id="dest-242-158"></a>

## [Collapsing Assignment Pyramids](<#dest-242-158>)

<a id="dest-242-930"></a>

## [Collapsing assignment pyramids with closures](<#dest-242-930>)

What you’ll learn: How Rust’s expression-based syntax and closures flatten deeply- nested C++ `if/else` validation chains into clean, linear code.

- C++ often requires multi-block `if/else` chains to assign variables, especially when validation or fallback logic is involved. Rust’s expression-based syntax and closures collapse these into flat, linear code.

<a id="dest-242-2970"></a>

### [Pattern 1: Tuple assignment with if expression](<#dest-242-2970>)

```
// C++ — three variables set across a multi-block if/else chain
uint32_t fault_code;
const char* der_marker;
const char* action;
if (is_c44ad) {
    fault_code = 32709; der_marker = "CSI_WARN"; action = "No action";
} else if (error.is_hardware_error()) {
    fault_code = 67956; der_marker = "CSI_ERR"; action = "Replace GPU";
} else {
    fault_code = 32709; der_marker = "CSI_WARN"; action = "No action";
}
```

```
// Rust equivalent:accel_fieldiag.rs
// Single expression assigns all three at once:
let (fault_code, der_marker, recommended_action) = if is_c44ad {
    (32709u32, "CSI_WARN", "No action")
} else if error.is_hardware_error() {
    (67956u32, "CSI_ERR", "Replace GPU")
} else {
    (32709u32, "CSI_WARN", "No action")
};
```

<a id="page-243"></a>

<a id="dest-243-0"></a>

### [Pattern 2: IIFE (Immediately Invoked Function Expression) for fallible chains](<#dest-243-0>)

```
// C++ — pyramid of doom for JSON navigation
std::string get_part_number(const nlohmann::json& root) {
    if (root.contains("SystemInfo")) {
        auto& sys = root["SystemInfo"];
        if (sys.contains("BaseboardFru")) {
            auto& bb = sys["BaseboardFru"];
            if (bb.contains("ProductPartNumber")) {
                return bb["ProductPartNumber"].get<std::string>();
            }
        }
    }
    return "UNKNOWN";
}
```

```
// Rust equivalent:framework.rs
// Closure + ? operator collapses the pyramid into linear code:
let part_number = (|| -> Option<String> {
    let path = self.args.sysinfo.as_ref()?;
    let content = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    let ppn = json
        .get("SystemInfo")?
        .get("BaseboardFru")?
        .get("ProductPartNumber")?
        .as_str()?;
    Some(ppn.to_string())
})()
.unwrap_or_else(|| "UNKNOWN".to_string());
```

The closure creates an `Option<String>` scope where `?` bails early at any step. The

`.unwrap_or_else()` provides the fallback once, at the end.

<a id="dest-243-5220"></a>

### [Pattern 3: Iterator chain replacing manual loop + push\_back](<#dest-243-5220>)

```
// C++ — manual loop with intermediate variables
std::vector<std::tuple<std::vector<std::string>, std::string, std::string>>
gpu_info;
for (const auto& [key, info] : gpu_pcie_map) {
    std::vector<std::string> bdfs;
    // ... parse bdf_path into bdfs
    std::string serial = info.serial_number.value_or("UNKNOWN");
    std::string model = info.model_number.value_or(model_name);
    gpu_info.push_back({bdfs, serial, model});
}
```

<a id="page-244"></a>

```
// Rust equivalent:peripherals.rs
// Single chain: values() → map → collect
let gpu_info: Vec<(Vec<String>, String, String, String)> = self
    .gpu_pcie_map
    .values()
    .map(|info| {
        let bdfs: Vec<String> = info.bdf_path
            .split(')')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('(').to_string())
            .collect();
        let serial = info.serial_number.clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let model = info.model_number.clone()
            .unwrap_or_else(|| model_name.to_string());
        let gpu_bdf = format!("{}:{}:{}.{}",
            info.bdf.segment, info.bdf.bus, info.bdf.device, info.bdf.function);
        (bdfs, serial, model, gpu_bdf)
    })
    .collect();
```

### [Pattern 4: .filter().collect() replacing loop + if (condition) ](<#page-244>) [continue](<#page-244>)

```
// C++
std::vector<TestResult*> failures;
for (auto& t : test_results) {
    if (!t.is_pass()) {
        failures.push_back(&t);
    }
}
```

```
// Rust — from accel_diag/src/healthcheck.rs
pub fn failed_tests(&self) -> Vec<&TestResult> {
    self.test_results.iter().filter(|t| !t.is_pass()).collect()
}
```

<a id="dest-244-5970"></a>

### [Summary: When to use each pattern](<#dest-244-5970>)

C++ Pattern Rust Replacement Key Benefit

Multi-block variable assignment

All variables bound atomically

```
let (a, b) = if ... { } else {
};
```

Nested `if`

IIFE closure with `?` operator Linear, flat, early-exit `(contains)` pyramid

<a id="page-245"></a>

![Source region, PDF page 245](Rust-for-C_CPP-Programmers-images/page-0245-region-0076.png)

> PDF page 245: Complex table retained as a source image.

<a id="dest-245-1605"></a>

## [Capstone Exercise: Diagnostic Event Pipeline](<#dest-245-1605>)

🔴 **Challenge** — integrative exercise combining enums, traits, iterators, error handling, and generics

This integrative exercise brings together enums, traits, iterators, error handling, and generics. You’ll build a simplified diagnostic event processing pipeline similar to patterns used in production Rust code.

#### **Requirements:**

1. Define an `enum Severity { Info, Warning, Critical }` with `Display` , and a `struct` `DiagEvent` containing `source: String` , `severity: Severity` , `message: String` , and

```
fault_code: u32
```

2. Define a `trait EventFilter` with a method `fn should_include(&self, event:` `&DiagEvent) -> bool`
3. Implement two filters: `SeverityFilter` (only events &gt;= a given severity) and `SourceFilter` (only events from a specific source string)
4. Write a function `fn process_events(events: &[DiagEvent], filters: &[&dyn` `EventFilter]) -> Vec<String>` that returns formatted report lines for events that pass all filters
5. Write a `fn parse_event(line: &str) -> Result<DiagEvent, String>` that parses lines of the form `"source:severity:fault_code:message"` (return `Err` for bad input)

#### **Starter code:**

<a id="page-246"></a>

```
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Info,
    Warning,
    Critical,
}
impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
#[derive(Debug, Clone)]
struct DiagEvent {
    source: String,
    severity: Severity,
    message: String,
    fault_code: u32,
}
trait EventFilter {
    fn should_include(&self, event: &DiagEvent) -> bool;
}
struct SeverityFilter {
    min_severity: Severity,
}
// TODO: impl EventFilter for SeverityFilter
struct SourceFilter {
    source: String,
}
// TODO: impl EventFilter for SourceFilter
fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) ->
Vec<String> {
    // TODO: Filter events that pass ALL filters, format as
    // "[SEVERITY] source (FC:fault_code): message"
    todo!()
}
fn parse_event(line: &str) -> Result<DiagEvent, String> {
    // Parse "source:severity:fault_code:message"
    // Return Err for invalid input
    todo!()
}
fn main() {
    let raw_lines = vec![
        "accel_diag:Critical:67956:ECC uncorrectable error detected",
        "nic_diag:Warning:32709:Link speed degraded",
```

<a id="page-247"></a>

```
        "accel_diag:Info:10001:Self-test passed",
        "cpu_diag:Critical:55012:Thermal throttling active",
        "accel_diag:Warning:32710:PCIe link width reduced",
    ];
    // Parse all lines, collect successes and report errors
    let events: Vec<DiagEvent> = raw_lines.iter()
        .filter_map(|line| match parse_event(line) {
            Ok(e) => Some(e),
            Err(e) => { eprintln!("Parse error: {e}"); None }
        })
        .collect();
    // Apply filters: only Critical+Warning events from accel_diag
    let sev_filter = SeverityFilter { min_severity: Severity::Warning };
    let src_filter = SourceFilter { source: "accel_diag".to_string() };
    let filters: Vec<&dyn EventFilter> = vec![&sev_filter, &src_filter];
    let report = process_events(&events, &filters);
    for line in &report {
        println!("{line}");
    }
    println!("--- {} event(s) matched ---", report.len());
}
```

- [ ] Solution (click to expand)

<a id="page-248"></a>

![Source region, PDF page 248](Rust-for-C_CPP-Programmers-images/page-0248-region-0077.png)

<a id="page-249"></a>

```
// Cargo.toml
// [dependencies]
// log = "0.4"
// env_logger = "0.11"    # One of many backends
use log::{info, warn, error, debug, trace};
fn check_sensor(id: u32, temp: f64) {
    trace!("Reading sensor {id}");           // Finest granularity
    debug!("Sensor {id} raw value: {temp}"); // Development-time detail
    if temp > 85.0 {
        warn!("Sensor {id} high temperature: {temp}°C");
    }
    if temp > 95.0 {
        error!("Sensor {id} CRITICAL: {temp}°C — initiating shutdown");
    }
    info!("Sensor {id} check complete");     // Normal operation
}
fn main() {
    // Initialize the backend — typically done once in main()
    env_logger::init();  // Controlled by RUST_LOG env var
    check_sensor(0, 72.5);
    check_sensor(1, 91.0);
}
```

```
# Control log level via environment variable
RUST_LOG=debug cargo run          # Show debug and above
RUST_LOG=warn cargo run           # Show only warn and error
RUST_LOG=my_crate=trace cargo run # Per-module filtering
RUST_LOG=my_crate::gpu=debug,warn cargo run  # Mix levels
```

<a id="dest-249-5085"></a>

### [C++ comparison](<#dest-249-5085>)

C++ Rust ( log ) Notes

Format checked at `debug!("{msg}")` `%s\n", msg)` compile time

```
printf("DEBUG:
```

Backend decides `error!("...")` where output goes `"...")`

```
syslog(LOG_ERR,
```

`#ifdef DEBUG` around log calls

`trace!` / `debug!` compiled out at max\_level

Zero-cost when disabled

Custom

Universal facade, swappable backend

`log::info!("...")` — all crates use same API

```
Logger::log(level,
msg)
```

<a id="page-250"></a>

![Source region, PDF page 250](Rust-for-C_CPP-Programmers-images/page-0250-region-0078.png)

> PDF page 250: Complex table retained as a source image.

<a id="dest-250-1118"></a>

[The tracing crate — structured logging with spans](<#dest-250-1118>)

`tracing` extends `log` with structured fields and spans (timed scopes). This is especially useful for diagnostics code where you want to track context:

<a id="page-251"></a>

```
// Cargo.toml
// [dependencies]
// tracing = "0.1"
// tracing-subscriber = { version = "0.3", features = ["env-filter"] }
use tracing::{info, warn, error, instrument, info_span};
#[instrument(skip(data), fields(gpu_id = gpu_id, data_len = data.len()))]
fn run_gpu_test(gpu_id: u32, data: &[u8]) -> Result<(), String> {
    info!("Starting GPU test");
    let span = info_span!("ecc_check", gpu_id);
    let _guard = span.enter();  // All logs inside this scope include gpu_id
    if data.is_empty() {
        error!(gpu_id, "No test data provided");
        return Err("empty data".to_string());
    }
    // Structured fields — machine-parseable, not just string interpolation
    info!(
        gpu_id,
        temp_celsius = 72.5,
        ecc_errors = 0,
        "ECC check passed"
    );
    Ok(())
}
fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter("debug")  // Or use RUST_LOG env var
        .with_target(true)          // Show module path
        .with_thread_ids(true)      // Show thread IDs
        .init();
    let _ = run_gpu_test(0, &[1, 2, 3]);
}
```

Output with `tracing-subscriber` :

```
2026-02-15T10:30:00.123Z DEBUG ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}:
my_crate: Starting GPU test
2026-02-15T10:30:00.124Z  INFO ThreadId(01) run_gpu_test{gpu_id=0
data_len=3}:ecc_check{gpu_id=0}: my_crate: ECC check passed gpu_id=0
temp_celsius=72.5 ecc_errors=0
```

<a id="page-252"></a>

<a id="dest-252-0"></a>

### [\#\[instrument\] — automatic span creation](<#dest-252-0>)

The `#[instrument]` attribute automatically creates a span with the function name and its arguments:

```
use tracing::instrument;
#[instrument]
fn parse_sel_record(record_id: u16, sensor_type: u8, data: &[u8]) -> Result<(),
String> {
    // Every log inside this function automatically includes:
    // record_id, sensor_type, and data (if Debug)
    tracing::debug!("Parsing SEL record");
    Ok(())
}
// skip: exclude large/sensitive args from the span
// fields: add computed fields
#[instrument(skip(raw_buffer), fields(buf_len = raw_buffer.len()))]
fn decode_ipmi_response(raw_buffer: &[u8]) -> Result<Vec<u8>, String> {
    tracing::trace!("Decoding {} bytes", raw_buffer.len());
    Ok(raw_buffer.to_vec())
}
```

<a id="dest-252-3728"></a>

### [log vs tracing — which to use](<#dest-252-3728>)

Aspect log tracing

Complexity Simple — 5 macros Richer — spans, fields, instruments

Structured Key-value fields: `info!(gpu_id = 0,` String interpolation only data

```
"msg")
```

Yes — `#[instrument]` , Timing / spans No

```
span.enter()
```

First-class — spans propagate across Async support Basic

```
.await
```

Compatible with `log` (has a `log` Compatibility Universal facade bridge)

Simple applications, When to use libraries

Diagnostic tools, async code, observability

Recommendation: Use `tracing` for production diagnostic-style projects (diagnostic tools with structured output). Use `log` for simple libraries where you want minimal

<a id="page-253"></a>

dependencies. `tracing` includes a compatibility layer so libraries using `log` macros still work with a `tracing` subscriber.

<a id="dest-253-840"></a>

[Backend options](<#dest-253-840>)

Backend Crate Output Use Case

Development, simple CLI stderr, colored `env_logger` tools stderr, formatted Production with `tracing` `tracing-subscriber`

System syslog Linux system services `syslog` systemd journal systemd-managed services `tracing-journald`

Rotating log files Long-running daemons `tracing-appender`

OpenTelemetry Distributed tracing collector

```
tracing-
opentelemetry
```

<a id="page-254"></a>

18. [18. C++ → Rust Semantic Deep Dives](<#page-254>)

<a id="dest-254-930"></a>

## [C++ → Rust Semantic Deep Dives](<#dest-254-930>)

**What you’ll learn:** Detailed mappings for C++ concepts that don’t have obvious Rust equivalents — the four named casts, SFINAE vs trait bounds, CRTP vs associated types, and other common friction points during translation.

The sections below map C++ concepts that don’t have an obvious 1:1 Rust equivalent. These differences frequently trip up C++ programmers during translation work.

<a id="dest-254-2955"></a>

### [Casting Hierarchy: Four C++ Casts → Rust Equivalents](<#dest-254-2955>)

C++ has four named casts. Rust replaces them with different, more explicit mechanisms:

```
// C++ casting hierarchy
int i = static_cast<int>(3.14);            // 1. Numeric / up-cast
Derived* d = dynamic_cast<Derived*>(base); // 2. Runtime downcasting
int* p = const_cast<int*>(cp);              // 3. Cast away const
auto* raw = reinterpret_cast<char*>(&obj); // 4. Bit-level reinterpretation
```

C++ Cast Rust Equivalent Safety Notes

Safe but can `static_cast` `as` keyword (numeric) truncate/wrap

```
let i = 3.14_f64 a
```

`i32;` — truncates to

```
let i: i32 =
```

Safe, compile- `static_cast` `From` / `Into` (numeric, checked) time verified

`42_u8.into();` — on widens

```
                  let i: u8 =
Result
```

Safe, returns `static_cast` `TryFrom` / `TryInto` (numeric, fallible)

`300_u16.try_into()` — returns Err

Pattern matching for enums; `Any` for trait objects

`match` on enum /

`dynamic_cast` (downcast)

Safe `Any::downcast_ref`

No equivalent Rust has no way to `const_cast` cast away `&` → `&mut` in safe code. Use

<a id="page-255"></a>

![Source region, PDF page 255](Rust-for-C_CPP-Programmers-images/page-0255-region-0079.png)

> PDF page 255: Complex table retained as a source image.

```
// Rust equivalents:
// 1. Numeric casts — prefer From/Into over `as`
let widened: u32 = 42_u8.into();             // Infallible widening — always
prefer
let truncated = 300_u16 as u8;                // ⚠ Wraps to 44! Silent data loss
let checked: Result<u8, _> = 300_u16.try_into(); // Err — safe fallible conversion
// 2. Downcast: enum (preferred) or Any (when needed for type erasure)
use std::any::Any;
fn handle_any(val: &dyn Any) {
    if let Some(s) = val.downcast_ref::<String>() {
        println!("Got string: {s}");
    } else if let Some(n) = val.downcast_ref::<i32>() {
        println!("Got int: {n}");
    }
}
// 3. "const_cast" → interior mutability (no unsafe needed)
use std::cell::Cell;
struct Sensor {
    read_count: Cell<u32>,  // Mutate through &self
}
impl Sensor {
    fn read(&self) -> f64 {
        self.read_count.set(self.read_count.get() + 1); // &self, not &mut self
        42.0
    }
}
// 4. reinterpret_cast → transmute (almost never needed)
// Prefer safe alternatives:
```

`let bytes: [u8; 4] = 0x12345678_u32.to_ne_bytes();  //` ✅ `Safe` `let val = u32::from_ne_bytes(bytes);                   //` ✅ `Safe` `// unsafe { std::mem::transmute::<u32, [u8; 4]>(val) } //` ❌ `Avoid`

Guideline: In idiomatic Rust, `as` should be rare (use `From` / `Into` for widening,

`TryFrom` / `TryInto` for narrowing), `transmute` should be exceptional, and `const_cast`

<a id="page-256"></a>

has no equivalent because interior mutability types make it unnecessary.

<a id="dest-256-818"></a>

### [Preprocessor → cfg, Feature Flags, and macro\_rules\!](<#dest-256-818>)

C++ relies heavily on the preprocessor for conditional compilation, constants, and code generation. Rust replaces all of these with first-class language features.

<a id="dest-256-1732"></a>

[\#define constants → const or const fn](<#dest-256-1732>)

```
// C++
#define MAX_RETRIES 5
#define BUFFER_SIZE (1024 * 64)
#define SQUARE(x) ((x) * (x))  // Macro — textual substitution, no type safety
```

```
// Rust — type-safe, scoped, no textual substitution
const MAX_RETRIES: u32 = 5;
const BUFFER_SIZE: usize = 1024 * 64;
const fn square(x: u32) -> u32 { x * x }  // Evaluated at compile time
// Can be used in const contexts:
const AREA: u32 = square(12);  // Computed at compile time
static BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
```

<a id="dest-256-4335"></a>

[\#ifdef / \#if → \#\[cfg()\] and cfg\!()](<#dest-256-4335>)

```
// C++
#ifdef DEBUG
    log_verbose("Step 1 complete");
#endif
#if defined(LINUX) && !defined(ARM)
    use_x86_path();
#else
    use_generic_path();
#endif
```

<a id="page-257"></a>

```
// Rust — attribute-based conditional compilation
#[cfg(debug_assertions)]
fn log_verbose(msg: &str) { eprintln!("[VERBOSE] {msg}"); }
#[cfg(not(debug_assertions))]
fn log_verbose(_msg: &str) { /* compiled away in release */ }
// Combine conditions:
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn use_x86_path() { /* ... */ }
#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn use_generic_path() { /* ... */ }
// Runtime check (condition is still compile-time, but usable in expressions):
if cfg!(target_os = "windows") {
    println!("Running on Windows");
}
```

<a id="dest-257-2820"></a>

[Feature flags in Cargo.toml](<#dest-257-2820>)

```
# Cargo.toml — replace #ifdef FEATURE_FOO
[features]
default = ["json"]
json = ["dep:serde_json"]       # Optional dependency
verbose-logging = []            # Flag with no extra dependency
gpu-support = ["dep:cuda-sys"]  # Optional GPU support
```

```
// Conditional code based on feature flags:
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}
#[cfg(feature = "verbose-logging")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose-logging"))]
macro_rules! verbose {
    ($($arg:tt)*) => { }; // Compiles to nothing
}
```

<a id="page-258"></a>

<a id="dest-258-0"></a>

[\#define MACRO(x) → macro\_rules\!](<#dest-258-0>)

```
// C++ — textual substitution, notoriously error-prone
#define DIAG_CHECK(cond, msg) \
    do { if (!(cond)) { log_error(msg); return false; } } while(0)
```

```
// Rust — hygienic, type-checked, operates on syntax tree
macro_rules! diag_check {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            log_error($msg);
            return Err(DiagError::CheckFailed($msg.to_string()));
        }
    };
}
fn run_test() -> Result<(), DiagError> {
    diag_check!(temperature < 85.0, "GPU too hot");
    diag_check!(voltage > 0.8, "Rail voltage too low");
    Ok(())
}
```

C++ Preprocessor Rust Equivalent Advantage

Typed, scoped, visible to `#define PI 3.14` `const PI: f64 = 3.14;` debugger

`macro_rules!` or generic `fn`

No double-evaluation bugs

`#define MAX(a,b)`

```
((a)>(b)?(a):(b))        max<T: Ord>
```

Checked by compiler, no `#ifdef DEBUG` `#[cfg(debug_assertions)]` typo risk

Cargo manages features; dependency- aware

```
#ifdef FEATURE_X         #[cfg(feature = "x")]
```

`mod module;` + `use`

No include guards, no circular includes

```
#include
"header.h"               module::Item;
```

Each `.rs` file is a module — included exactly once

Not needed `#pragma once`

<a id="dest-258-6570"></a>

### [Header Files and \#include → Modules and use](<#dest-258-6570>)

In C++, the compilation model revolves around textual inclusion:

<a id="page-259"></a>

```
// widget.h — every translation unit that uses Widget includes this
#pragma once
#include <string>
#include <vector>
class Widget {
public:
    Widget(std::string name);
    void activate();
private:
    std::string name_;
    std::vector<int> data_;
};
```

```
// widget.cpp — separate definition
#include "widget.h"
Widget::Widget(std::string name) : name_(std::move(name)) {}
void Widget::activate() { /* ... */ }
```

In Rust, there are no header files, no forward declarations, no include guards:

```
// src/widget.rs — declaration AND definition in one file
pub struct Widget {
    name: String,         // Private by default
    data: Vec<i32>,
}
impl Widget {
    pub fn new(name: String) -> Self {
        Widget { name, data: Vec::new() }
    }
    pub fn activate(&self) { /* ... */ }
}
```

```
// src/main.rs — import by module path
mod widget;  // Tells compiler to include src/widget.rs
use widget::Widget;
fn main() {
    let w = Widget::new("sensor".to_string());
    w.activate();
}
```

C++ Rust Why it’s better

`mod foo;` in parent + `use`

No textual inclusion, no ODR violations

`#include`

```
"foo.h"             foo::Item;
```

`#pragma once` / Each `.rs` file is a module — Not needed include guards compiled once

<a id="page-260"></a>

C++ Rust Why it’s better

Forward Compiler sees entire crate; Not needed declarations order doesn’t matter `class Foo;` (incomplete type)

No separate Not needed declaration/definition split

`.h` + `.cpp` for No declaration/definition Single `.rs` file each class mismatch bugs

Always explicit — no global namespace pollution

`using`

`use`

`namespace std;`

```
std::collections::HashMap;
```

Nested

Nested `mod a { mod b { } }` or `a/b.rs`

File system mirrors module tree

```
namespace a::b
```

<a id="dest-260-2722"></a>

### [friend and Access Control → Module Visibility](<#dest-260-2722>)

C++ uses `friend` to grant specific classes or functions access to private members. Rust has no

`friend` keyword — instead, privacy is module-scoped:

```
// C++
class Engine {
    friend class Car;   // Car can access private members
    int rpm_;
    void set_rpm(int r) { rpm_ = r; }
public:
    int rpm() const { return rpm_; }
};
```

<a id="page-261"></a>

```
// Rust — items in the same module can access all fields, no `friend` needed
mod vehicle {
    pub struct Engine {
        rpm: u32,  // Private to the module (not to the struct!)
    }
    impl Engine {
        pub fn new() -> Self { Engine { rpm: 0 } }
        pub fn rpm(&self) -> u32 { self.rpm }
    }
    pub struct Car {
        engine: Engine,
    }
    impl Car {
        pub fn new() -> Self { Car { engine: Engine::new() } }
        pub fn accelerate(&mut self) {
        }
        pub fn rpm(&self) -> u32 {
        }
    }
}
fn main() {
    let mut car = vehicle::Car::new();
    car.accelerate();
    println!("RPM: {}", car.rpm()); // ✅ Public method on Car
}
```

`self.engine.rpm = 3000; //` ✅ `Same module — direct field access`

`self.engine.rpm  //` ✅ `Same module — can read private field`

`// car.engine.rpm = 9000;  //` ❌ ``Compile error: `engine` is private``

C++ Access Rust Equivalent Scope

Accessible within the same module (default, no keyword) `private` only

Use `pub(super)` for parent module No direct equivalent `protected` access

Accessible everywhere `public` `pub`

Put `Foo` in the same Module-level privacy replaces friend module

```
friend class
Foo
```

Visible within the crate but not to — `pub(crate)` external dependents

— Visible to the parent module only `pub(super)`

Visible within a specific module — `pub(in crate::path)` subtree

<a id="page-262"></a>

Key insight: C++ privacy is per-class. Rust privacy is per-module. This means you control access by choosing which types live in the same module — colocated types have full access to each other’s private fields.

<a id="dest-262-1290"></a>

### [volatile → Atomics and read\_volatile/write\_volatile](<#dest-262-1290>)

In C++, `volatile` tells the compiler not to optimize away reads/writes — typically used for memory-mapped hardware registers. Rust has no volatile keyword.

```
// C++: volatile for hardware registers
volatile uint32_t* const GPIO_REG = reinterpret_cast<volatile uint32_t*>
(0x4002'0000);
*GPIO_REG = 0x01;              // Write not optimized away
uint32_t val = *GPIO_REG;     // Read not optimized away
```

```
// Rust: explicit volatile operations — only in unsafe code
use std::ptr;
const GPIO_REG: *mut u32 = 0x4002_0000 as *mut u32;
// SAFETY: GPIO_REG is a valid memory-mapped I/O address.
unsafe {
    ptr::write_volatile(GPIO_REG, 0x01);   // Write not optimized away
    let val = ptr::read_volatile(GPIO_REG); // Read not optimized away
}
```

For concurrent shared state (the other common C++ `volatile` use), Rust uses atomics:

```
// C++: volatile is NOT sufficient for thread safety (common mistake!)
```

`volatile bool stop_flag = false;  //` ❌ `Data race — UB in C++11+`

```
// Correct C++:
std::atomic<bool> stop_flag{false};
```

<a id="page-263"></a>

```
// Rust: atomics are the only way to share mutable state across threads
use std::sync::atomic::{AtomicBool, Ordering};
static STOP_FLAG: AtomicBool = AtomicBool::new(false);
// From another thread:
STOP_FLAG.store(true, Ordering::Release);
// Check:
if STOP_FLAG.load(Ordering::Acquire) {
    println!("Stopping");
}
```

C++ Usage Rust Equivalent

`ptr::read_volatile` / `volatile` for hardware registers

```
ptr::write_volatile
```

`volatile` for thread signaling `AtomicBool` / `AtomicU32` etc.

```
std::atomic<T>                                  std::sync::atomic::AtomicT
```

```
std::atomic<T>::load(memory_order_acquire)      AtomicT::load(Ordering::Acquire)
```

<a id="dest-263-5332"></a>

### [static Variables → static, const, LazyLock, OnceLock](<#dest-263-5332>)

<a id="dest-263-5760"></a>

[Basic static and const](<#dest-263-5760>)

```
// C++
const int MAX_RETRIES = 5;                    // Compile-time constant
static std::string CONFIG_PATH = "/etc/app";  // Static init — order undefined!
```

```
// Rust
const MAX_RETRIES: u32 = 5;                   // Compile-time constant, inlined
static CONFIG_PATH: &str = "/etc/app";         // 'static lifetime, fixed address
```

<a id="page-264"></a>

<a id="dest-264-0"></a>

[The static initialization order fiasco](<#dest-264-0>)

C++ has a well-known problem: global constructors in different translation units execute in unspecified order. Rust avoids this entirely — `static` values must be compile-time constants

(no constructors).

For runtime-initialized globals, use `LazyLock` (Rust 1.80+) or `OnceLock` :

```
use std::sync::LazyLock;
// Equivalent to C++ `static std::regex` — initialized on first access, thread-
safe
static CONFIG_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[a-z]+_diag$").expect("invalid regex")
});
fn is_valid_diag(name: &str) -> bool {
    CONFIG_REGEX.is_match(name)  // First call initializes; subsequent calls are
fast
}
```

```
use std::sync::OnceLock;
// OnceLock: initialized once, can be set from runtime data
static DB_CONN: OnceLock<String> = OnceLock::new();
fn init_db(connection_string: &str) {
    DB_CONN.set(connection_string.to_string())
        .expect("DB_CONN already initialized");
}
fn get_db() -> &'static str {
    DB_CONN.get().expect("DB not initialized")
}
```

C++ Rust Notes

Both compile-time. Rust requires type annotation

```
const int X = 5;        const X: i32 = 5;
constexpr int X =                                          Rust const is always
                        const X: i32 = 5;
5;                                                         constexpr
static int count =      static COUNT: AtomicI32 =
                        AtomicI32::new(0);
static std::string
s = "hi";               LazyLock<String>
```

Mutable statics require

`0;` (file scope)

`unsafe` or atomics

No runtime constructor for simple cases

![Source region, PDF page 264](Rust-for-C_CPP-Programmers-images/page-0264-region-0080.png)

> PDF page 264: Mathematics retained as a source image.

<a id="page-265"></a>

C++ Rust Notes

Thread-safe, lazy, no init order issues

`static MyObj obj;` (complex init)

```
                        static OBJ: LazyLock<MyObj> =
                       LazyLock::new(|| { ... });
                        thread_local! { static X:
thread_local
```

Same semantics `Cell<u32> = Cell::new(0); }`

<a id="dest-265-1410"></a>

### [constexpr → const fn](<#dest-265-1410>)

C++ `constexpr` marks functions and variables for compile-time evaluation. Rust uses `const`

`fn` and `const` for the same purpose:

```
// C++
constexpr int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}
constexpr int val = factorial(5);  // Computed at compile time → 120
```

```
// Rust
const fn factorial(n: u32) -> u32 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
const VAL: u32 = factorial(5);  // Computed at compile time → 120
// Also works in array sizes and match patterns:
const LOOKUP: [u32; 5] = [factorial(1), factorial(2), factorial(3),
                           factorial(4), factorial(5)];
```

![Source region, PDF page 265](Rust-for-C_CPP-Programmers-images/page-0265-region-0081.png)

> PDF page 265: Complex table retained as a source image.

**Current limitations of** const fn (stabilized as of Rust 1.82):

<a id="page-266"></a>

- No trait methods (can’t call `.len()` on a `Vec` in const context)
- No heap allocation ( `Box::new` , `Vec::new` not const)
- No floating-point arithmetic — stabilized in Rust 1.82
- Can’t use `for` loops (use recursion or `while` with manual index)

<a id="dest-266-1358"></a>

### [SFINAE and enable\_if → Trait Bounds and where Clauses](<#dest-266-1358>)

In C++, SFINAE (Substitution Failure Is Not An Error) is the mechanism behind conditional generic programming. It is powerful but notoriously unreadable. Rust replaces it entirely with trait bounds:

```
// C++: SFINAE-based conditional function (pre-C++20)
template<typename T,
         std::enable_if_t<std::is_integral_v<T>, int> = 0>
T double_it(T val) { return val * 2; }
template<typename T,
         std::enable_if_t<std::is_floating_point_v<T>, int> = 0>
T double_it(T val) { return val * 2.0; }
// C++20 concepts — cleaner but still verbose:
template<std::integral T>
T double_it(T val) { return val * 2; }
```

<a id="page-267"></a>

```
// Rust: trait bounds — readable, composable, excellent error messages
use std::ops::Mul;
fn double_it<T: Mul<Output = T> + From<u8>>(val: T) -> T {
    val * T::from(2)
}
// Or with where clause for complex bounds:
fn process<T>(val: T) -> String
where
    T: std::fmt::Display + Clone + Send,
{
    format!("Processing: {}", val)
}
// Conditional behavior via separate impls (replaces SFINAE overloads):
trait Describable {
    fn describe(&self) -> String;
}
impl Describable for u32 {
    fn describe(&self) -> String { format!("integer: {self}") }
}
impl Describable for f64 {
    fn describe(&self) -> String { format!("float: {self:.2}") }
}
```

C++ Template Rust Equivalent Readability Metaprogramming

🟢 Clear English `std::enable_if_t<cond>` `where T: Trait`

Bound on a numeric trait `std::is_integral_v<T>` or specific types

🟢 No `_v` / `_t` suffixes

Separate `impl Trait for` SFINAE overload sets

🟢 Each impl stands alone

`ConcreteType` blocks

Specialization via trait impls

🟢 Compile-time dispatched

```
if constexpr
(std::is_same_v<T, int>)
```

🟢 Nearly identical C++20 `concept` `trait` intent

🟢 Same position, `requires` clause `where` clause similar syntax

Compilation fails deep inside template

Compilation fails at the call site with trait mismatch

🟢 No 200-line error cascades

<a id="page-268"></a>

Key insight: C++ concepts (C++20) are the closest thing to Rust traits. If you’re familiar with C++20 concepts, think of Rust traits as concepts that have been a first-class language feature since 1.0, with a coherent implementation model (trait impls) instead of duck typing.

<a id="dest-268-1462"></a>

### [std::function → Function Pointers, impl Fn, and Box&lt;dyn Fn&gt;](<#dest-268-1462>)

C++ `std::function<R(Args...)>` is a type-erased callable. Rust has three options, each with

different trade-offs:

```
// C++: one-size-fits-all (heap-allocated, type-erased)
#include <functional>
std::function<int(int)> make_adder(int n) {
    return [n](int x) { return x + n; };
}
```

```
// Rust Option 1: fn pointer — simple, no captures, no allocation
fn add_one(x: i32) -> i32 { x + 1 }
let f: fn(i32) -> i32 = add_one;
println!("{}", f(5)); // 6
// Rust Option 2: impl Fn — monomorphized, zero overhead, can capture
fn apply(val: i32, f: impl Fn(i32) -> i32) -> i32 { f(val) }
let n = 10;
let result = apply(5, |x| x + n);  // Closure captures `n`
// Rust Option 3: Box<dyn Fn> — type-erased, heap-allocated (like std::function)
fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}
let adder = make_adder(10);
println!("{}", adder(5));  // 15
// Storing heterogeneous callables (like vector<function<int(int)>>):
let callbacks: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x + 1),
    Box::new(|x| x * 2),
    Box::new(make_adder(100)),
];
for cb in &callbacks {
    println!("{}", cb(5));  // 6, 10, 105
}
```

<a id="page-269"></a>

When to use C++ Equivalent Rust Choice

Top-level function, no Function pointer `fn(Args) -> Ret` captures

Generic function `impl Fn(Args) -> Ret` Template parameter accepting callables (static dispatch)

Trait bound in `template<typename F>` `F: Fn(Args) -> Ret` generics

Stored callable, type- `Box<dyn Fn(Args) ->` `std::function<R(Args)>` erased

```
Ret>
```

Callback that mutates state

`std::function` with mutable lambda

`Box<dyn FnMut(Args) ->`

```
Ret>
```

One-shot callback `Box<dyn FnOnce(Args) -` `std::function` (moved) (consumed)

```
> Ret>
```

Performance note: `impl Fn` has zero overhead (monomorphized, like a C++ template).

`Box<dyn Fn>` has the same overhead as `std::function` (vtable + heap allocation). Prefer `impl Fn` unless you need to store heterogeneous callables.

<a id="dest-269-3982"></a>

[Container Mapping: C++ STL → Rust std::collections](<#dest-269-3982>)

C++ STL Container Rust Equivalent Notes

Nearly identical API. Rust checks bounds by default

```
std::vector<T>              Vec<T>
```

Stack-allocated `std::array<T, N>` `[T; N]` fixed-size array

Ring buffer. Efficient push/pop at both ends

```
std::deque<T>               std::collections::VecDeque<T>
```

Rarely used in Rust — `Vec` is almost always faster

```
std::list<T>                std::collections::LinkedList<T>
```

<a id="page-270"></a>

C++ STL Container Rust Equivalent Notes

Use `Vec` or No equivalent `std::forward_list<T>`

```
                                                                  VecDeque
std::unordered_map<K,
                            std::collections::HashMap<K, V>
V>
```

Uses `SipHash` by default (DoS- resistant)

B-tree; keys sorted; `K: Ord` required

```
                            std::collections::BTreeMap<K,
std::map<K, V>
                           V>
```

`T: Hash + Eq` `std::unordered_set<T>` `std::collections::HashSet<T>` required

Sorted set; `T:` `std::set<T>` `std::collections::BTreeSet<T>`

`Ord` required

Max-heap by default (same as C++)

```
std::priority_queue<T>      std::collections::BinaryHeap<T>
```

No separate stack type needed

`Vec<T>` with `.push()` / `.pop()` `std::stack<T>`

No separate queue type needed

`VecDeque<T>` with `.push_back()` / `std::queue<T>`

```
.pop_front()
```

UTF-8 guaranteed, not null-terminated

```
std::string                 String
```

Borrowed UTF- `std::string_view` `&str` 8 slice

Rust slices have been a first- class type since 1.0

`std::span<T>` (C++20) `&[T]` / `&mut [T]`

First-class syntax, destructurable

```
std::tuple<A, B, C>         (A, B, C)
```

Just a 2-element `std::pair<A, B>` `(A, B)` tuple

Use the `bitvec` crate or `[u8;`

No std equivalent `std::bitset<N>`

```
N/8]
```

<a id="page-271"></a>

Key differences:

- Rust’s `HashMap` / `HashSet` require `K: Hash + Eq` — the compiler enforces this at the type level, unlike C++ where using an unhashable key gives a template error deep in the STL
- `Vec` indexing ( `v[i]` ) panics on out-of-bounds by default. Use `.get(i)` for `Option<&T>` or iterators to avoid bounds checks entirely
- No `std::multimap` or `std::multiset` — use `HashMap<K, Vec<V>>` or `BTreeMap<K,`

```
Vec<V>>
```

<a id="dest-271-1852"></a>

[Exception Safety → Panic Safety](<#dest-271-1852>)

C++ defines three levels of exception safety (Abrahams guarantees):

![Source region, PDF page 271](Rust-for-C_CPP-Programmers-images/page-0271-region-0082.png)

> PDF page 271: Complex table retained as a source image.

<a id="dest-271-4418"></a>

[How Rust’s ownership model helps](<#dest-271-4418>)

```
// Strong guarantee for free — if file.write() fails, config is unchanged
fn update_config(config: &mut Config, path: &str) -> Result<(), Error> {
    let new_data = fetch_from_network()?; // Err → early return, config untouched
    let validated = validate(new_data)?;   // Err → early return, config untouched
    *config = validated;                   // Only reached on success (commit)
    Ok(())
}
```

In C++, achieving the strong guarantee requires manual rollback or the copy-and-swap idiom. In Rust, `?` propagation gives you the strong guarantee by default for most code.

<a id="page-272"></a>

<a id="dest-272-0"></a>

[catch\_unwind — Rust’s equivalent of catch(...)](<#dest-272-0>)

```
use std::panic;
// Catch a panic (like catch(...) in C++) — rarely needed
let result = panic::catch_unwind(|| {
    // Code that might panic
    let v = vec![1, 2, 3];
    v[10]  // Panics! (index out of bounds)
});
match result {
    Ok(val) => println!("Got: {val}"),
    Err(_) => eprintln!("Caught a panic — cleaned up"),
}
```

<a id="dest-272-2468"></a>

[UnwindSafe — marking types as panic-safe](<#dest-272-2468>)

```
use std::panic::UnwindSafe;
// Types behind &mut are NOT UnwindSafe by default — the panic may have
// left them in a partially-modified state
fn safe_execute<F: FnOnce() + UnwindSafe>(f: F) {
    let _ = std::panic::catch_unwind(f);
}
// Use AssertUnwindSafe to override when you've audited the code:
use std::panic::AssertUnwindSafe;
let mut data = vec![1, 2, 3];
let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
    data.push(4);
}));
```

C++ Exception Pattern Rust Equivalent

`return Err(MyError::...)` (preferred) or `panic!` `throw MyException()`

```
                                ("...")
                                 match result { Ok(v) => ..., Err(e) => ... }
try { } catch (const E& e)
                                or ?
catch (...)                      std::panic::catch_unwind(...)
```

`-> Result<T, E>` (errors are values, not exceptions) `noexcept`

RAII cleanup in stack unwinding

`Drop::drop()` runs during panic unwinding

```
std::uncaught_exceptions()       std::thread::panicking()
```

`-fno-exceptions` compile flag `panic = "abort"` in `Cargo.toml` \[profile\]

<a id="page-273"></a>

Bottom line: In Rust, most code uses `Result<T, E>` instead of exceptions, making error paths explicit and composable. `panic!` is reserved for bugs (like `assert!` failures), not routine errors. This means “exception safety” is largely a non-issue — the ownership system handles cleanup automatically.

<a id="dest-273-1575"></a>

### [C++ to Rust Migration Patterns](<#dest-273-1575>)

<a id="dest-273-2175"></a>

[Quick Reference: C++ → Rust Idiom Map](<#dest-273-2175>)

C++ Pattern Rust Idiom Notes

Prefer enums for closed sets

```
class Derived : public           enum Variant { A
Base                             {...}, B {...} }
                                 trait MyTrait { fn
virtual void method() = 0
                                 method(&self); }
                                 match value {
dynamic_cast<Derived*>
                                 Variant::A(data) =>
(ptr)
                                 ..., }
```

Use for open/extensible interfaces

Exhaustive, no runtime failure

Only when genuinely `vector<unique_ptr<Base>>` `Vec<Box<dyn Trait>>` polymorphic

Prefer `Box<T>` or `Rc<T>` or `Arc<T>` `shared_ptr<T>` owned values first

Arena pattern ( `Vec<T>` + `enable_shared_from_this<T>` indices)

Eliminates reference cycles entirely

`Base* m_pFramework` in every class

Pass context, don’t store pointers

`fn execute(&mut self,`

```
                                ctx: &mut Context)
                                 match result { Ok(v)
try { } catch (...) { }         => ..., Err(e) => ...
                                }
```

Or use `?` for propagation

`match` required, can’t `std::optional<T>` `Option<T>` forget None

Accepts both `String` `const std::string&` `&str` parameter parameter and `&str`

<a id="page-274"></a>

C++ Pattern Rust Idiom Notes

```
                                                        Rust enums can also
enum class Foo { A, B, C }       enum Foo { A, B, C }
                                                              carry data
```

Move is the default, no `std::move` needed

```
auto x = std::move(obj)          let x = obj;
```

One tool for everything

`cargo build / test /` CMake + make + lint

```
clippy / fmt
```

<a id="dest-274-1882"></a>

[Migration Strategy](<#dest-274-1882>)

1. Start with data types: Translate structs and enums first — this forces you to think about ownership
2. Convert factories to enums: If a factory creates different derived types, it should probably be `enum` + `match`
3. Convert god objects to composed structs: Group related fields into focused structs
4. Replace pointers with borrows: Convert `Base*` stored pointers to `&'a T` lifetime- bounded borrows
5. Use Box&lt;dyn Trait&gt; sparingly: Only for plugin systems and test mocking
6. Let the compiler guide you: Rust’s error messages are excellent — read them carefully
19. <a id="page-275"></a> [19. Rust Macros: From Preprocessor to](<#page-275>)

## [Metaprogramming](<#page-275>)

<a id="dest-275-1252"></a>

### [Rust Macros: From Preprocessor to Metaprogramming](<#dest-275-1252>)

What you’ll learn: How Rust macros work, when to use them instead of functions or generics, and how they replace the C/C++ preprocessor. By the end of this chapter you can write your own `macro_rules!` macros and understand what `#[derive(Debug)]` does under the hood.

Macros are one of the first things you encounter in Rust ( `println!("hello")` on line one) but one of the last things most courses explain. This chapter fixes that.

<a id="dest-275-3465"></a>

[Why Macros Exist](<#dest-275-3465>)

Functions and generics handle most code reuse in Rust. Macros fill the gaps where the type system can’t reach:

Need Function/Generic? Macro? Why

✅ `fn max<T: Ord>` Compute a value

Type system — handles it `(a: T, b: T) -> T`

Accept variable number of arguments

Macros accept any number of tokens

❌ Rust has no variadic functions

✅ `println!("`

```
{} {}", a, b)
```

Generate repetitive `impl` blocks

Macros generate code at compile time

❌ No way with generics alone

✅

```
macro_rules!
```

Full Rust code runs at compile time

Run code at compile time

❌ `const fn` is limited

✅ Procedural macros

Attribute macros control compilation

Conditionally ❌ ✅ `#[cfg(...)]` include code

<a id="page-276"></a>

If you’re coming from C/C++, think of macros as the _only correct replacement for the preprocessor_ — except they operate on the syntax tree instead of raw text, so they’re hygienic (no accidental name collisions) and type-aware.

For C developers: Rust macros replace `#define` entirely. There is no textual preprocessor. See [ch18](<#page-254>) for the full preprocessor → Rust mapping.

<a id="dest-276-1898"></a>

### [Declarative Macros with macro\_rules\!](<#dest-276-1898>)

Declarative macros (also called “macros by example”) are Rust’s most common macro form. They use pattern matching on syntax, similar to `match` on values.

<a id="dest-276-3000"></a>

[Basic syntax](<#dest-276-3000>)

```
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}
fn main() {
    say_hello!();  // Expands to: println!("Hello!");
}
```

The `!` after the name is what tells you (and the compiler) this is a macro invocation.

<a id="dest-276-5340"></a>

[Pattern matching with arguments](<#dest-276-5340>)

Macros match on _token trees_ using fragment specifiers:

<a id="page-277"></a>

```
macro_rules! greet {
    // Pattern 1: no arguments
    () => {
        println!("Hello, world!");
    };
    // Pattern 2: one expression argument
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}
fn main() {
    greet!();           // "Hello, world!"
    greet!("Rust");     // "Hello, Rust!"
}
```

<a id="dest-277-2415"></a>

[Fragment specifiers reference](<#dest-277-2415>)

Specifier Matches Example

Any expression `42` , `a + b` , `foo()` `$x:expr`

A type `i32` , `Vec<String>` , `&str` `$x:ty`

An identifier `foo` , `my_var` `$x:ident`

A pattern `Some(x)` , `_` , `(a, b)` `$x:pat`

![Source region, PDF page 277](Rust-for-C_CPP-Programmers-images/page-0277-region-0083.png)

> PDF page 277: Mathematics retained as a source image.

A block `$x:block` `{ println!("hi"); 42 }`

A literal `42` , `"hello"` , `true` `$x:literal`

A single token tree Anything — the wildcard `$x:tt`

An item (fn, struct, impl, etc.) `$x:item` `fn foo() {}`

<a id="dest-277-5272"></a>

### [Repetition — the killer feature](<#dest-277-5272>)

C/C++ macros can’t loop. Rust macros can repeat patterns:

<a id="page-278"></a>

```
macro_rules! make_vec {
    // Match zero or more comma-separated expressions
    ( $( $element:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($element); )*  // Repeat for each matched element
            v
        }
    };
}
fn main() {
    let v = make_vec![1, 2, 3, 4, 5];
    println!("{v:?}");  // [1, 2, 3, 4, 5]
}
```

The `$( ... ),*` syntax means “match zero or more of this pattern, separated by commas.” The `$( ... )*` in the expansion repeats the body once for each match.

This is exactly how vec\!\[\] is implemented in the standard library. The actual source

is:

```
macro_rules! vec {
    () => { Vec::new() };
    ($elem:expr; $n:expr) => { vec::from_elem($elem, $n) };
    ($($x:expr),+ $(,)?) => { <[_]>::into_vec(Box::new([$($x),+])) };
}
```

The `$(,)?` at the end allows an optional trailing comma.

<a id="dest-278-4905"></a>

[Repetition operators](<#dest-278-4905>)

![Source region, PDF page 278](Rust-for-C_CPP-Programmers-images/page-0278-region-0084.png)

> PDF page 278: Complex table retained as a source image.

<a id="dest-278-6458"></a>

### [Practical example: a hashmap\! constructor](<#dest-278-6458>)

The standard library has `vec![]` but no `hashmap!{}` . Let’s build one:

<a id="page-279"></a>

```
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}
fn main() {
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Carol" => 92,  // trailing comma OK thanks to $(,)?
    };
    println!("{scores:?}");
}
```

<a id="dest-279-2925"></a>

### [Practical example: diagnostic check macro](<#dest-279-2925>)

A pattern common in embedded/diagnostic code — check a condition and return an error:

```
use thiserror::Error;
#[derive(Error, Debug)]
enum DiagError {
    #[error("Check failed: {0}")]
    CheckFailed(String),
}
macro_rules! diag_check {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            return Err(DiagError::CheckFailed($msg.to_string()));
        }
    };
}
fn run_diagnostics(temp: f64, voltage: f64) -> Result<(), DiagError> {
    diag_check!(temp < 85.0, "GPU too hot");
    diag_check!(voltage > 0.8, "Rail voltage too low");
    diag_check!(voltage < 1.5, "Rail voltage too high");
    println!("All checks passed");
    Ok(())
}
```

#### **C/C++ comparison:**

<a id="page-280"></a>

```
// C preprocessor — textual substitution, no type safety, no hygiene
#define DIAG_CHECK(cond, msg) \
    do { if (!(cond)) { log_error(msg); return -1; } } while(0)
```

The Rust version returns a proper `Result` type, has no double-evaluation risk, and the compiler checks that `$cond` is actually a `bool` expression.

<a id="dest-280-1515"></a>

### [Hygiene: why Rust macros are safe](<#dest-280-1515>)

C/C++ macro bugs often come from name collisions:

```
// C: dangerous — `x` could shadow the caller's `x`
#define SQUARE(x) ((x) * (x))
int x = 5;
int result = SQUARE(x++);  // UB: x incremented twice!
```

Rust macros are **hygienic** — variables created inside a macro don’t leak out:

```
macro_rules! make_x {
    () => {
        let x = 42;  // This `x` is scoped to the macro expansion
    };
}
fn main() {
    let x = 10;
    make_x!();
    println!("{x}");  // Prints 10, not 42 — hygiene prevents collision
}
```

The macro’s `x` and the caller’s `x` are treated as different variables by the compiler, even

though they have the same name. This is impossible with the C preprocessor.

<a id="dest-280-5932"></a>

## [Common Standard Library Macros](<#dest-280-5932>)

You’ve been using these since chapter 1 — here’s what they actually do:

Macro What it does Expands to (simplified)

Format and print to stdout + newline

```
                                               std::io::_print(format_args!
x)                                             (...))
```

`println!("{}",`

<a id="page-281"></a>

Macro What it does Expands to (simplified)

Print to stderr + Same but to stderr newline

```
eprintln!("{}",
x)
```

Format into a `String` Allocates and returns a `String` `format!("{}", x)`

Create a `Vec` with `vec![1, 2, 3]` elements

`Vec::from([1, 2, 3])` (approximately)

Mark unfinished code `todo!()` `panic!("not yet implemented")`

Mark deliberately `unimplemented!()` `panic!("not implemented")` unimplemented code

Mark code the compiler can’t prove unreachable

```
unreachable!()                                 panic!("unreachable")
```

Panic if condition is `assert!(cond)` `if !cond { panic!(...) }` false

Panic if values aren’t Shows both values on failure `assert_eq!(a, b)` equal

Print expression + value to stderr, return value

```
                                               eprintln!("[file:line] expr =
dbg!(expr)
                                              {:#?}", &expr); expr
```

Embed file contents as `&str` at compile time

```
include_str!
("file.txt")
```

Reads file during compilation

Embed file contents as `&[u8]` at compile time

```
include_bytes!
("data.bin")
```

Reads file during compilation

Compile-time `true` or `false` based on target `cfg!(condition)` condition as a `bool`

Read environment variable at compile time

Fails compilation if not set

`env!("VAR")`

Concatenate literals at `"ab"` compile time

```
concat!("a",
"b")
```

<a id="page-282"></a>

<a id="dest-282-0"></a>

### [dbg\! — the debugging macro you’ll use daily](<#dest-282-0>)

```
fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {     // Prints: [src/main.rs:2] n <= 1 = false
        dbg!(1)           // Prints: [src/main.rs:3] 1 = 1
    } else {
        dbg!(n * factorial(n - 1))  // Prints intermediate values
    }
}
fn main() {
    dbg!(factorial(4));   // Prints all recursive calls with file:line
}
```

`dbg!` returns the value it wraps, so you can insert it anywhere without changing program behavior. It prints to stderr (not stdout), so it doesn’t interfere with program output. Remove all dbg\! calls before committing code.

<a id="dest-282-2970"></a>

### [Format string syntax](<#dest-282-2970>)

Since `println!` , `format!` , `eprintln!` , and `write!` all use the same format machinery, here’s the quick reference:

```
let name = "sensor";
let value = 3.14159;
let count = 42;
println!("{name}");                    // Variable by name (Rust 1.58+)
println!("{}", name);                  // Positional
println!("{value:.2}");                // 2 decimal places: "3.14"
println!("{count:>10}");               // Right-aligned, width 10: "        42"
println!("{count:0>10}");              // Zero-padded: "0000000042"
println!("{count:#06x}");              // Hex with prefix: "0x002a"
println!("{count:#010b}");             // Binary with prefix: "0b00101010"
println!("{value:?}");                 // Debug format
println!("{value:#?}");                // Pretty-printed Debug format
```

For C developers: Think of this as a type-safe `printf` — the compiler checks that `{:.2}` is applied to a float, not a string. No `%s` / `%d` format mismatch bugs.

For C++ developers: This replaces `std::cout << std::fixed << std::setprecision(2)`

`<< value` with a single readable format string.

<a id="page-283"></a>

<a id="dest-283-0"></a>

## [Derive Macros](<#dest-283-0>)

You’ve seen `#[derive(...)]` on nearly every struct in this book:

```
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

`#[derive(Debug)]` is a derive macro — a special kind of procedural macro that generates trait implementations automatically. Here’s what it produces (simplified):

```
// What #[derive(Debug)] generates for Point:
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
```

Without `#[derive(Debug)]` , you’d have to write that `impl` block by hand for every struct.

<a id="dest-283-4132"></a>

### [Commonly derived traits](<#dest-283-4132>)

![Source region, PDF page 283](Rust-for-C_CPP-Programmers-images/page-0283-region-0085.png)

> PDF page 283: Complex table retained as a source image.

<a id="page-284"></a>

Derive What it generates When to use

`serde::Serialize` /

JSON/TOML/etc. serialization

Data types that cross API boundaries

```
Deserialize
```

<a id="dest-284-952"></a>

### [The derive decision tree](<#dest-284-952>)

```
Should I derive it?
  │
  ├── Does my type contain only types that implement the trait?
  │     ├── Yes → #[derive] will work
  │     └── No  → Write a manual impl (or skip it)
  │
  └── Will users of my type reasonably expect this behavior?
        ├── Yes → Derive it (Debug, Clone, PartialEq are almost always reasonable)
        └── No  → Don't derive (e.g., don't derive Copy for a type with a file
handle)
```

C++ comparison: `#[derive(Clone)]` is like auto-generating a correct copy constructor. `#`

`[derive(PartialEq)]` is like auto-generating `operator==` that compares each field — something C++20’s `= default` spaceship operator finally provides.

<a id="dest-284-4350"></a>

## [Attribute Macros](<#dest-284-4350>)

Attribute macros transform the item they’re attached to. You’ve already used several:

```
#[test]                    // Marks a function as a test
fn test_addition() {
    assert_eq!(2 + 2, 4);
}
#[cfg(target_os = "linux")] // Conditionally includes this function
fn linux_only() { /* ... */ }
#[derive(Debug)]            // Generates Debug implementation
struct MyType { /* ... */ }
#[allow(dead_code)]         // Suppresses a compiler warning
fn unused_helper() { /* ... */ }
#[must_use]                 // Warn if return value is discarded
fn compute_checksum(data: &[u8]) -> u32 { /* ... */ }
```

<a id="page-285"></a>

Common built-in attributes:

![Source region, PDF page 285](Rust-for-C_CPP-Programmers-images/page-0285-region-0086.png)

> PDF page 285: Complex table retained as a source image.

For C/C++ developers: Attributes replace a mix of preprocessor directives ( `#pragma` ,

`__attribute__((...))` ), and compiler-specific extensions. They’re part of the language grammar, not bolted-on extensions.

<a id="dest-285-4688"></a>

### [Procedural Macros (Conceptual Overview)](<#dest-285-4688>)

Procedural macros (“proc macros”) are macros written as separate Rust programs that run at compile time and generate code. They’re more powerful than `macro_rules!` but also more complex.

There are three kinds:

Kind Syntax Example What it does

Parses custom syntax, generates Rust code

Function- like

```
                       sql!(SELECT * FROM
my_macro!(...)
                      users)
```

Generates trait impl from struct definition

```
#
                       #[derive(Serialize)]
[derive(MyTrait)]
```

Derive

<a id="page-286"></a>

Kind Syntax Example What it does

```
                                      #[tokio::main] , #
Attribute      #[my_attr]
                                     [instrument]
```

Transforms the annotated item

<a id="dest-286-952"></a>

[You’ve already used proc macros](<#dest-286-952>)

- `#[derive(Error)]` from `thiserror` — generates `Display` and `From` impls for error enums
- `#[derive(Serialize, Deserialize)]` from `serde` — generates serialization code
- `#[tokio::main]` — transforms `async fn main()` into a runtime setup + block\_on
- `#[test]` — registered by the test harness (built-in proc macro)

<a id="dest-286-2535"></a>

[When to write your own proc macro](<#dest-286-2535>)

You likely won’t need to write proc macros during this course. They’re useful when:

- You need to inspect struct fields/enum variants at compile time (derive macros)
- You’re building a domain-specific language (function-like macros)
- You need to transform function signatures (attribute macros)

For most code, `macro_rules!` or plain functions are sufficient.

C++ comparison: Procedural macros fill the role that code generators, template metaprogramming, and external tools like `protoc` fill in C++. The difference is that proc macros are part of the cargo build pipeline — no external build steps, no CMake custom commands.

<a id="page-287"></a>

<a id="dest-287-0"></a>

## [When to Use What: Macros vs Functions vs Generics](<#dest-287-0>)

```
Need to generate code?
  │
  ├── No → Use a function or generic function
  │         (simpler, better error messages, IDE support)
  │
  └── Yes ─┬── Variable number of arguments?
            │     └── Yes → macro_rules! (e.g., println!, vec!)
            │
            ├── Repetitive impl blocks for many types?
            │     └── Yes → macro_rules! with repetition
            │
            ├── Need to inspect struct fields?
            │     └── Yes → Derive macro (proc macro)
            │
            ├── Need custom syntax (DSL)?
            │     └── Yes → Function-like proc macro
            │
            └── Need to transform a function/struct?
                  └── Yes → Attribute proc macro
```

**General guideline:** If a function or generic can do it, don’t use a macro. Macros have worse error messages, no IDE auto-complete inside the macro body, and are harder to debug.

<a id="dest-287-4162"></a>

## [Exercises](<#dest-287-4162>)

<a id="dest-287-4762"></a>

### [🟢 Exercise 1: min\! macro](<#dest-287-4762>)

Write a `min!` macro that:

- `min!(a, b)` returns the smaller of two values
- `min!(a, b, c)` returns the smallest of three values
- Works with any type that implements `PartialOrd`

Hint: You’ll need two match arms in your `macro_rules!` .

- [ ] Solution (click to expand)

<a id="page-288"></a>

<a id="dest-288-0"></a>

### [🟡 Exercise 2: hashmap\! from scratch](<#dest-288-0>)

Without looking at the example above, write a `hashmap!` macro that:

- Creates a `HashMap` from `key => value` pairs
- Supports trailing commas
- Works with any hashable key type

Test with:

```
let m = hashmap! {
    "name" => "Alice",
    "role" => "Engineer",
};
assert_eq!(m["name"], "Alice");
assert_eq!(m.len(), 2);
```

- [ ] Solution (click to expand)

<a id="dest-288-3188"></a>

### [🟡 Exercise 3: assert\_approx\_eq\! for floating-point comparison](<#dest-288-3188>)

Write a macro `assert_approx_eq!(a, b, epsilon)` that panics if `|a - b| > epsilon` . This is useful for testing floating-point calculations where exact equality fails.

Test with:

```
assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);        // Should pass
assert_approx_eq!(3.14159, std::f64::consts::PI, 1e-4); // Should pass
// assert_approx_eq!(1.0, 2.0, 0.5);              // Should panic
```

- [ ] Solution (click to expand)

<a id="dest-288-5498"></a>

### [🔴 Exercise 4: impl\_display\_for\_enum\!](<#dest-288-5498>)

Write a macro that generates a `Display` implementation for simple C-like enums. Given:

```
impl_display_for_enum! {
    enum Color {
        Red => "red",
        Green => "green",
        Blue => "blue",
    }
}
```

<a id="page-289"></a>

It should generate both the `enum Color { Red, Green, Blue }` definition AND the `impl` `Display for Color` that maps each variant to its string.

Hint: You’ll need both `$( ... ),*` repetition and multiple fragment specifiers.

- [ ] Solution (click to expand)
