Source PDF: <file:///Users/motyzas/Downloads/C-to-Rust/Rust_for_C%2B%2B_Developers.pdf>

<a id="page-1"></a>

![Source region, PDF page 1](Rust_for_C%2B%2B_Developers-images/page-0001-region-0001.png)

<a id="page-2"></a>

![Source region, PDF page 2](Rust_for_C%2B%2B_Developers-images/page-0002-region-0005.png)

![Source region, PDF page 2](Rust_for_C%2B%2B_Developers-images/page-0002-region-0006.png)

- Developer, Trainer, Author and Speaker
- Book author

![Source region, PDF page 2](Rust_for_C%2B%2B_Developers-images/page-0002-region-0003.png)

    - “Windows Kernel Programming” (2019)
    - “Windows Internals 7th edition, Part 1” (co-author, 2017)
    - “WPF 4.5 Cookbook” (2012)

![Source region, PDF page 2](Rust_for_C%2B%2B_Developers-images/page-0002-region-0002.png)

- Pluralsight author

![Source region, PDF page 2](Rust_for_C%2B%2B_Developers-images/page-0002-region-0004.png)

- Author of several open-source tools

([http://github.com/zodiacon](<http://github.com/zodiacon>)) [http://scorpiosoftware.net](<http://scorpiosoftware.net/>)

- Blogs: [http://blogs.microsoft.co.il/pavely](<http://blogs.microsoft.co.il/pavely>),

<a id="page-3"></a>

![Source region, PDF page 3](Rust_for_C%2B%2B_Developers-images/page-0003-region-0007.png)

<a id="page-4"></a>

![Source region, PDF page 4](Rust_for_C%2B%2B_Developers-images/page-0004-region-0009.png)

![Source region, PDF page 4](Rust_for_C%2B%2B_Developers-images/page-0004-region-0010.png)

- Performance
    - Rust is blazingly fast and memory-efficient
- Reliability
    - Rust’s rich type system and ownership model

guarantee memory-safety and thread-safety with useful error messages, and top-notch tooling

- Productivity
    - Rust has great documentation, a friendly compiler
- What’s not to like?

![Source region, PDF page 4](Rust_for_C%2B%2B_Developers-images/page-0004-region-0008.png)

<a id="page-5"></a>

![Source region, PDF page 5](Rust_for_C%2B%2B_Developers-images/page-0005-region-0012.png)

![Source region, PDF page 5](Rust_for_C%2B%2B_Developers-images/page-0005-region-0011.png)

- Primitive types
    - Primitive types (+associated methods)
- Structs / classes
    - Structs
- Enumerations
    - Enumerations
- Unions
    - Enumerations\!
- (Interfaces)
    - Traits
- Inheritance
    - Trait Inheritance
- Polymorphism
    - Polymorphism
- Attributes
    - Traits\!
- Templates
    - Generics

<a id="page-6"></a>

![Source region, PDF page 6](Rust_for_C%2B%2B_Developers-images/page-0006-region-0013.png)

![Source region, PDF page 6](Rust_for_C%2B%2B_Developers-images/page-0006-region-0014.png)

- Single owner or shared
        - Single owner

ownership construction mean “copy”

        - Explicit
- Developer managed
        - Compiler enforced
- Assignment and copy
            - a.k.a. “Borrow Checker”
        - Assignment means “move”
    - Unless R-value provided or
            - Unless type implements `Copy` trait

`std::move` used explicitly (and there is a move ctor/assignment)

        - Borrowing

<a id="page-7"></a>

![Source region, PDF page 7](Rust_for_C%2B%2B_Developers-images/page-0007-region-0015.png)

![Source region, PDF page 7](Rust_for_C%2B%2B_Developers-images/page-0007-region-0016.png)

```
vector<int> v1{ 1, 2, 3 };                  let v1 = vec![1, 2, 3];
auto v2 = v1;                               let v2 = v1;
                                                       Running "cargo build":
auto v3 = v1;                               let v3 = v1; Compiling hello v0.1.0
                                                       error[E0382]: use of moved value: `v1`
                                                              --> src\main.rs:18:14
cout << v1.size() << " " << v2.size()
                                                                                                                       |
                                                              16 | let v1 = vec![1, 2, 3];
                                              let v1 = vec![1, 2, 3];
3 3 3                                         let v2 = v1.clone();
                                                       does not implement the `Copy` trait
                                              let v3 = v1.clone();
                                                       17 | let v2 = v1;
                                                       | -- value moved here
                                                       18 | let v3 = v1; println!("{} {} {}",
                                                       | ^^ value used here after move
```

`println!("{} {} {}",`

`<< " " << v3.size() << endl;`

`v1.len(), v2.len(), v3.len());`

\| -- move occurs because \`v1\` has type \`std::vec::Vec&lt;i32&gt;\`, which

`v1.len(), v2.len(), v3.len());`

<a id="page-8"></a>

![Source region, PDF page 8](Rust_for_C%2B%2B_Developers-images/page-0008-region-0017.png)

![Source region, PDF page 8](Rust_for_C%2B%2B_Developers-images/page-0008-region-0018.png)

```
fn greet(s: String) {                               fn greet(s: &String) {
    println!("Hello, {}!", s);                          println!("Hello, {}!", s);
   error[E0382]: borrow of moved value: `name`
}                                                   }
   --> src\main.rs:41:31
      |
   39 | let name = String::from("Pavel");
   | ---- move occurs because `name` has type let name = String::from("Pavel"); let name = String::from("Pavel");
    greet(name);                                        greet(&name);
   40 | greet(name); println!("Hello again, {}!", name); println!("Hello again, {}!", name);
}  | ---- value moved here                          }
    41 | println!("Hello again, {}!", name);
   | ^^^^ value borrowed here after move
