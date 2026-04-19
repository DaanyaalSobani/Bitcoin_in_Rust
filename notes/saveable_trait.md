# The `Saveable` Trait and `IoResult`

Reference: `lib/src/util.rs` — `Saveable`, lines 30–44

---

## `IoResult`

At the top of `util.rs`:

```rust
use std::io::{Result as IoResult, Write};
```

`IoResult` is just a type alias — `std::io::Result<T>` renamed to avoid clashing with
`std::result::Result`. They are the same type:

```rust
// inside the standard library:
pub type Result<T> = std::result::Result<T, std::io::Error>;
```

So `IoResult<Self>` expands to `Result<Self, std::io::Error>` — either you get the
value back, or you get an IO error (file not found, permission denied, disk full, etc.).

The rename is purely to avoid ambiguity. If you wrote `use std::io::Result` without the
alias, the name `Result` would conflict with the prelude's `Result` everywhere in the
file.

---

## The `Saveable` trait

```rust
pub trait Saveable
where
    Self: Sized,
{
    fn load<I: Read>(reader: I) -> IoResult<Self>;
    fn save<O: Write>(&self, writer: O) -> IoResult<()>;
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> IoResult<()> { ... }
    fn load_from_file<P: AsRef<Path>>(path: P) -> IoResult<Self> { ... }
}
```

The trait has two required methods (`load`, `save`) and two default methods
(`save_to_file`, `load_from_file`) that are implemented for free on top of the required ones.

---

## `load` and `save` — required, generic over IO source

```rust
fn load<I: Read>(reader: I) -> IoResult<Self>;
fn save<O: Write>(&self, writer: O) -> IoResult<()>;
```

`I: Read` and `O: Write` are generic over anything that implements the `Read` or `Write`
traits. This means the same `load`/`save` implementation works for:

- `File` — reading from or writing to disk
- `Vec<u8>` — reading from or writing to an in-memory buffer
- a network socket — anything that implements the trait

The implementor defines how to serialise/deserialise. The trait doesn't care what the
source or destination is.

---

## `save_to_file` and `load_from_file` — default implementations

```rust
fn save_to_file<P: AsRef<Path>>(&self, path: P) -> IoResult<()> {
    let file = File::create(&path)?;
    self.save(file)
}
fn load_from_file<P: AsRef<Path>>(path: P) -> IoResult<Self> {
    let file = File::open(&path)?;
    Self::load(file)
}
```

These are provided for free — any type that implements `load` and `save` automatically
gets `save_to_file` and `load_from_file` without writing any extra code.

`P: AsRef<Path>` — accepts anything that can be viewed as a path: a `&str`, a `String`,
a `PathBuf`. You don't have to construct a `Path` explicitly to call these.

`Self::load(file)` — `Self` means "whatever type is implementing this trait". When
`Blockchain` implements `Saveable`, this calls `Blockchain::load(file)`.

---

## `where Self: Sized`

`load` and `load_from_file` return `Self` by value:

```rust
fn load<I: Read>(reader: I) -> IoResult<Self>;
```

Returning by value requires the compiler to know the size of `Self` at compile time so
it knows how much stack space to allocate. `where Self: Sized` enforces this — it
prevents `Saveable` from being implemented on dynamically-sized types (like `[u8]` or
`dyn Trait`) that have no fixed size.

Almost every normal type (`Blockchain`, `Block`, `u64`, structs, enums) is `Sized` by
default, so this bound is rarely something you have to think about — it just closes off
an edge case.

---

## `load_from_file` has no `self` — it's a static method

`save_to_file` takes `&self` because you already have the value you want to save.
`load_from_file` takes no `self` because you're creating the value — there's nothing
to call it on yet. It's called on the type directly:

```rust
let blockchain = Blockchain::load_from_file("blockchain.dat")?;
//  result ────────────────────────────────────────────────────┘
//  no existing Blockchain needed to call this
```

---

## Summary

| | `load` / `save` | `load_from_file` / `save_to_file` |
|---|---|---|
| Required to implement | yes | no — provided for free |
| Works with | any `Read` / `Write` | files only |
| Takes `self` | `save` yes, `load` no | `save_to_file` yes, `load_from_file` no |
| Returns | `IoResult<Self>` / `IoResult<()>` | same |
