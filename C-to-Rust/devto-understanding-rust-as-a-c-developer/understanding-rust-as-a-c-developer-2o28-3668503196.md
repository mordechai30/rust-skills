Source: [https://dev.to/daaitch/understanding-rust-as-a-c-developer-2o28](<https://dev.to/daaitch/understanding-rust-as-a-c-developer-2o28>)

<a id="article-show-container"></a>

<a id="main-title"></a>

[![Cover image for Understanding Rust as a C++ developer](<understanding-rust-as-a-c-developer-2o28/732eddf4bc126a9e7fdccbc0.webp>)](https://media2.dev.to/dynamic/image/width=1000,height=420,fit=cover,gravity=auto,format=auto/https%3A%2F%2Fdev-to-uploads.s3.amazonaws.com%2Fuploads%2Farticles%2Fuxl6mjluovl5kyp6f6nn.png)

<a id="action-space"></a>

[![Philipp Renoth](<understanding-rust-as-a-c-developer-2o28/2aa62147be836e42c1097e65.webp>)](https://dev.to/daaitch)

[Philipp Renoth](https://dev.to/daaitch)

Posted on May 5, 2021
•
Edited on May 7, 2021

<a id="understanding-rust-as-a-c-developer"></a>

# Understanding Rust as a C++ developer

[#cpp](https://dev.to/t/cpp)
[#rust](https://dev.to/t/rust)

<a id="article-body"></a>

Rust has a steep learning curve and I would also say, not even for professional C++ developers it's an easy switch, although it's said that when you have some C++ background, Rust and you are getting best friends, soon. Well, I really love Rust, also having a C++ background, but the first steps were hard.

One thing you can do is reading ["The Rust Programming Language"](https://doc.rust-lang.org/book/) book for free. It's well-written, but as a C++ developer, we already have some common understandings which would really makes it easier to dive into Rust with the C++ wording. I hope this article may close this gap to get you on board as a C++ developer.

<a id="as-a-c-developer-why-should-i-learn-rust"></a>

# <a id="as-a-c-developer-why-should-i-learn-rust-1"></a> As a C++ developer, why should I learn Rust?

Check out some of latest news:

- Rust is to be introduced as the second language in the Linux kernel.
- Microsoft with a large C++ codebase uses Rust for new modules. According to their [CVE analysis](https://msrc-blog.microsoft.com/2019/07/16/a-proactive-approach-to-more-secure-code/), 70% of all CVEs are memory related bugs and Rust is kinda famous to not allow memory bugs.
- Google is adopting Rust for Android.
- Rust is "the language" for WebAssembly, which is also getting more and more import for the cloud native world.

From my experience with Rust:

- I have much more less bugs (no random segfaults or undefined behavior)
  my components are better designed
- I can write code way faster
- I can use Rust everywhere: embedded, native, web/browser

To use Rust is not an either or choice, so you don't have to change everything, but you can simply start adopting Rust for some non-critical components and get a feeling and I can almost promise that you'll love it.  
That's also where we start. Calling into Rust or C++ with FFI.

<a id="foreign-function-interface-ffi"></a>

# <a id="foreign-function-interface-ffi-1"></a> Foreign function interface (FFI)

Probably, C++ projects will not be entirely written in Rust from one day to another, just because Rust has many advantages. One thing that is happening at some companies is, that they start to adopt Rust and keep their C++ codebase while developing small meaningful components with Rust and include them.

In order to allow calling from C++ into Rust and vice versa, Rust has to follow calling conventions and other interface specifications. For optimization reasons the Rust compiler mangles everything: function names, attribute and parameter order and so on. Also the types have to match, like `long` from C++ is the same like a `i64` in Rust, but only for 64bit arch, so we should use `isize` for that case. Rust is also not compatible to C++ `char[]` for string types. Rust is using UTF-8 encoding with explicit length, unlike C++ has null-byte terminated `char` buffers. `std::ffi` module has some convenient helpers bringing those worlds together.

Rust also supports pointer-types like we have in C++, but you will not likely have to use them, unless you do high sophisticated Rust development or more likely doing FFI. To use references in Rust, we don't do that with pointers, but so called borrows. I also spend a whole section for that topic.

<a id="primitive-data-types"></a>

# <a id="primitive-datatypes"></a> Primitive data-types

Let's start with the real basic types:

```
// comments are C++

// void u; doesn't work
let u = ();

// bool b = true;
let b = true;

// char c = 'c';
let c: i8 = 99;
// unsigned char uc = 'u';
let uc    = b'c';

// short s = -8;
let s = -8i16;
// unsigned short us = 8;
let us = 8u16;

// int i = -8;
let i = -8i32;
// unsigned int i = 8;
let ui = 8u32;

// long long ll = -8;
let ll = -8i64;
// unsigned long long ull = -8;
let ull = 8u64;

// float f = 8.f;
let f = 8f32;
// double d = 8.0;
let d = 8f64;

// long i = -8;
let l = -8isize;
// unsigned long i = 8;
let ul = 8usize;

// unsigned char buf[1024];
let buf = [u8; 1024];

let t = "hi"; // type "&str" (utf8 u8 buffer)
```

So the naming convention of the primitive types in Rust is quite simple. A special case is the type `&str` . That's a reference to a byte buffer, but it's representation is UTF-8 so you cannot have random access. What you can do is to make a so called slice (like an array) out of it and then you have random access, but of course, not to the characters itself, but the underlying bytes. We will cover more details in the "Strings" section.

<a id="arrays-and-slices"></a>

# <a id="arrays-and-slices-1"></a> Arrays and slices

<a id="arrays"></a>

## <a id="arrays-1"></a> Arrays

```
// C++
long a[] = {1, 2, 3, 4};
std::array<long, 8> b{1, 2, 3, 4};
```

```
// Rust
let a = [1, 2, 3, 4]; // type is [i32; _]
```

Arrays in C++ and Rust is contiguous data, where every item has the same size. For FFI if you need a pointer to an array, it's safe to use `my_array.as_ptr()` (also same for Rust slices).

<a id="slices"></a>

## <a id="slices-1"></a> Slices

It's hard to compare `std::slice` with Rust slices, because Rust references data, not only index, size and stride information, like `std::slice` does. Rust slices store a pointer and the length. The stride is known from the type itself:

```
// Rust
let a = [1, 2, 3, 4];
let b = &a;             // type &[i32; _] => array borrow
let c = &a[..];         // type &[i32]    => slice
let d  = &a[1..3];      // type &[i32]    => slice
println!("{:?}", &a);   // [1, 2, 3, 4]
println!("{:?}", b);    // [1, 2, 3, 4]
println!("{:?}", c);    // [1, 2, 3, 4]
println!("{:?}", d);    // [2, 3]
```

<a id="initialization"></a>

# <a id="initialization-1"></a> Initialization

We don’t have uninitialized data in Rust. When we declare a variable, we also have to initialize it with value.

```
// Rust: not working
let i;
let a = i + 4;
```

Although it’s valid Rust to separate declaration and definition it’s quite unlikely to see this:

```
// Rust
let i;
// ...
i = 5;
```

Mostly we do this:

```
// Rust
let i = ...;
```

To say that there is no uninitialized in Rust to not quite correct. Well, there is, but without `unsafe` Rust we can't access it and no data structure should expose uninitialized data. For example Rust also has a vector called `Vec` and we can create one with a given capacity, so the needed size is allocated and the memory is uninitialized (because it's faster than zeroed memory). `Vec` doesn't let us access uninitialized elements, so we are safe.

<a id="type-inference"></a>

# <a id="type-inference-1"></a> Type inference

In Rust we don’t have to write types on the left-hand side of an assignment statement, when it can be inferred from the right-hand side. In C++ we have `auto`, but a type that really doesn't do unnecessary copies whenever we use it on the left-hand side, is the universal type `auto&&`.

```
// C++
auto&& text = "some text";
auto&& i = 8; // int
auto&& j = i; // &int
```

```
// Rust
let text = "some text";    // vs. let text: &str = "some text";
let i = 8u32;              // vs. let i: u32 = 8;
let j = &i;                // &u32
```

There are also generic types, which can be inferred, even for return values, so we either pass the generic parameter type to the function or we simply give the left-hand side a type, so the generic parameter on the right can be inferred from the left.

<a id="functions"></a>

# <a id="functions-1"></a> Functions

```
fn get_five() -> u32 {
    5                            // no semi-colon
}

fn add_five(value: u32) -> u32 {
    value + 5                    // no semi-colon
}

fn main() {
     let five = get_five();
     let ten = add_five(five);
}
```

A function starts with the keyword `fn` and has the return type at the end like `-> u32`. If the function doesn't return anything which is the unit type `()`, then you can omit it, like in the main function. Rust also has the `return` keyword, but it's mostly used for "early return", because in Rust the last expression without an appended semi-colon is the return value.

<a id="meta-programming"></a>

# <a id="meta-programming-1"></a> Meta programming

Rust provides an advanced macro system in contrast to C/C++ "replace" macros. In short, at first it's possible to do the same like in C/C++, but you can also be more specific about the macro parameter types and second, you can use macros as a preceding phase where Rust code can create other Rust code to be compiled. This allows us to do incredible things, like e.g. embedding preprocessed resources or do compile-time integrity checks of data we embed. Calling Rust macros is like with functions, but with `!` at the end.

```
let name = format!("{} {}", get_first_name(), get_last_name());
```

What look’s like a waste of resources in Rust with string formatting with `format!` is the way to go when you want to concatenate strings. Everything is done at compile time and it's safe, not like `printf` in C with runtime cost. The string is parsed at compile time. For all curly braces there has to be a parameter. If not, it will not compile.

Embedding a file is also very easy, so that the file's content is in the binary itself.

```
let image_data = include_bytes!("./image.png");
```

<a id="strings"></a>

# <a id="strings-1"></a> Strings

C++ has `std::string` and `char *` (and some other flavors, like wide strings). Rust has byte slices `[u8]` or `[i8]` as a byte representation and `&str` and `String` as UTF-8 string.

```
// C++
const char* t = "some text";
std::string s(t);
std::string a = s + " in C++";
```

```
// Rust
let t = "some text";
let s = t.to_owned();
let a = format!("{} in C++", &s);
```

In Rust every value or variable has a type and all types have some functions to be called, even numbers. `&str` implements `.to_owned()` which makes a String out of it. The same is happening as in C++: heap is allocated and data is copied, so the string is now owned.

<a id="optional-values-and-exceptions-vs-results"></a>

# <a id="optional-values-and-exceptions-vs-results-1"></a> Optional values and exceptions vs. results

One of the core types in Rust (beside the primitive ones) is `Option` and `Result`. Rust doesn't have exceptions, so without that side-channel our return value needs to wrap information about it. Those two types are also part of the prelude, so we don't have to "import" and just use them.

<a id="optiont"></a>

## <a id="-raw-optionlttgt-endraw-"></a> `Option<T>`

The fact that a value exists or not can be represented by an Option in Rust and like `std::optional` in C++. Let's say we build an CLI argument parser.

```
fn is_dry_run() -> Option<bool> {
    Some(true)
}

fn get_host() -> Option<String> {
    None
}

fn main() {
    let dry_run = is_dry_run().unwrap_or(false);
    let host = get_host().unwrap_or("localhost".to_owned());
}
```

`Option` has a lot of convenience methods like `unwrap_or` which is like `std::optional::value_or`.

<a id="resultt-e"></a>

## <a id="-raw-resultltt-egt-endraw-"></a> `Result<T, E>`

I think C++ doesn't have such a wrapper type in `std`, but one can simply return the success value or throw an error. `Result` in Rust has two generic parameters: the success value type and the error type.  
Let's say we want to get the current user name form a DB or return the error code.

```
fn fetch_current_user() -> Result<String, u32> {
    Ok("robert_rust".to_owned())
    // ...or
    // Err(34)
}
```

That way the result can have two different values and it's not possible to access let's say the success value, if the return value is `Err(..)`, because it will immediately "crash" (we say "panic" in Rust and we'll cover that in the next session as well).

<a id="-operator"></a>

## <a id="-raw-endraw-operator"></a> `?`-operator

Don't confuse it with the ternary conditional operator in C++. It's some syntactic sugar. So we don't have exceptions on the one hand, but writing error handling boilerplate on the other hand is also not fun. This is why we have a new nifty operator for "early return".

For the next example we have some functions for a DB and of course, they may fail see we use `Result`.

```
fn connect_db() -> Result<(), u32> {
    Ok(())
}

fn fetch_current_user() -> Result<String, u32> {
    Err(34)
}
```

Let's glue it together with all boilerplate code. To be honest there are really better ways without `?`-operator, like with match and we will cover them soon, but let's make it real bad first.

```
// not so good Rust :(
fn fetch_users_from_db() -> Result<String, u32> {
    let connect = connect_db();
    if connect.is_err() {
        return Err(connect.unwrap_err());
    }
    let current_user = fetch_current_user();
    if current_user.is_err() {
        return Err(current_user.unwrap_err());
    }
    Ok(current_user.unwrap())
}
```

We store the result in a variable, we check if it has an error and then we `unwrap` the error and wrap it in another `Err` which is a `Result` and we have to do that for all checks. That's not so readable. Let's use the `?`-operator.

```
fn fetch_users_from_db() -> Result<String, u32> {
    connect_db()?;
    let current_user = fetch_current_user()?;
    current_user
}
```

Or even shorter, because the result of fetch\_current\_user is of the same return type like `fetch_user_from_db()`.

```
fn fetch_users_from_db() -> Result<String, u32> {
    connect_db()?;
    fetch_current_user() // omit semi-colon
}
```

That even works for `Option` when `None` values should be "early returned". When you have different error types, you need to map them to the the same type to be allowed to use the `?`-operator, but that's more advanced Rust, we skip for now.

<a id="stdterminate-vs-panic"></a>

# <a id="-raw-stdterminate-endraw-vs-raw-panic-endraw-"></a> `std::terminate()` vs. `panic!()`

No exceptions in Rust, but when there is something going so terribly wrong that there is no chance to recover, we can panic the current thread. Doing that on the main thread means our application will crash. An uncaught exception in C++ has the same behavior.

Rust allows us to write robust and safe code, but you can make wrong assumptions, that will panic at runtime.

```
let name: Option<String> = None;
println!("{}", name.unwrap());    // means "it MUST be Some(..)"
```

For example all `.unwrap...()` methods of `Option` or `Result` will panic if it can't unwrap. Another example are "index out of bounds" panics. Consider panics as a last resort for a Rust program to terminate, before more bad things may happen. Panics are always unexpected errors. E.g. a connection to a database may break, so it's an error but not unexpected.

<a id="control-flow--conditions-and-loop"></a>

# <a id="control-flow-conditions-and-loop"></a> Control flow — conditions and loop

```
// comments are C++

// if (i == 4) {
// } else {
// }

if i == 4 {
} else {
}

// switch (i) {                      
// case 3:                               
// case 4:                               
// default:                              
// }

match i { // no fall through!
    3 => {}
    4 => {}
    _ => {}
}

// for (int i = 0; i < 10; i++) {
// }

for i in 0..10 {
}

// while (i < 10) {                  
// }

while i < 10 {
}

// do {                              
// } while (i < 10);

loop {
    if i < 10 {
        break;
    }
}
```

There is more. `if`, `match` and `loop` are also expressions in Rust, so they can return a value which is quite handy for many use-cases.

```
// C++
int v = i < 10 ? 3 : 5;         
```

```
// Rust
let v = if i < 10 { 3 } else { 5 };
```

The C++ code may look more pleasant, but let's do more in the conditional blocks.

```
let meal = if customer.is_very_hungry() {
    let burger = new_burger();
    for i in 0..3 {
        burger.add_paddy();
    }
    new_plate(burger)
} else {
    new_plate(new_cheeseburger())
};
```

The elegant part of that code is, that the outcome of `if` is stored in `meal`, so it can also be guaranteed that it is initialized. In C++ it might be better to put everything in another function or an immediately invoked lambda.  
We can do the same with `match` and even `loop`.

```
let age = 24u32;
let score = match age {
    0..=18 => 0,
    19..=35 => 1,
    _ => 2
};
```

Another very useful thing about `match` is that all patterns from all arms have to be exhaustive regarding the input type. Code will not compile if we omit the `_`-placeholder which matches "the rest" of it all. That's very helpful for enums, because let's say an enum has 3 possible values and you all handle them, and then someone adds a forth one, then you have to handle that case, too, because it's a compile error to be non-exhaustive.

<a id="enums"></a>

# <a id="enums-1"></a> Enums

I'd say enums is Rust are a combination of C++ `enum`, `union` or `std::variant`. It's not only the case, that we define enumerators, but they can hold different types each and that's a Rust killer feature. We can even `match` the type and unwrap its inner value together in one line.

```
enum Error {                 // an app error type
    DbQuery(u32),            // db has error codes
    Unexpected(String),      // everything else should be a string
}

fn handle_error(err: Error) {
    match err {
        Error::DbQuery(code) => {},    // code is the u32 db code
        Error::Unexpected(msg) => {},  // msg is a String
    }
}
```

We've already used the most popular enums in Rust: `Option` and `Result`.

<a id="oop"></a>

# <a id="oop-1"></a> OOP

Most of the things in C++ can also be done in Rust and for a few things we need change our mindset. It may look like a limitation, but Rust community didn't introduce every known OOP feature from all languages. Even such basic OOP features like inheritance is nowadays often considered harmful, because it introduces complexity or even a bit phoniness, a straight specialized design may not have.

<a id="structclass-vs-structenum"></a>

## <a id="structclass-vs-structenum-1"></a> struct/class vs. struct/enum

In C++ we have struct and class and they can have attributes and methods. In Rust there is `struct` and `enum` with the attributes and `impl` with its methods.

```
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, I'm {} {}", &self.first_name, &self.last_name);
    }
}

enum Value {
    High,
    Low,
}

impl Value {
    fn is_high(&self) -> bool {
        match self {
            Value::High => true,
            Value::Low => false,
        }
    }
}
```

What looks a bit strange at first, is the `&self` parameter. That parameter has an implicit type, namely the struct itself, so it's like the `this` from C++. Given `&self` says, that it is an object method. Omit it and it's a static method.

```
impl Person {
    fn name_placeholder() -> String {
        "Please enter your name".to_owned()
    }
}

fn main() {
    let placeholder = Person::name_placeholder();
}
```

<a id="inheritance-vs-traits"></a>

## <a id="inheritance-vs-traits-1"></a> Inheritance vs. traits

There is no inheritance (or overloading) in Rust, but it’s possible to extend the implementation with a so called `trait` and they also allow to have abstract interfaces. Rust is also quite explicit about dynamic dispatch, as it adds additional runtime costs (same in C++) and most of the things that are not obvious for free, have some code hints in Rust, like `dyn` for dynamic dispatching.

```
trait HasName {
    fn get_name(&self) -> String;
}

impl HasName for Person {
    fn get_name(&self) -> String {
        format!("{} {}", &self.first_name, &self.last_name)
    }
}

fn good_bye(sth: &dyn HasName) { // dyn for dynamic dispatch
    println!("Good bye {}", sth.get_name());
}
```

Let's simply assume `&` is like a reference in C++. There will be a section about references, borrows and so on, but for now we are okay to know that this is a reference to some `HasName`. Because `HasName` is a `trait`, the call has to go through a `vtable`. The same happens in C++ with virtual methods, but we don't see it there. Here, Rust clearly says, that we have a variable where all calls are dynamic dispatched, because the type is a `trait`. The additional costs are not that much, but we may don't want it, because we are in a tight loop and performance matters and with dynamic dispatch the compiler cannot make further optimizations like inline the invoked method, because at compile time Rust doesn't know which specialized function will be invoked at runtime. In "Templates vs. generic code" there will be the same example, but with monomorphization to make it static dispatched.

<a id="con-destructor-vs-constructor-drop"></a>

## <a id="condestructor-vs-constructor-raw-drop-endraw-"></a> con-/destructor vs. constructor/`drop()`

Now what about a constructor and destructor in Rust? While a constructor in C++ is a special method, it's simply a static method in Rust, like a factory method. Per convention its name is `new`, but one can use any name, e.g.

```
let s1 = String::new();               // -> String
let s2 = String::with_capacity(1024); // -> String
```

Let's also give Person a constructor.

```
impl Person {
    fn new(first_name: String, last_name: String) -> Person {
        Person {
            first_name, // shorthand for "first_name: first_name"
            last_name,
        }
    }
}
```

There is also a destructor in Rust and it's quite interesting how it's used. There is a special `trait Drop` with one method, which is called from the Rust compiler, when an object goes out of scope.

```
impl Drop for Person {
    fn drop(&mut self) { // we will handle "mut" later
        // ...
    }
}
```

That should be enough for OOP. There is a section about "Const-correctness vs. mutability" which will dive into OOP again. Then we clarify what `mut` is.

<a id="templates-vs-generic-code"></a>

# <a id="templates-vs-generic-code-1"></a> Templates vs. generic code

Generic Rust code is similar to C++ templates. It's compile-time generated code for all different use-cases in the code, so we write it once and the compiler inserts it multiple times for every different generic parameter set.

The big difference is, that C++ templates are not checked by the compiler, until they are used and the generated is compiled, more or less like macros. It's possible to put constraints on the generic parameters to only allow what really works for the implementation, but at least everything is allowed as long as it finally compiles.

```
// C++
#include <iostream>
template<class T> // any T allowed here
void good_bye(T& sth) {
    std::cout << "Good bye " << sth.GetName();
}

struct Person {
    std::string name;

    const std::string& GetName() const {
        return name;
    }
};

int main()
{
    Person p {name: "Charlie Cohan"};
    good_bye(p); // okay
    good_bye("bad value"); // error: request for member ‘GetName’ in ‘sth’, which is of non-class type ‘const char [10]’
    return 0;
}
```

Rust is different as we have to write generic code which is valid for all possible values. Going back to our `trait HasName`:

```
// Rust
fn good_bye<T: HasName>(sth: &T) {
    println!("Good bye {}", sth.get_name());
}

// or different (just cosmetics)
fn good_bye<T>(sth: &T)
where
    T: HasName,
{
    println!("Good bye {}", sth.get_name());
}

// , but not
fn good_bye<T>(sth: &T) { // notice missing type constraint here
    println!("Good bye {}", sth.get_name()); // err: no method named `get_name` found for reference `&T` in the current scope
method not found in `&T`
}
```

I remember situations, when I had no clue what the C++ compiler wanted to tell me, like generic code errors can get very confusing. Rust will tell if you wrote incorrect generic code. If you really (really) need fuzzy templates in Rust you can you use macros.

There is another point worth mentioning here, which also applies to C++. What we've done here is called monomorphization. We've already had the `good_bye` function with dynamic dispatching. The versions we create with the generic code is for each individual type when we use the function, so the type of the borrow is not `&dyn HasName`, but the specialized type e.g. `&Person` which allows static dispatching with all compiler optimizations. It's a trade-off between performance and larger binaries, but I think larger binaries are mostly okay.

<a id="namespaces-vs-modules"></a>

# <a id="namespaces-vs-modules-1"></a> Namespaces vs. modules

Like C++ namespaces, modules in Rust tie together code and hide implementation details. There are strict rules about the module hierarchy and the source file locations in Rust, while in C++ each file is considered without any context like file location. Because Rust's module system is quite different (at least to the ones I know from other languages), I spend another article for that topic:

[![daaitch](<understanding-rust-as-a-c-developer-2o28/20614f976e78de5b794692a1.jpeg>)](https://dev.to/daaitch)

<a id="rust-module-system"></a>

## [Rust module system](https://dev.to/daaitch/rust-module-system-3ja4)

<a id="philipp-renoth--may-5-21"></a>

### [Philipp Renoth ・ May 5 '21](https://dev.to/daaitch/rust-module-system-3ja4)

[#rust](https://dev.to/daaitch/rust-module-system-3ja4)

<a id="const-vs-mut"></a>

# <a id="-raw-const-endraw-vs-raw-mut-endraw-"></a> `const` vs. `mut`

In C++ everything can be mutated if not `const`. In Rust it's the other way round: everything is immutable if you don't make it `mut` (for "mutable").

```
// C++
const int j = 4;
int i = 4;
i = 8;
```

```
// Rust
let j = 4;
let mut i = 4; // mandatory "mut"
i = 8;
```

Rust has a `const` keyword, but we don't use it for making a variable const as this is the default. There are other use-cases for const in Rust:

- for global values e.g. `const SOME_VALUE: u64 = 8;`
- for FFI compatibility e.g. `*const u32`

Be prepared, for Rust `mut` is more than just a keyword whether we change something or not. There is a section "Const-correctness vs. mutability" where we will talk about the difference.

<a id="passing-values--reference-borrows-ownership"></a>

# <a id="passing-values-reference-borrows-ownership"></a> Passing Values — reference, borrows, ownership

In C++ there are maybe 6 or more value types, like "lvalue", "rvalue", "xvalue" and so on. In short: forget them for Rust.

Rust has 2 value types:

- borrow: like a C++ reference
- ownership takeover: like C++ "move", but a bit different so I also spend a section for move semantics

```
// Rust
fn foo(s: &S) {}
fn bar(s: S) {}

fn main() {
  let s = S {};
  foo(&s); // borrow, see the "&"
  bar(s);  // ownership takeover
  // don't use `s` here anymore (or it will not compile)
}

struct S {}
```

One of Rust's core concepts is "ownership". If you want to know more about it have a look at this article:

[![daaitch](<understanding-rust-as-a-c-developer-2o28/20614f976e78de5b794692a1.jpeg>)](https://dev.to/daaitch)

<a id="rust-ownership-and-borrows---fighting-the-borrow-checker"></a>

## [Rust ownership and borrows - Fighting the borrow-checker](https://dev.to/daaitch/rust-ownership-and-borrows-fighting-the-borrow-checker-4ea3)

<a id="philipp-renoth--may-5-21-1"></a>

### [Philipp Renoth ・ May 5 '21](https://dev.to/daaitch/rust-ownership-and-borrows-fighting-the-borrow-checker-4ea3)

[#rust](https://dev.to/daaitch/rust-ownership-and-borrows-fighting-the-borrow-checker-4ea3)

So there are only 2 options in Rust I'll show with some C++ code:

```
// C++
struct S {};
void foo(S& s) {} // reference
void bar(S s) {}  // by value:
                  // copy-constructed with lvalue (not in Rust)
                  // move-constructed with rvalue

int main() {
  S s = {};
  foo(s);            // reference, but we don't see it here
  bar(std::move(s)); // make rvalue of s, so it's move-constructed
}
```

The first `foo` function is taking a reference, which is quite the same in Rust, beside the borrow rules we cover a few sections later. The second `bar` function is getting more interesting. It's a by value parameter, so according to C++ OOP it's possible to create copy-constructed or move-constructed `S`. The first one is not possible in Rust as we can only pass a reference or an rvalue in Rust (but it's of course possible to pass a "copy" see the next section):

- reference means: you can use it, but you just borrowed it. It's not yours.
- ownership takeover: it's like passing a rvalue in C++ where you are not allowed to use the old lvalue anymore. If you use it, it will not compile anymore.

Understanding move semantics in C++ can become quite heavy. That's really gotten easy in Rust. It's not possible to implement that on your own, so the compiler is in full charge to do a good job for us.

<a id="lifetimes"></a>

## <a id="lifetimes-1"></a> Lifetimes

Both C++ and Rust have lifetimes for data, but the difference is, that C++ is very permissive according their checks and the language C++ doesn't support any lifetime annotations so we have conventions, good and bad practices, like the compiler allows us to do that with a warning:

```
// bad C++
int& get_int() {
    int i = 4;
    return i;
}
```

In that case the compiler has some checks which say, that returning locals as reference may be a bug. But let's have a look at this horrible piece of C++ code which should hopefully not compile, but it does.

```
// very bad C++
#include <iostream>
struct Person {
    std::string name;
};

int main()
{
    std::string* name;

    {
        Person p {name: "Charlie Cohan"};
        name = &p.name;
        std::cout << "Name: " << *name << std::endl; // ok
    }
    std::cout << "Name: " << *name << std::endl; // bad!
    return 0;
}
```

Now let's try to crash a Rust program.

```
fn get_int() -> &u32 { // err: missing lifetime specifier
    let i = 8;
    &i
}
```

The first error is, that when we return a borrow, we need to be explicit about a lifetime. Lifetimes in Rust look like `'a` or `'b` or `'whatever_you_like`. Then there is another special lifetime `'static` which means "it lives for the entire runtime of the application". For example `let t = "text";` is of the type `&'static str`, because "text" is from the data segment and valid through the whole program.

Now, let's fix that and face the next issue.

```
fn get_int<'a>() -> &'a u32 {
    let i = 8;
    &i // err: returns a reference to data owned by the current function
}
```

So what is a warning in C++ is an error in Rust and that's good. Let's try the other example:

```
// Rust
fn main() {
    let name: &String;
    {
        let p = Person { name: "Robert Rust".to_owned() };
        name = &p.name; // err: `p.name` does not live long enough

        println!("Name: {}", name); // ok
    }
    println!("Name: {}", name); // err: borrow later used here
}

struct Person {
    name: String,
}
```

Rust compiler says: "I will not allow you to use name after the lifetime of `p` and `p` is dropped inside the block, like in C++. So the Rust compiler also terminates the lifetime of `name` inside the block to fix the constraint that the lifetime of `p` is shorter.

This is another reason why Rust is safe. We are not allowed to violate those constraints and the borrow-checker ensures that.

<a id="copy-vs-clone"></a>

# <a id="-raw-copy-endraw-vs-raw-clone-endraw-"></a> `Copy` vs. `Clone`

Rust is very restrictive and there is a difference in cloning or copying data. You're not supposed to "duplicate" data only because you want to. The only data you're allowed to do that are scalar primitive data, like `u32`. No struct has a default implementation for copying or cloning data and that's for a reason, because for example, how does Rust know that you're not storing resource handles as `u64` and duplicating the struct means double freeing the resource when it drops?

<a id="clone"></a>

## <a id="-raw-clone-endraw-"></a> `Clone`

What's called copy-constructor in C++ is cloning in Rust and the convention is different, because we have a simple object function implemented called `.clone()` and return the clone. To be more precise the `trait Clone` is implemented. Rust will never ever make a clone for you, unless you call it. So in other words, if someone wants to takeover ownership, but you don't want to give away ownership, pass a clone (if possible).

```
fn main() {
  let s = "text".to_owned();
  foo(s.clone());
  println!("{}", &s);
}

fn foo(s: String) {} // wants ownership
```

A clone can be expensive during runtime, like a vector is doing heap allocation. That's why we have to call it and we will read it in the code. In C++ it's possible that we're copying data around, where there is no need to, because like e.g. we're passing lvalues to pass by values parameters, although we actually want to move it.

<a id="copy"></a>

## <a id="-raw-copy-endraw-"></a> `Copy`

Simple data structures can be marked as `Copy` as long as their attributes also implement it, which means that the compiler is allowed to make a bit-wise copy and the structure itself has to be `Clone`, because a `Copy` struct should always also be `Clone`.

```
fn main() {
    let mut f = Fraction{ nom: 3, denom: 10 };
    let c = f; // can't be moved as we want to borrow "f" later,
               // but a copy fixes it
    f.nom = 4;
}

#[derive(Copy, Clone)] // Rust magically implements everything
struct Fraction {
    nom: u32,
    denom: u32,
}
```

<a id="tuples"></a>

# <a id="tuples-1"></a> Tuples

Since C++11 we have e.g. `std::tuple<int, long>`. In Rust that's simply the type `(i32, isize)`. You can't have multiple return values in Rust, but you can return a tuple, which is the same. There is a special tuple called "unit" in Rust which is `()` and is what void is in C++, so nothing.

```
// C++
std::tuple<int, long> values {1, 2};
int i  = std::get<0>(values);
long l = std::get<1>(values);
```

```
// Rust
let values = (1i32, 2isize);
let i = values.0;
let l = values.1;
```

<a id="memory-management"></a>

# <a id="memory-management-1"></a> Memory management

Like C++, Rust also ...

- doesn't have a garbage collector
- has two memory types for dynamic data: stack and heap. Everything with fixed size (not too big) can be allocated on the stack, while dynamic sized data has to live on the heap.
- has no runtime per default (no overhead, just a main thread and a main() entry point)

Unlike C++, Rust ...

- also has move semantics, but implements it differently, so I spend a complete section for it
- has a default safe mode, which should not allow undefined behavior, e.g. memory corruption, dangling pointers, double free, used after free and so on.

<a id="runtimes"></a>

# <a id="runtimes-1"></a> Runtimes

There is also no runtime default in Rust. We have a `main()` entry point and the main thread running it. Especially IO-heavy Rust applications, like many web applications are, often stick to so called "async-runtimes" like `tokio` being an executor for Rust's `async/.await` syntax for non-linear async code. Just mentioned it, we will not cover that. Another problem solved are transitions from computation-heavy to IO-heavy code and back or using legacy/blocking APIs. Those runtimes solve similar problems like ReactiveX (Rx) or boost async package does.

<a id="move-semantics"></a>

# <a id="move-semantics-1"></a> Move semantics

C++ and Rust have move semantics, but they are very different. I think it's important to understand the difference so let's recap move semantics in C++, first.

<a id="c-move"></a>

## <a id="c-move-1"></a> C++ move

The main use-case in C++ is instead of copying data around, we move internal data, that e.g. strings or vectors can be moved, by simply transferring the heap pointer from our stack-object to another object and leave the old object in a valid state. One should actually not use them anymore, after they have been moved and at least they are destructed, so it has to be valid, that e.g. the memory is not double-freed. In C++ having classes without explicit dynamic memory allocation (`new`, `malloc`) we (mostly) also don't need to implement the move semantics on our own, as the default is what we want. I said "mostly" because there might be some edge-cases where a move is more complicated and need to be implemented manually or someone holding a pointer to an object (for whatever reason) and moving that object then is not allowed.

To make one's life easier, we can use smart pointers in our classes, so there is no dynamic memory allocation done by the class itself and default implementation should be fine. If one of the attributes doesn't implement "moving" or "copying" nor either does the class, e.g. copying `unique_ptr` will not work.

<a id="rust-move"></a>

## <a id="rust-move-1"></a> Rust move

Good news is, that we have another concept in Rust called "ownership". I'd call it a good practice in C++ to have every piece of data being owned by another component, but even the definition of "ownership" in C++ can be vague. Modern C++ with `unique_ptr` obviously expresses who owns that data, but what about a function returning a pointer? Should the caller free it, is it somehow garbage-collected or what's happening? In that case a function should have documentation about ownership.

We always have two objects involved in C++, where one moves into the other and obviously the first one going out of scope and getting destructed. In Rust "moving" means, that object data is copied bit-wise and the source is not "destructed". That boilerplate is implemented by compiler and he's even allowed to not really "move" but just reference the data and shorten the scope. From our perspective it is moved, but compiler is in charge to optimize it. So whatever data structures we create in Rust, they are all "move"-ready! Isn't that great?

To be more precise they are all "move"-ready, but there is a concept called "pinning" which suppresses moving data for a good reason coming from the `async/.await` world of Rust, but we don't cover that here.

<a id="const-correctness-vs-mutability"></a>

# <a id="constcorrectness-vs-mutability"></a> Const-correctness vs. mutability

For scalar values to be changed, they need to be `mut` in Rust, like they are not allowed to be `const` in C++. There is this thing called "const-correctness" in C++. Almost no case needs us to const\_cast except for a few edge cases, like "caching", where the outer view is to just read values, but internally we also cache the value. We have the same view in Rust, plus another "core rule" for mutability.

The rule in Rust says, that we can either have multiple immutable borrows of something or exclusively one mutable borrow. That way for example it's not possible to iterate over a collection while modifying it directly. But there are also other cases, where we want multiple mutable owners, like shared state in multithreaded applications. If the threads only want to read data, they are allowed to do this in parallel, but if someone wants to mutate the state, too, it needs to be wrapped in a `Mutex` or `RwLock` to ensure the rule and that wrapper pretend immutable access to mutable data, only because they can make sure that only one thread can hold a lock at any time.

<a id="smart-pointer-vs-box--rc--arc"></a>

# <a id="smart-pointer-vs-raw-box-endraw-raw-rc-endraw-raw-arc-endraw-"></a> Smart pointer vs. `Box`/`Rc`/`Arc`

<a id="stdunique_ptrt-vs-boxt"></a>

## <a id="-raw-stduniqueptrlttgt-endraw-vs-raw-boxlttgt-endraw-"></a> `std::unique_ptr<T>` vs. `Box<T>`

Let's start with the `std::unique_ptr` in C++ which can't be copied, but moved and it's holds a pointer to something living on the heap. In Rust it's called `Box` and the same applies, so it's not `Clone`.

```
// Rust
fn main() {
    let b = Box::new(Person {
        name: "Robert Rust".to_owned(),
    });
    b.say_hello(); // via `Deref`
}

struct Person {
    name: String,
}

impl Person {
    fn say_hello(&self) {
        println!("Hi, I'm {}", &self.name);
    }
}
```

Interesting points are

- Rust always creates a `Box` from moved data, so we create a `Person` on the stack, where the owned `String` also has data on the heap. Then the person is bit-wise moved to the heap, so `name` itself is moved, but a `String` consists of a heap pointer and a length, so a few bytes (I think 4+4 Bytes) no matter how long the string is. This is safe, because `Person` is destroyed, when `Box` is and we're not allowed to use moved-out values in Rust.
- For convenience reasons we have an `operator->` in C++ so that we can access the underlying pointer with an arrow. We have the same in Rust, but it's called `Deref` which is a `trait`, but access with a simple dot. So we can call all `Box` and `Person` methods.

<a id="stdshared_ptrt-vs-rct--arct"></a>

## <a id="-raw-stdsharedptrlttgt-endraw-vs-raw-rclttgt-endraw-raw-arclttgt-endraw-"></a> `std::shared_ptr<T>` vs. `Rc<T>`/`Arc<T>`

`std::shared_ptr<T>` in C++ is what `Arc<T>` (Atomic Reference Counted) is in Rust. It's not possible in C++ to have an instance with a dangling pointer, and it's not double freed, so the reference counter is atomic and it can be used from multiple threads, but the internal data structure has to be synchronized for multiple threads.

In Rust `Arc<T>` has the same properties, but the internal data structure can only be accessed as immutable. That way it's safe in Rust to use any `T` for an `Arc`, because we are not allowed to mutate it and multiple reads are fine. In the next section we'll talk about how to anyway share mutable state with `Arc<T>`.

The fact that `Arc<T>` is using atomic counters is unnecessary overhead in a single-threaded context. Rust always addresses such concerns so we can stick to the little brother `Rc<T>`. Again the internal value is immutable. To break that restriction, we can use `Rc<RefCell<T>>` to enable interior mutability, but to also ensure exclusive mutable access, `RefCell<T>` adds runtime checks for borrows. Of course accessing the value won't block the thread as it would be an immediate deadlock, but it panics if the the borrow rules are violated.

<a id="lambdas-vs-closures"></a>

# <a id="lambdas-vs-closures-1"></a> Lambdas vs. closures

Let's check out the next example. C++ lambdas and Rust closures are also quite similar.

```
// C++
#include <iostream>
#include <vector>
#include <algorithm>

bool has_number(int i, const std::vector<int>& v) {
    return v.end() !=
      std::find_if(v.begin(), v.end(), [i](auto& n) {
        return n == i;
      });
}

int main() {
    std::vector<int> v {1, 2, 3, 4, 5};
    std::cout << (has_number(4, v) ? "yes" : "no") << std::endl;
    return 0;
}
```

The lambda is capturing `i` as copy, but that's fine, because it just a scalar value. Let's have a look at Rust closures.

```
// Rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{}", if has_number(4, &v) { "yes" } else { "no" });
}

fn has_number(i: i32, v: &Vec<i32>) -> bool {
    v.iter().find(|n| {
        **n == i
    }).is_some()
}
```

So what's happening inside `has_number`? `v` is our borrow to `Vec` and it has a method `.iter()` which creates an iterator. In Rust collections only have their container specific methods, like `.push()` or `.pop()` if possible, but e.g. no `.map(..)`. For iteration they all provide `.iter()` which creates an iterator where we can find like `.map(..)` or `.find(..)`. Here we use `.find(..)` which wants a predicate closure, taking a borrow of the item type. That means .iter() creates an iterator over `&i32` because the items still live in the `Vec`, not moved into the iterator. And `.find(..)` wants a closure taking `&&i32`, which is borrow of a borrow of `i32`, like double pointers in C++. That's why we need to deref it twice (`**n`) and then compare. The return value of `.find(..)` is an Option so we check with `.is_some()`.  
Rust has 3 closure types (`Fn`, `FnMut`, `FnOnce`) for expressing different capture types of the closure context. Closures have an anonymous type as well as a lifetime depending on the capture type. In the last section we have another interesting example about closures and lifetimes.

<a id="multithreading"></a>

# <a id="multithreading-1"></a> Multithreading

In C++ we're allowed to share everything between threads. It's our duty to make sure that data is synchronized. That can be, everyone is just reading or only one thread is writing exclusively. It's easy to violate that and immediately crash your application. But to have an application running correctly for hours until it segfaults and crashes or even worser doing undefined things, like corrupting memory, can drive you nuts.

<a id="send-and-sync"></a>

## <a id="-raw-send-endraw-and-raw-sync-endraw-"></a> `Send` and `Sync`

Rust has two traits `Send` and `Sync` to manage correct data access in multi-threaded applications. They are called auto traits as they are automatically implemented for all types where appropriate. The traits are marker traits without any implementation, just marking the type. `Send` means that the type can be safely transferred between threads and `Sync` means that access from multiple threads is allowed. So let’s do some examples:

```
// Rust
fn main() {
    let p = Person { age: 24 };
    std::thread::spawn(|| { // err: closure may outlive the current function, but it borrows `p`, which is owned by the current function
may outlive borrowed value `p`
        println!("I'm {}", p.age);
    });
}

struct Person {
    age: u32,
}
```

It sounds intuitive to not make this compile, because what's happening here is, that `p` may have already been dropped when the thread starts computation and using a borrow of `p`, it seems correct to not allow this. Let's have a look at the definition of `spawn` to clarify why Rust doesn't let us create such silly bugs:

```
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce<(), Output = T> + Send + 'static,
    T: Send + 'static
```

- closure is called once (`FnOnce`) and needs to be `Send` and `'static`
- the return value of the closure needs to be `Send` and `'static`

With the borrow of `p` we are shortening the lifetime of the closure to the lifetime of `p` so it cannot outlive it. That violates the first rule that it has to be `'static`. In order to make it work, we can either move the `Person` into the closure or use the `age` value itself. Let's move `p`:

```
let p = Person { age: 24 };
std::thread::spawn(move || { // move keyword
    println!("I'm {}", p.age);
});
```

So `p` ownership is taken over by the closure, so it stays `'static` and it is `Send` because `Person` is auto `Send` because all attributes are `Send`. For the sake of completeness, also the struct itself is not explicitly marked as `!Send`, which is possible to manually disable `Send`. For example it may contain some resource that can only be handled from that thread, like some OpenGL handle which are only numbers, but useless in other threads.

```
let p = Person { age: 24 };
let age = p.age;
std::thread::spawn(move || {
    println!("I'm {}", age);
});
```

The same rules apply for age here and we're not moving the whole `Person`.

<a id="shared-state"></a>

## <a id="shared-state-1"></a> Shared state

There is so much more about concurrency in Rust, like channels, but let's go back to an easy example of how we can share a mutable state in Rust between threads. I'm not saying that this is a good idea or it will scale with all threads, but at least it will work and there are no race conditions possible.

```
// Rust
use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

fn main() {
    let s = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let thread_state = s.clone();
        handles.push(spawn(move || {
            let mut number = thread_state.lock().unwrap();
            *number += 1;
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let state = s.lock().unwrap();
    println!("n = {}", *state); // n = 10
}
```

A common pattern is to put a `Mutex` or `RwLock` into an `Arc`, because:

- `Arc<T>` is `Clone`, but it will not make a deep clone, just allow sharing the internal value. It will also not allow to access `&mut T`, so from that point cloning is safe.
- `Arc<T>` is `Send` if `T` is `Sync` and `Send`, which means
  1. `Send` takes care that the value "is useful/allowed" on another thread, like the example of OpenGL handles are not `Send` at all and
  2. `Sync` means, that `&T` can be shared between threads, or in other other words `&T` is Send which works for a `Mutex`, because access is synchronized.
- `Mutex<T>` is `Send` if `T` is `Send`, which again makes sense, because of the same reason of the OpenGL handle example.
- `Mutex<T>` is also `Sync` if `T` is `Send` and the reason is, only one thread can access the value at one time. `T` doesn't need to be `Sync`, so we can put like `HashMap` or `Vec` in it and because of the mutual exclusive access, it's safe.

Crashing a Rust program is possible when doing something wrong, but we will never ever run into undefined behavior, because it's simply not possible (without unsafe Rust).

<a id="conclusion"></a>

# <a id="conclusion-1"></a> Conclusion

So again, Rust is not easy, but the way the language compels me to write code and I'd really say better and safer code, all in all less boilerplate code, but also more explicit and I'm fully aware about what my code does, is a price worth to pay, I think.

If Rust has no chance to be adopted for your work, nevertheless, I would also recommend that you should give it a try. Definitely, you will write better code next time you use C++. It's not all about the hard stuff, like multithreading, but also the easy parts where we tend to write C++ code a Rust compiler would not compile for good reasons. We can also take those mutability and lifetime concepts into account for C++, but I think it's better to have a compiler doing that job, so I prefer Rust — change my mind :).
