# APIs and language contracts: Rust versus C++

Examples use Rust 1.99, edition 2024, and C++20 unless marked C++23. Rust `compile_fail` examples are  
deliberately rejected.

## Traits versus structural constraints

**Use:** bounds for the operations an API needs. Use associated types for one implementation-selected  
related type. Choose generics/`impl Trait` for concrete dispatch and `dyn Trait` for genuine runtime  
polymorphism. Do not ban dynamic dispatch or add a generic layer without a caller benefit.

**Difference:** a C++20 concept can test whether an expression is valid without an explicit declaration of  
participation. A Rust trait requires an implementation; matching method names alone is insufficient. Rust  
generic bodies are checked against their declared bounds. Both approaches still need semantic laws such as  
consistent equality/hashing or ordering; compilation does not prove them.

```cpp
#include <cassert>
#include <concepts>
template<class T>
concept Sized = requires(const T& value) { { value.size() } -> std::convertible_to<unsigned>; };
struct Packet { unsigned size() const { return 7; } };
unsigned length(const Sized auto& value) { return value.size(); }
int main() { assert(length(Packet{}) == 7); }
```

```rust
trait SizedPacket { fn size(&self) -> usize; }
struct Packet;
impl SizedPacket for Packet { fn size(&self) -> usize { 7 } }
fn length(value: &impl SizedPacket) -> usize { value.size() }
fn main() { assert_eq!(length(&Packet), 7); }
```

Trait coherence restricts overlapping implementations and foreign-trait implementations on foreign types.  
Use a local newtype to establish a contract instead of trying to attach arbitrary behavior globally. C++  
operators/free functions do not have Rust's same orphan/coherence rules. Sealing and blanket impls affect  
downstream extension; choose them as public API commitments.

### Extending a foreign type

C++ permits this ordinary global overload for a standard-library operand. Rust cannot implement the  
foreign `Display` trait on foreign `Vec`; wrap it in a local type. Neither example edits the standard  
library's namespace/implementation.

```cpp
#include <cassert>
#include <ostream>
#include <sstream>
#include <vector>
std::ostream& operator<<(std::ostream& out, const std::vector<int>& values) {
    return out << values.size();
}
int main() {
    std::ostringstream out;
    out << std::vector<int>{1, 2};
    assert(out.str() == "2");
}
```

```rust,compile_fail
impl std::fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.len())
    }
}
fn main() {} // E0117: foreign trait on foreign type.
```

```rust
struct Packet(Vec<i32>);
impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.len())
    }
}
fn main() { assert_eq!(Packet(vec![1, 2]).to_string(), "2"); }
```

## Consuming methods and closure capabilities

**Use:** consuming `self` methods for real ownership transitions; `&self` for shared operations;  
`&mut self` for exclusive operations. For callbacks, choose `FnOnce` when one invocation suffices, `FnMut`  
for repeated mutation, and `Fn` for repeated shared calls. Do not require a stronger callable contract  
than needed.

**Difference:** C++ rvalue-qualified methods restrict the value category of the call but do not make the  
caller's object statically unusable. A Rust consuming method invalidates a non-`Copy` source binding. Rust  
closure traits also express whether calling consumes captured state; a C++ lambda's type does not provide  
the same standard ownership tracking.

```cpp
#include <cassert>
#include <utility>
struct Session {
    int finish() && { return 7; }
};
int main() {
    Session session;
    assert(std::move(session).finish() == 7);
    // session still exists; its valid operations depend on its implementation.
}
```

```rust
struct Session;
impl Session { fn finish(self) -> i32 { 7 } }
fn once<F: FnOnce() -> String>(action: F) -> String { action() }
fn main() {
    let session = Session;
    assert_eq!(session.finish(), 7);
    let message = String::from("done");
    assert_eq!(once(move || message), "done");
}
```

```rust,compile_fail
struct Session;
impl Session { fn finish(self) {} }
fn main() {
    let session = Session;
    session.finish();
    session.finish(); // E0382: consuming transition already used this owner.
}
```

A consuming callback makes the distinction visible too:

```cpp
#include <cassert>
#include <memory>
#include <utility>
int main() {
    auto take = [value = std::make_unique<int>(7)]() mutable { return std::move(value); };
    auto first = take();
    auto second = take(); // Callable again; returns an empty owner in this implementation.
    assert(*first == 7 && !second);
}
```

```rust,compile_fail
fn main() {
    let value = String::from("job");
    let take = move || value;
    let first = take();
    let second = take(); // E0382: FnOnce call consumed the closure.
    println!("{first} {second}");
}
```

Typestate is available in both languages. Rust's consuming transitions add enforced ownership loss of the  
prior state; do not present all typestate design as uniquely Rust. Use runtime enums when retries,  
persistence, or dynamically changing states make static transitions cumbersome.

## Exhaustive states and error propagation

**Use:** enums for mutually exclusive payload-bearing states, `Option` for absence, and `Result` for  
recoverable failures. Propagate with `?` when no local recovery is needed; retain structured errors at  
decision boundaries. Do not unwrap untrusted failures or treat a panic as a normal domain result.

