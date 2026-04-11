# OOP in Rust

Rust has no classes and no inheritance. Instead it uses structs, enums, and
traits. This is not a limitation — it's a different model that avoids many
pitfalls of classical OOP.

---

## Visibility

Rust defaults to private. You opt into visibility explicitly:

```rust
pub fn public_function() {}              // visible everywhere
fn private_function() {}                 // visible only in this module
pub(crate) fn crate_function() {}        // visible within this crate only
pub(super) fn parent_function() {}       // visible to parent module only
pub(self) fn self_function() {}          // same as private — for clarity
pub(in crate::some::path) fn f() {}     // visible from a specific path onwards
```

Struct fields are also private by default — you must explicitly `pub` each one:

```rust
pub struct Block {
    pub height: u64,    // visible
    hash: String,       // private — only accessible via methods
}
```

---

## Structs and impl blocks

Data and behaviour are deliberately separate. A struct holds data. An `impl`
block attaches methods to it.

```rust
pub struct AveragedCollection {
    list: Vec<i32>,     // private fields — callers use methods
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) { ... }   // &mut self = can mutate
    pub fn average(&self) -> f64 { ... }        // &self = read only
    fn update_average(&mut self) { ... }        // no pub = private method
}
```

`&self` — borrows the struct immutably (read only)
`&mut self` — borrows the struct mutably (can modify fields)
`self` — takes ownership (consumes the struct, rare)

---

## Traits

A trait is a contract — a set of methods a type must provide. This is Rust's
answer to interfaces in Java/C# and type classes in Haskell/OCaml.

```rust
trait Quack {
    fn quack(&self);
}

impl Quack for Duck {
    fn quack(&self) { println!("quack"); }
}
```

Unlike Python's duck typing, Rust requires explicit declaration — a type with
a `quack()` method is NOT automatically a `Quack` unless you write `impl Quack for`.

### Marker traits

Traits with no methods. They signal a property to the compiler:

- `Copy` — use copy semantics instead of move semantics
- `Send` — safe to move between threads
- `Sync` — safe to share between threads

---

## Static dispatch

The compiler generates a separate version of the function for each concrete
type used — called monomorphization. No runtime cost, larger binary.

```rust
fn ducks_say<T: Quack>(quacker: T) {
    quacker.quack()
}
```

At compile time `ducks_say::<Duck>` and `ducks_say::<FormalDuck>` are generated
as separate functions. The generic disappears entirely at runtime.

### Hidden trait bound

All generic parameters have an implicit `T: Sized` bound — the type's size must
be known at compile time. Opt out with `T: ?Sized` if you need to work with
dynamically sized types like `str`.

---

## Box\<T\>

`Box<T>` is the simplest smart pointer — it allocates a value on the heap and
gives you an owned pointer to it. When the `Box` goes out of scope, the heap
memory is freed automatically.

```rust
let x = 5;            // i32 on the stack
let y = Box::new(5);  // i32 on the heap, y owns it
```

`Box` is always pointer-sized (8 bytes) regardless of what's inside it. This
matters for trait objects — see below.

---

## Dynamic dispatch

The compiler does NOT generate separate versions. Instead, a vtable (virtual
method table) is used at runtime to look up which method to call. Smaller
binary, small runtime cost, more flexible.

```rust
fn ducks_say(quacker: &dyn Quack) {
    quacker.quack()
}
```

`dyn Quack` is a trait object. It has no known size so it must always be
behind a pointer:

```rust
&dyn Quack       // borrowed trait object
Box<dyn Quack>   // owned trait object, heap allocated
```

### Reading `&[Box<dyn Quack>]` inside out

```
&  [  Box  <  dyn Quack  >  ]
^   ^   ^       ^
|   |   |       trait object — type resolved at runtime via vtable
|   |   heap-allocated owned pointer — always 8 bytes regardless of contents
|   slice — a sequence of elements, all the same size
borrowed — we don't take ownership
```

Full reading: "a borrowed slice of heap-allocated trait objects, each pointing
to some type that implements Quack"

### Why Box is needed in collections

Inside a `Vec` every element must be the same size. `Duck` and `FormalDuck`
are different sizes. `Box<dyn Quack>` solves this — every Box is 8 bytes
(a pointer), the actual data lives separately on the heap:

```
Vec<Box<dyn Quack>>:
  [ ptr → Duck | ptr → FormalDuck | ptr → Duck ]
    ^8 bytes      ^8 bytes           ^8 bytes       ← uniform size in Vec
         ↓              ↓
      heap: Duck    heap: FormalDuck                ← different sizes, doesn't matter
```

### Why static dispatch can't do heterogeneous collections

`T` in `fn f<T: Quack>(items: &[T])` resolves to ONE concrete type at compile
time. Every element must be the same type. There's no way to mix `Duck` and
`FormalDuck` in the same slice with static dispatch.

Use dynamic dispatch when:
- You need a heterogeneous collection (`Vec<Box<dyn Quack>>`)
- The concrete type is only known at runtime
- Binary size matters more than raw speed

---

## Static vs Dynamic dispatch

| | Static (`T: Trait`) | Dynamic (`&dyn Trait`) |
|---|---|---|
| How | Monomorphization | vtable lookup |
| Speed | Faster | Small overhead per call |
| Binary size | Larger | Smaller |
| Flexibility | Type known at compile time | Type can vary at runtime |
| Heterogeneous collections | No | Yes |

---

## Supertraits

A trait can require another trait as a prerequisite:

```rust
trait Saveable: Display {   // can only implement Saveable if you also implement Display
    fn save(&self, path: ...) { ... }
}
```

This is Rust's alternative to inheritance — instead of inheriting behaviour,
you compose it through trait requirements.

---

## Summary

| OOP concept | Rust equivalent |
|---|---|
| Class | `struct` + `impl` block |
| Interface | `trait` |
| Inheritance | Supertraits + composition |
| Virtual methods | `dyn Trait` (dynamic dispatch) |
| Private/public | `pub`, `pub(crate)`, `pub(super)` |
| Static methods | Associated functions (no `self` param) |
| Instance methods | Methods with `&self` or `&mut self` |