```

`fn main() {`

`fn main() {`

\`std::string::String\`, which does not implement the \`Copy\` trait

<a id="page-9"></a>

![Source region, PDF page 9](Rust_for_C%2B%2B_Developers-images/page-0009-region-0019.png)

![Source region, PDF page 9](Rust_for_C%2B%2B_Developers-images/page-0009-region-0020.png)

```
                                                           error[E0502]: cannot borrow `v` as mutable because it is also
vector<string> v;                        let mut v = vec![];
                                                           borrowed as immutable
v.push_back("hello");                    v.push("hello");
                                                                  --> src\main.rs:12:5
                                                                                                                               |
auto& hello = v[0];                      let hello = &v[0];       9 | let hello = &v[0];
cout << hello.c_str() << endl;           println!("{}", hello); | - immutable borrow occurs here
                                                                                                                      ...
                                                                  12 | v.push("Rust");
v.push_back("C++");                      v.push("Rust");
                                                                      | ^^^^^^^^^^^^^^ mutable borrow occurs here
cout << hello.c_str() << endl;           println!("{}", hello);
                                                                                                   13 | println!("{}", hello);
                                                           | ----- immutable borrow later used here
```

<a id="page-10"></a>

![Source region, PDF page 10](Rust_for_C%2B%2B_Developers-images/page-0010-region-0021.png)

![Source region, PDF page 10](Rust_for_C%2B%2B_Developers-images/page-0010-region-0022.png)

- `unique_ptr<T>`
        - `Box<T>`
- `shared_ptr<T>`
        - `Rc<T>, Arc<T>`
- References
        - References (borrowing)
- Default is non-const
        - Default is immutable
    - Add `const` to declaration
            - Add `mut` to declaration to mutate
        - Multiple immutable references allowed
        - Mutable reference means no other

references can exist at that scope

<a id="page-11"></a>

![Source region, PDF page 11](Rust_for_C%2B%2B_Developers-images/page-0011-region-0023.png)

<a id="page-12"></a>

![Source region, PDF page 12](Rust_for_C%2B%2B_Developers-images/page-0012-region-0024.png)

![Source region, PDF page 12](Rust_for_C%2B%2B_Developers-images/page-0012-region-0025.png)

## •A set of named values

```
enum class Season {                   auto s = Season::Summer;
    Winter,
    Spring
    Summer,
    Fall
};
```

<a id="page-13"></a>

![Source region, PDF page 13](Rust_for_C%2B%2B_Developers-images/page-0013-region-0026.png)

![Source region, PDF page 13](Rust_for_C%2B%2B_Developers-images/page-0013-region-0027.png)

- More than just named values (closer to C++ union)
- Can be generic

```
                                                     enum TurtleCommand {
                                                         Forward(f32),
                                                         Backwards(f32),
                                                         Rotate(f32),
enum Season {
                                                         RotateRight,
    Winter,
                                                         RotateLeft,
    Spring,
                                                         PenColor { r: u8, g: u8, b: u8 }
    Summer,
                                                     }
    Fall     enum Result<T, E> {
}                Ok(T),
                 Err(E)
             }
                          enum Option<T> {
                             Some(T),
                             None
                          }
```

- Can have methods

<a id="page-14"></a>

![Source region, PDF page 14](Rust_for_C%2B%2B_Developers-images/page-0014-region-0028.png)

<a id="page-15"></a>

![Source region, PDF page 15](Rust_for_C%2B%2B_Developers-images/page-0015-region-0029.png)

![Source region, PDF page 15](Rust_for_C%2B%2B_Developers-images/page-0015-region-0030.png)

•Somewhat similar to C++ interfaces •Some similar to attributes •Basis for polymorphism •Can inherit from other traits •Some syntactic sugar in Rust is based on traits

- Abstract class with pure virtual functions

<a id="page-16"></a>

![Source region, PDF page 16](Rust_for_C%2B%2B_Developers-images/page-0016-region-0031.png)

![Source region, PDF page 16](Rust_for_C%2B%2B_Developers-images/page-0016-region-0032.png)

```
fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
       if item > largest {
           largest = item;
       }
    }
    largest
}
let nums = vec![12, 33, 45, 3, 13, 40];
println!("largest: {}", largest(&nums));
```

<a id="page-17"></a>

![Source region, PDF page 17](Rust_for_C%2B%2B_Developers-images/page-0017-region-0033.png)

<a id="page-18"></a>

![Source region, PDF page 18](Rust_for_C%2B%2B_Developers-images/page-0018-region-0034.png)

![Source region, PDF page 18](Rust_for_C%2B%2B_Developers-images/page-0018-region-0035.png)

- Large ecosystem
    - Fast growing ecosystem
- The boost libraries
    - Built-in package manager (Cargo)
- Many other libraries out there
    - Each package is a “Crate”
- No single repository
    - Central crates repository (crates.io)
- (Microsoft has Nuget)
    - Easy to use and consistent

<a id="page-19"></a>

![Source region, PDF page 19](Rust_for_C%2B%2B_Developers-images/page-0019-region-0036.png)

<a id="page-20"></a>

![Source region, PDF page 20](Rust_for_C%2B%2B_Developers-images/page-0020-region-0037.png)

![Source region, PDF page 20](Rust_for_C%2B%2B_Developers-images/page-0020-region-0038.png)

- Rust and C++ complete for (roughly) the same space
    - Both are native, statically typed, emphasize zero cost

abstraction, stack over heap, etc.

- Rust has unique model for safety
    - “Borrow checker”
- C++ leaves safety to developers
    - Does provide types to help
- Rust supports pattern matching and functional style
    - Surprisingly rich in functionality and libraries
- Give rust a try\!

<a id="page-21"></a>

![Source region, PDF page 21](Rust_for_C%2B%2B_Developers-images/page-0021-region-0039.png)
