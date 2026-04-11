# Rust Memory Model

Rust has no garbage collector. Memory is managed by the compiler through three
related concepts: ownership, borrowing, and lifetimes. Together they guarantee
memory safety at compile time — no runtime crashes, no dangling pointers, no
data races.

---

## 1. Ownership

Every value has exactly one owner. When the owner goes out of scope, the value
is freed automatically — no `free()`, no GC pause.

```rust
fn main() {
    let x = String::from("hello");  // x owns this string, heap allocated
}                                   // x goes out of scope, string is freed
```

**Move semantics** — passing a value to a function transfers ownership. The
original variable can no longer be used.

```rust
fn consume(value: String) {}        // takes ownership, drops it at end

let x = String::from("hello");
consume(x);
println!("{}", x);                  // COMPILE ERROR — x was moved
```

To get ownership back, return the value:

```rust
fn consume(value: String) -> String { value }

let x = String::from("hello");
let x = consume(x);                 // ownership returned, shadowing x
println!("{}", x);                  // fine
```

---

## 2. Borrowing (References)

To use a value without taking ownership, borrow it with `&`. The original owner
keeps ownership and the value is not freed when the borrow ends.

```rust
fn print(s: &String) {              // borrows, does not own
    println!("{}", s);
}                                   // borrow ends here, s is not freed

let x = String::from("hello");
print(&x);
print(&x);                          // fine — x still owns the string
```

### Immutable vs Mutable borrows

| | Syntax | How many at once |
|---|---|---|
| Immutable borrow | `&T` | unlimited |
| Mutable borrow | `&mut T` | exactly one |

**You can never have a mutable and immutable borrow active at the same time.**

```rust
let mut s = String::from("hello");

let r1 = &s;                        // immutable borrow
let r2 = &mut s;                    // COMPILE ERROR — s already borrowed
```

This rule prevents data races entirely — if someone is mutating, nobody can
read. If many are reading, nobody can mutate.

### Why `&str` over `&String` for parameters

`&str` is more flexible — it accepts both `String` references and string
literals. `&String` only accepts `String` references.

```rust
fn greet(name: &str) {}             // accepts both

greet("literal");                   // &str — works
greet(&String::from("owned"));      // &String coerces to &str — works
```

---

## 3. Dangling References — Impossible in Rust

A dangling pointer points to memory that has been freed. Rust's borrow checker
makes this a compile error.

```rust
fn give_me_a_ref() -> &String {
    let temp = String::from("hello");
    &temp                           // COMPILE ERROR — temp is freed when
}                                   // function ends, reference would dangle
```

Fix: return owned data instead.

```rust
fn give_me_a_string() -> String {
    String::from("hello")           // ownership moves to caller — no dangling
}
```

---

## 4. Lifetimes

A lifetime tracks how long a reference is valid. The compiler infers them in
most cases — you only write them explicitly when it can't figure out the
connection between input and output references.

```rust
// 'a links the output reference to the input references.
// "the returned reference lives as long as both inputs"
fn max_ref<'a>(left: &'a i32, right: &'a i32) -> &'a i32 {
    if left > right { left } else { right }
}
```

Without `'a` the compiler can't verify the returned reference won't outlive
the data it points to.

`'static` is a special lifetime meaning "valid for the entire program":

```rust
static N: &'static i32 = &42;      // lives forever
```

---

## Summary

| Concept | Rule | Why |
|---|---|---|
| Ownership | One owner, freed on drop | No double-free, no leaks |
| Move | Passing by value transfers ownership | No use-after-free |
| Borrow `&T` | Many immutable borrows allowed | Safe concurrent reads |
| Borrow `&mut T` | Only one mutable borrow at a time | No data races |
| Lifetimes | References can't outlive their data | No dangling pointers |

All of this is checked at compile time. The resulting binary has none of the
overhead of a garbage collector and none of the unsafety of manual memory
management.
