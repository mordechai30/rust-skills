Source: [https://www.ardanlabs.com/blog/2024/10/transitioning-from-c-and-c++-to-rust.html](<https://www.ardanlabs.com/blog/2024/10/transitioning-from-c-and-c++-to-rust.html>)

![Ardan Labs](<transitioning-from-c-and-c-to-rusthtml/a5d75493ef869a36e2bf2b25.svg>)

<a id="title"></a>

# Transitioning from C and C++ to Rust: A Beginner’s Guide

![Ardan Labs photo](<transitioning-from-c-and-c-to-rusthtml/51e5fd520aa0b28a6d88e6c1.jpeg>)

Ardan Labs

Published October 30, 2024
-
9 min read

<a id="introduction"></a>

### Introduction:

As you embark on the journey from C or C++ to Rust, you’ll discover a world of exciting possibilities. Rust’s emphasis on safety, concurrency, and performance can significantly enhance your programming toolkit. This beginner’s guide on transitioning from C and C++ to Rust will provide a structured approach to making that transition, addressing essential concepts and practical applications. Let’s dive in!

<a id="step-1-understanding-rusts-ownership-model"></a>

### Step 1: Understanding Rust’s Ownership Model

**Concept Overview:**
Rust’s ownership model is its most distinctive feature. Unlike C and C++, where you have pointers and manual memory management, Rust uses a system of ownership with rules that the compiler checks at compile time.

**Key Concepts:**

- **Ownership:** Every value in Rust has a single owner (variable). When the owner goes out of scope, Rust automatically cleans up the memory.
- **Borrowing:** You can temporarily lend a value without giving up ownership.
- **References:** Immutable (default) and mutable references allow controlled access to data.

<a id="example-1-move-by-default"></a>

## Example 1: Move by Default

Rust parameters are “move by default”. This can be surprising, coming from C or C++. This will not compile:

```
fn example(s: String) {
	// Insert code here
}

fn main() {
	let s = "Hello".to_string();
	example(s);
	println!("{s}");
}
```

Equivalent C++ code would leave you scratching your head due to undefined behavior:

```
#include <string>
#include <iostream>

void example(std::string s) {
	std::cout << "[" << s << "]" << "\n";
}

int main() {
	std::string s("hello");
	example(std::move(s));
	std::cout << "[" << s << "]" << "\n";
}
```

The example prints “[hello][]”. Rust moves are destructive, allowing Rust to ensure that ownership is always preserved.

<a id="example-2-borrowing"></a>

## Example 2: Borrowing

```
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1; // Borrowing
    println!("{}", s2); // Valid
    // println!("{}", s1); // Also valid, as s1 is still in scope.
}
```

Borrowing is a lot like a reference in C++.

<a id="example-3-use-after-free"></a>

## Example 3: Use After Free

The following Rust code will not compile:

```
fn example(s: &String) {
	// Insert code here
}

fn main() {
	let s = "Hello".to_string();
	example(&s);
	std::mem::drop(s);
	println!("{s}");
}
```

Rust’s ownership system - via the borrow checker - can deduce that `s` is no longer valid, and prevents a use-after-free bug. Conversely, this C++ code compiles:

```
#include <string>
#include <iostream>

void example(std::string *s) {
	std::cout << "[" << *s << "]" << "\n";
}

int main() {
	std::string *s = new std::string("hello");
	example(s);
	delete s;
	std::cout << "[" << *s << "]" << "\n";
}
```

The C++ example crashes with a segmentation fault.

**Why It Matters:**
This model prevents common bugs like double free and memory leaks, encouraging you to think about data ownership and lifetimes early in the development process.

---

<a id="step-2-exploring-the-rust-type-system"></a>

### Step 2: Exploring the Rust Type System

**Concept Overview:**
Rust’s type system is robust and helps catch errors at compile time. This includes features like algebraic data types (enums), pattern matching, and generics.

**Key Concepts:**

- **Static Typing:** All variables must have a type known at compile time.
- **Enums and Pattern Matching:** Powerful tools for expressing complex data types.
- **Generics:** Allows code to be more flexible and reusable.

<a id="example"></a>

## Example:

```
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
    }
}
```

**Why It Matters:**
A strong type system reduces runtime errors and clarifies your code’s intent. It also encourages better documentation and understanding of data flows.

---

<a id="step-3-error-handling-with-result-and-option-in-rust"></a>

### Step 3: Error Handling with `Result` and `Option` in Rust

**Concept Overview:**
In Rust, error handling is built into the type system with `Result` and `Option` types, replacing traditional error codes and exceptions.

**Key Concepts:**

- **`Result<T, E>`:** Represents either a success (`T`) or an error (`E`).
- **`Option<T>`:** Represents an optional value that can be `Some(T)` or `None`.

<a id="example-1"></a>

## Example:

```
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}
```

**Why It Matters:**
By forcing you to handle errors explicitly, Rust enhances code reliability and makes the handling of edge cases more apparent. Dereferences of null pointers are impossible in safe Rust - the type system requires that you both specify and acknowledge if a value is optional.

---

<a id="step-4-concurrency-in-rust-without-data-races"></a>

### Step 4: Concurrency in Rust without Data Races

**Concept Overview:**
Rust provides built-in mechanisms to handle concurrency safely, primarily through its ownership model.

**Key Concepts:**

- **Data Races:** Rust’s compile-time checks ensure that data is either mutable or shared, but not both.
- **Threads and Channels:** Rust’s standard library supports multi-threading and communication between threads through channels.

<a id="example-2"></a>

## Example:

```
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread").unwrap();
    });

    println!("{}", rx.recv().unwrap());
}
```

**Example 2: A Data Race**
The following C++ code compiles with no errors:

```
#include <thread>
#include <iostream>
#include <vector>

int main() {
	int n = 0;
	std::vector<std::thread> handles = {};
	for(int i=0; i<5; i++) {
    		handles.push_back(std::thread([&n]() {
        	for (int i=0; i<1000000; i++)
            	n++;
    	}));
	}
	for (auto& h : handles) {
    		h.join();
	}
	std::cout << n << "\n";
}
```

Every execution gives a different result. Equivalent Rust code will not compile:

```
fn main() {
	let mut n = 0;
	std::thread::scope(|scope| {
    	for i in 0..5 {
        	scope.spawn(|| {
            	for i in 0..1_000_000 {
                		n += 1;
            	}
        	});
    	}
	});
	println!("{n}");
}
```

**Why It Matters:**
Rust’s approach to concurrency allows you to write highly concurrent applications without the fear of data races, a common source of bugs in C and C++.

---

<a id="step-5-rust-memory-safety-without-garbage-collection"></a>

### Step 5: Rust Memory Safety Without Garbage Collection

**Concept Overview:**
Rust achieves memory safety through its ownership system without a garbage collector, allowing for predictable performance.

**Key Concepts:**

- **Stack vs Heap:** Understanding where data lives and how Rust handles memory allocation.
- **Smart Pointers:** Use of `Box`, `Rc`, and `Arc` to manage ownership and reference counting.

<a id="example-3"></a>

## Example:

```
fn main() {
    let b = Box::new(5); // Box allocates memory on the heap
    println!("{}", b); // Automatically deallocated when `b` goes out of scope
}
```

The concept should be familiar to C++ programmers: Rust safety is built on RAII (Resource Acquisition is Initialization).

**Why It Matters:**
This model combines the efficiency of manual memory management with the safety of automated systems, providing the best of both worlds.

---

<a id="step-6-structs-and-traits-for-abstraction-in-rust"></a>

### Step 6: Structs and Traits for Abstraction in Rust

**Concept Overview:**
Rust allows you to define custom data types (structs) and behavior (traits), enabling polymorphism and code reuse.

**Key Concepts:**

- **Structs:** Define complex data types.
- **Traits:** Define shared behavior; similar to interfaces in C++.

<a id="example-4"></a>

## Example:

```
struct Dog;

trait Bark {
    fn bark(&self);
}

impl Bark for Dog {
    fn bark(&self) {
        println!("Woof!");
    }
}

fn main() {
    let dog = Dog;
    dog.bark();
}
```

**Why It Matters:**
This powerful abstraction lets you define clear interfaces for your types, promoting code organization and reusability. When combined with generics, it allows for very powerful abstractions.

---

<a id="step-7-effective-use-of-crates-in-rust"></a>

### Step 7: Effective Use of Crates in Rust

**Concept Overview:**
Rust has a vibrant ecosystem of libraries (crates) that can be easily integrated into your projects via Cargo, its package manager.

**Key Concepts:**

- **Cargo:** The build system and package manager for Rust.
- **Crates.io:** The central repository for Rust libraries.

<a id="example-5"></a>

## Example:

To include an external crate, simply add it to your `Cargo.toml` file:

```
[dependencies]
serde = "1.0"
```

Your project now supports serialization and deserialization. You can further refine with feature flags:

```
[dependencies]
serde = { version = "1.0", features = [ “derive” ] }
```

You can now decorate structures with `[Serializable]` and/or `[Deserializable]` for automatic code generation.

**Why It Matters:**
Leveraging existing libraries can accelerate development, allowing you to focus on your core project without reinventing the wheel.

---

<a id="step-8-using-macros-for-metaprogramming-in-rust"></a>

### Step 8: Using Macros for Metaprogramming in Rust

**Concept Overview:**
Rust supports macros that enable you to write code that writes other code, facilitating DRY (Don’t Repeat Yourself) principles.

**Key Concepts:**

- **Declarative Macros:** `macro_rules!` for pattern-based code generation.
- **Procedural Macros:** More advanced, enabling complex code transformations.

<a id="example-6"></a>

## Example:

```
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!(); // Expands to println!("Hello, world!");
}
```

**Why It Matters:**
Macros can reduce boilerplate and improve the maintainability of your code, particularly in larger projects. Unlike C and C++ `#define`, Rust macros are not text substitution.

---

<a id="step-9-embracing-the-ecosystem-tooling-and-community-in-rust"></a>

### Step 9: Embracing the Ecosystem: Tooling and Community in Rust

**Concept Overview:**
Rust has excellent tooling support, including IDE integrations, linters, and testing frameworks that enhance the development experience.

**Key Concepts:**

- **Rustfmt:** Tool for formatting Rust code.
- **Clippy:** A linter for catching common mistakes.
- **Cargo Test:** Built-in support for testing your code.

<a id="example-7"></a>

## Example:

Run the following commands to format and test your code:

```
cargo fmt
cargo test
```

**Why It Matters:**
Robust tooling allows you to maintain high code quality and adhere to best practices, which is essential for collaborative projects.

---

<a id="step-10-building-a-strong-understanding-of-lifetimes-in-rust"></a>

### Step 10: Building a Strong Understanding of Lifetimes in Rust

**Concept Overview:**
Lifetimes are Rust’s way of ensuring that references are valid for as long as they are needed, preventing dangling references.

**Key Concepts:**

- **Lifetime Annotations:** Indicate how long references should be valid.
- **Static Lifetime:** A special lifetime for values that live for the entire duration of the program.

<a id="example-8"></a>

## Example:

```
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

**Why It Matters:**
Understanding lifetimes is crucial for safe code. It ensures that your references are used correctly, which is a common source of confusion when transitioning from C and C++. Lifetime elision - covered in a future article - means that you rarely have to explicitly name a lifetime.

---

<a id="summary"></a>

### Summary

While this Transitioning from C and C++ to Rust tutorial can be a bit long for some, it can also be an exhilarating journey filled with learning and growth. Here’s a quick recap of the key points:

- **Ownership Model:** Understand ownership, borrowing, and references.
- **Type System:** Leverage static typing, enums, pattern matching, and generics.
- **Error Handling:** Utilize `Result` and `Option` for robust error management.
- **Concurrency:** Write safe concurrent code without data races.
- **Memory Safety:** Manage memory effectively without garbage collection.
- **Structs and Traits:** Use these for creating abstractions and polymorphism.
- **Crates:** Take advantage of the Rust ecosystem and Cargo.
- **Macros:** Implement metaprogramming to reduce boilerplate.
- **Tooling:** Embrace Rust’s tools for code quality and testing.
- **Lifetimes:** Master lifetimes for safe reference handling.

With the concepts covered in our Transitioning from C and C++ to Rust tutorial in hand, you’re well on your way to becoming proficient in Rust, capable of tackling complex, latency-sensitive systems with confidence and flair. Happy coding!