**Difference:** Rust `match` requires exhaustiveness for the type visible to the caller. A C++ enum  
`switch` can compile without handling every enumerator. C++ `variant` plus an exhaustive visitor can  
provide similarly complete payload handling. Rust `?` exits the current function with its error/absence;  
C++23 `expected` has explicit operations but no direct language `?` propagation. C++20 fallback: the  
project result type or distinct `variant` alternatives.

```cpp
#include <cassert>
enum class State { ready, closed };
int inspect(State state) {
    switch (state) { case State::ready: return 1; }
    return 0; // A missing enumerator is accepted by the language.
}
int main() { assert(inspect(State::closed) == 0); }
```

```rust,compile_fail
enum State { Ready, Closed }
fn inspect(state: State) -> i32 {
    match state { State::Ready => 1 } // E0004: Closed must be handled.
}
fn main() {}
```

```cpp
#include <cassert>
#include <charconv>
#include <cstdint>
#include <string_view>
#include <system_error>
#include <variant>
std::variant<std::uint16_t, std::errc> parse_limit(std::string_view text) {
    std::uint16_t value{};
    auto parsed = std::from_chars(text.data(), text.data() + text.size(), value);
    if (parsed.ec != std::errc{}) return parsed.ec;
    if (parsed.ptr != text.data() + text.size()) return std::errc::invalid_argument;
    return value;
}
int main() { assert(std::holds_alternative<std::errc>(parse_limit("invalid"))); }
```

```rust
fn parse_limit(text: &str) -> Result<u16, std::num::ParseIntError> {
    let value = text.parse::<u16>()?;
    Ok(value)
}
fn main() { assert!(parse_limit("invalid").is_err()); }
```

Panic unwinding may run `Drop`; abort does not. The language's memory-safety guarantee does not promise  
application rollback. Keep partially updated state valid before calling code that can panic. C++  
exceptions and Rust panics are not mechanically interchangeable error APIs.

## Iteration consumes or borrows

**Use:** `.iter()` for shared iteration, `.iter_mut()` for exclusive iteration, and `.into_iter()` for  
consumption when the input is no longer needed. Use a loop when early exits or state changes are clearer.  
Avoid intermediate collection unless ownership or repeated work requires materialization.

**Difference:** Rust `for value in vec` consumes an owned `Vec`; a C++ range-for over a vector does not  
itself consume the container, even when elements are copied or the range expression uses `std::move`.  
Rust iterator adapters store and enforce borrow relationships; C++20 range views require the programmer's  
owner/invalidation contract. Both are lazy and can silently truncate with zip-like operations unless equal  
lengths are checked.

```cpp
#include <cassert>
#include <vector>
int main() {
    std::vector<int> values{1, 2};
    int sum = 0;
    for (auto value : values) sum += value;
    assert(sum == 3 && values.size() == 2);
}
```

```rust
fn main() {
    let values = vec![1, 2];
    let sum: i32 = values.iter().sum();
    assert_eq!(values.len(), 2);
    let owned_sum: i32 = values.into_iter().sum();
    assert_eq!(sum, owned_sum); // values has been consumed.
}
```

## Const evaluation and macro expansion

**Use:** `const fn` where compile-time use is meaningful. Use functions/generics before macros. Use  
precise macro fragments, evaluate input expressions once, and use `$crate` for defining-crate paths in  
exported declarative macros.

**Difference:** C++ `constexpr` on an ordinary function also constrains declaration rules differently from  
Rust `const fn`; neither forces all calls to happen at compile time. Rust declarative macros match token  
syntax and provide mixed-site hygiene; C++ preprocessor macros perform textual/token substitution without  
Rust's hygiene. Hygiene does not prevent duplicated evaluation or establish correctness of generated  
unsafe code.

```cpp
#include <cassert>
constexpr unsigned twice(unsigned n) { return n * 2; }
#define TWICE(n) ((n) + (n))
int main() {
    static_assert(twice(3) == 6);
    int calls = 0;
    auto next = [&] { return ++calls; };
    (void)TWICE(next()); // Calls next twice; no result-order assumption is needed.
    assert(calls == 2);
}
```

```rust
const fn twice(n: u32) -> u32 { n * 2 }
macro_rules! twice_once { ($value:expr) => {{ let value = $value; value + value }}; }
fn main() {
    const SIX: u32 = twice(3);
    let mut calls = 0;
    let mut next = || { calls += 1; 3 };
    assert_eq!(twice_once!(next()), SIX);
    assert_eq!(calls, 1);
}
```

For cross-edition/public macros, test external use; an in-crate expansion can hide path or visibility  
mistakes. Cargo features configure compilation and may unify through dependencies, unlike a local runtime  
switch. Use a supported feature matrix rather than assuming all combinations are valid.

## Sources

- [Traits](https://doc.rust-lang.org/reference/items/traits.html)
-  
  [Coherence](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)
- [Closure traits](https://doc.rust-lang.org/reference/types/closure.html#call-traits-and-coercions)
- [Match](https://doc.rust-lang.org/reference/expressions/match-expr.html)
- [Iterator](https://doc.rust-lang.org/std/iter/index.html)
- [Question mark][width-1]
- [Const evaluation](https://doc.rust-lang.org/reference/const_eval.html)
- [Macro hygiene](https://doc.rust-lang.org/reference/macros-by-example.html#hygiene)
- [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)

[width-1]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator
