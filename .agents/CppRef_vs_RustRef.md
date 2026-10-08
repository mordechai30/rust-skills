# C++ references vs Rust references: lifetime enforcement and exclusive access

## The mindset difference

**C++ reference:** an alias for an object, with no information about how long the object lives or who else is touching it. The compiler checks types and constness. *You* guarantee lifetime and aliasing.

**Rust reference:** a *borrow*, a temporary permission with a compile-time-checked duration (lifetime) and a compile-time-checked access mode (shared `&T` or exclusive `&mut T`). The compiler guarantees both, and it rejects code it cannot prove.

| | C++ `T&` / `const T&` | Rust `&T` / `&mut T` |
|---|---|---|
| Can dangle | yes (silently, UB) | no (in safe code) |
| Lifetime tracked by compiler | no | yes (inferred or annotated) |
| Multiple writers via aliases | allowed | forbidden |
| Writer + readers at once | allowed | forbidden |
| `const` means | *I* won't modify through this name | `&T`: nobody can modify while it lives (except interior mutability) |
| Rebindable | no (always bound to the original object) | the variable can be rebound; the reference value itself is a normal value |
| Can be null | no (but can dangle) | no, and cannot dangle |
| Cost | zero | zero (all checks are compile-time) |

---

## 1. Lifetime enforcement

**C++:** the compiler checks that a reference is *initialized*, never that its target is *still alive*.
**Rust:** every reference carries a lifetime. The compiler verifies that the reference never outlives the thing it points to.

### Example 1: returning a reference to a local

```cpp
// C++20: compiles (GCC/Clang warn at best). Using the result is UB.
const std::string& make_greeting() {
    std::string s = "hello";
    return s;                    // s destroyed at the closing brace
}

int main() {
    const std::string& g = make_greeting();
    std::cout << g;              // dangling reference: UB
}
```

```rust
// Rust: rejected at compile time.
fn make_greeting() -> &String {          // error[E0106]: missing lifetime specifier
    let s = String::from("hello");
    &s
}

// Even with an annotation, it cannot work:
fn make_greeting<'a>() -> &'a String {
    let s = String::from("hello");
    &s                                    // error[E0515]: cannot return reference to local variable `s`
}
```

**Mindset:** In C++ you must remember that a local dies at `}`. In Rust the compiler reasons about it: there is no lifetime `'a` that can be longer than the function's own stack frame. The fix is to **return ownership**:

```rust
fn make_greeting() -> String { String::from("hello") }   // caller owns it
```

### Example 2: a reference outliving its source (scope escape and `string_view`)

```cpp
// C++20: all of this compiles.
std::string_view pick() {
    std::string tmp = "temporary";
    return tmp;                  // string_view into tmp, which dies on return
}

int main() {
    std::string_view v;
    {
        std::string s = "inner";
        v = s;                   // v points into s
    }                            // s destroyed
    std::cout << v;              // UB: reads freed memory
}
```

```rust
fn main() {
    let r: &str;
    {
        let s = String::from("inner");
        r = &s;                  // error[E0597]: `s` does not live long enough
    }                            // `s` dropped while still borrowed
    println!("{r}");             // the borrow is used here, so the error fires
}
```

The compiler's message points at three spots: where the borrow starts, where the owner dies, and where the reference is later used. That is exactly the proof the compiler found.

### How Rust expresses relationships: lifetime annotations

When a function takes several references and returns one, the compiler needs to know which input the output borrows from:

```rust
// "The returned reference lives no longer than BOTH inputs."
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("long string");
    let result;
    {
        let b = String::from("short");
        result = longest(&a, &b);
    }                                    // b dies here
    println!("{result}");                // error[E0597]: `b` does not live long enough
}
```

The annotation is a **contract in the signature**. Callers cannot violate it, and the function body is checked against it. In C++, the same contract lives only in a comment ("returns a reference to the longer arg, don't use after either dies"). Lifetime annotations do not change runtime behavior; they exist only for the compiler.

**Lifetime elision:** for the common case, you write nothing. `fn first_word(s: &str) -> &str` is understood as "the output borrows from the single input". With `&self` methods, the output is tied to `self`.

---

## 2. Exclusive access (the borrow checker)

**C++:** any number of `T&` aliases may exist, all readable and writable. The compiler assumes nothing about aliasing beyond strict-aliasing rules for *different types*.
**Rust:** at any moment a place has either **many `&T`** or **exactly one `&mut T`**. This is called "aliasing XOR mutability". It is why `&mut` means *exclusive*, not just *mutable*.

### Example 3: iterator invalidation (mutating a container while holding a reference into it)

```cpp
// C++20: compiles. UB if push_back reallocates.
std::vector<int> v = {1, 2, 3};
int& first = v[0];
v.push_back(4);                  // may reallocate: `first` now dangles
first = 99;                      // UB (works "by luck" if capacity was enough)
```

```rust
let mut v = vec![1, 2, 3];
let first = &mut v[0];           // exclusive borrow of v (through the index)
v.push(4);                       // error[E0499]: cannot borrow `v` as mutable more than once
*first = 99;                     // the first borrow is still needed here
```

The same applies with a shared reference:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];               // shared borrow
v.push(4);                       // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
println!("{first}");
```

**Mindset:** C++ gives you the footgun and trusts you to know `push_back` can invalidate references (the docs say so). Rust encodes "this operation may invalidate" as "this operation needs `&mut self`", and `&mut self` cannot coexist with any other live borrow. No documentation lookup is required, and the compiler enforces it.

The fix is to end the borrow before mutating (borrows end at their *last use*, not at scope end):

```rust
let mut v = vec![1, 2, 3];
let first = v[0];                // copies the i32 out; no borrow remains
v.push(4);                       // fine
println!("{first}");
```

### Example 4: aliasing two mutable references to the same object

```cpp
// C++20: compiles and runs, but the intent is easy to get wrong.
void add_twice(int& a, int& b) {
    a += 10;
    b += 10;
}

int main() {
    int x = 1;
    add_twice(x, x);             // a and b alias the same int; x == 21, not 11 + 11 = 22 "as if distinct"
}

// More dangerous: the compiler must assume a and b MAY alias,
// so it cannot keep values in registers or reorder loads and stores.
void scale(float* out, const float* in, size_t n) { /* ... */ }
// scale(buf, buf, n) is legal; the author may have assumed no overlap.
// (Hence __restrict as a non-standard promise you have to uphold yourself.)
```

```rust
fn add_twice(a: &mut i32, b: &mut i32) {
    *a += 10;
    *b += 10;
}

fn main() {
    let mut x = 1;
    add_twice(&mut x, &mut x);   // error[E0499]: cannot borrow `x` as mutable more than once at a time
}
```

Rust can **prove** `a` and `b` never alias, so it can tell LLVM to optimize aggressively (`noalias`), the equivalent of `restrict`, but guaranteed rather than promised. When you *do* need two mutable views into one container, you split it so the compiler can see they are disjoint:

```rust
let mut arr = [1, 2, 3, 4];
let (left, right) = arr.split_at_mut(2);   // two non-overlapping &mut slices
left[0] = 10;
right[0] = 30;                              // fine, provably disjoint
```

Shared mutation, when truly required, is made explicit in the type (`Cell`, `RefCell`, `Mutex`, atomics), moving the check from compile time to run time (or to hardware atomics) instead of silently permitting it.

---

## Summary: the two mindsets side by side

| Situation | C++20+ | Rust |
|---|---|---|
| Return ref to local | compiles, UB on use | compile error E0515 |
| Ref outlives scope of source | compiles, UB on use | compile error E0597 |
| Hold ref across `push_back` | compiles, UB if reallocated | compile error E0502/E0499 |
| Two `&mut` to same object | compiles, silent aliasing surprises | compile error E0499 |
| Who proves safety | **the programmer** (docs, reviews, sanitizers) | **the compiler** (signatures and borrow rules) |
| Cost of the guarantee | none at runtime | none at runtime; paid in design effort |
| Escape hatch | everything is the escape hatch | `unsafe`, `RefCell`, `Rc`/`Arc`, `Mutex`, cloning |

**Practical consequences of the mindset shift:**

1. **Think in ownership first.** In C++ you reach for `const T&` as the default parameter. In Rust, `&T` is also the default for read-only access, but returning or storing a reference forces you to think about *who owns the data*. Often the answer is "return an owned value" or "store a `String`, not a `&str`".
2. **Structs holding references need lifetime parameters** (`struct Parser<'a> { input: &'a str }`). In C++ a `string_view` member silently dangles if the source dies. In Rust the struct cannot outlive `'a`.
3. **Short borrows are idiomatic.** Rust's non-lexical lifetimes end a borrow at its last use, so you rarely fight the checker if you keep borrows small and avoid holding references across mutations.
4. **When the checker says no, it usually found a real bug**, but not always. It is conservative, so some correct programs are rejected (for example, certain self-referential structures or graph-like data). The remedies are indices instead of references, `Rc`/`Arc` plus `RefCell`/`Mutex`, or a small, encapsulated `unsafe`.
5. **C++ is catching up partially**, with sanitizers (ASan, TSan), `[[clang::lifetimebound]]`, hardened standard libraries, and static analysis (clang-tidy, lifetime profiles proposals). These are *opt-in, partial, and heuristic*. Rust's checks are *mandatory, complete for safe code, and part of the type system*.