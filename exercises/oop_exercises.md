# OOP Exercises

Work through these in order — each one builds on the last.

The reference implementation lives in [rsbtc/oop_examples/](../rsbtc/oop_examples/).
Read through the source files there before starting:
- `src/traits.rs` — trait definition and implementations
- `src/structs.rs` — struct + impl pattern
- `src/dispatch.rs` — static vs dynamic dispatch
- `src/visibility.rs` — pub/private/pub(crate)

For your practice code, create a new file `rsbtc/oop_examples/src/practice.rs`
and declare it in `main.rs` with `mod practice;`, then call `practice::run()`.

Or create a standalone binary:
```bash
# create rsbtc/oop_examples/src/bin/practice.rs and run with:
# from rsbtc/
cargo run -p oop_examples --bin practice
```

---

## Exercise 1 — Struct and impl

Create a struct called `Wallet` with two private fields:
- `owner: String`
- `balance: u64`

Implement the following methods:
- `new(owner: &str) -> Self` — constructor, balance starts at 0
- `deposit(&mut self, amount: u64)` — add to balance
- `withdraw(&mut self, amount: u64) -> bool` — subtract from balance, return `false` if insufficient funds
- `balance(&self) -> u64` — return current balance

In `main`, create a wallet, deposit some funds, withdraw some, and print the balance.

---

## Exercise 2 — Trait basics

Define a trait called `Describable` with one method:
```rust
fn describe(&self) -> String;
```

Implement it for your `Wallet` from Exercise 1, and for a new struct `Transaction`:
```rust
struct Transaction {
    from: String,
    to: String,
    amount: u64,
}
```

Write a function `print_description` that accepts anything implementing `Describable`
and prints what `describe()` returns. Call it with both a `Wallet` and a `Transaction`.

---

## Exercise 3 — Static vs Dynamic dispatch

Take your `Describable` trait from Exercise 2 and write two versions of `print_description`:

```rust
fn print_static<T: Describable>(item: T) { ... }    // static dispatch
fn print_dynamic(item: &dyn Describable) { ... }    // dynamic dispatch
```

Call both versions with a `Wallet` and a `Transaction`. They should produce identical output.

Then create a `Vec<Box<dyn Describable>>` containing a mix of `Wallet` and `Transaction`
values and iterate over it printing each description. This is only possible with dynamic dispatch —
try it with static dispatch and see what error you get.

---

## Exercise 4 — Default trait methods

Add a method to `Describable` with a default implementation:
```rust
fn summarise(&self) -> String {
    format!("Item: {}", self.describe())
}
```

Override `summarise` for `Transaction` only to include the amount prominently:
```rust
// should produce something like: "Transfer of 500 sats: Alice -> Bob"
```

Leave `Wallet` using the default. Call `summarise()` on both and verify only
`Transaction` uses the custom version.

---

## Exercise 5 — Visibility

Create a module called `bitcoin` inline in your file using `mod bitcoin { }`.

Inside it put:
- A public struct `Block` with a public `height: u64` field and a private `nonce: u64` field
- A public constructor `Block::new(height: u64) -> Self` that sets nonce to 0
- A `pub(crate)` method `mine(&mut self)` that increments nonce by 1
- A public method `height(&self) -> u64`

From outside the module, create a `Block`, call `mine()` a few times, and print the height.
Try to access the `nonce` field directly from outside the module and observe the compile error.

---

## Stretch — Supertraits

Define a trait `Printable` that requires `Describable` as a supertrait:
```rust
trait Printable: Describable {
    fn print(&self) {
        println!("{}", self.describe());
    }
}
```

Implement `Printable` for `Wallet` and `Transaction` (the body can be empty since
`print` has a default implementation). Verify you cannot implement `Printable` for
a type that doesn't also implement `Describable`.
