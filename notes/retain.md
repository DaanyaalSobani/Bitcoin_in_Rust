# `Vec::retain` — in-place filtering

Reference: `lib/src/types.rs` — `Blockchain::add_block`
Examples: `practice/retain_examples/src/main.rs`

---

## What it does

Keeps only the elements for which the closure returns `true`. Removes everything else
in place — no new `Vec` is allocated.

```rust
pub fn retain<F>(&mut self, f: F)
where F: FnMut(&T) -> bool
```

Requires `&mut self` — it mutates the `Vec` directly.

---

## Scalars

```rust
let mut numbers = vec![1, 2, 3, 4, 5, 6];
numbers.retain(|x| x % 2 == 0);
// [2, 4, 6]
```

---

## Tuples — destructuring in the closure parameter

When the `Vec` holds tuples you can destructure directly in the closure signature:

```rust
let mut transactions: Vec<(String, u64)> = vec![...];

transactions.retain(|(_, amount)| *amount >= 10_000);
//                   ^^^^^^^^^^ destructure the tuple
//                    _ = ignore first field
//                       amount = bind second field
//                                * dereference &u64 to compare
```

`_` is the Rust convention for "I don't need this value" — the compiler won't warn about
it being unused.

---

## In the Bitcoin codebase

`lib/src/types.rs` — after a block is added, remove its transactions from the mempool:

```rust
self.mempool
    .retain(|(_, tx)| !block_transactions.contains(&tx.hash()));
```

- `mempool` is `Vec<(SomeType, Transaction)>`
- `(_, tx)` — ignore the first field, bind the transaction as `tx`
- keep transactions whose hash is NOT in the newly mined block

---

## Is `retain` part of a trait?

No — it is an inherent method on `Vec<T>`, defined directly in the standard library.
`HashMap` also has a `retain` but they are separate implementations, not a shared trait.
There is no `Retain` trait.

---

## `retain` vs `filter`

| | `retain` | `filter` |
|---|---|---|
| Mutates in place | yes | no |
| Returns new collection | no | yes (iterator) |
| Ownership | keeps the original `Vec` | consumes the iterator |

```rust
// retain — mutates v, no new allocation
v.retain(|x| x % 2 == 0);

// filter — produces a new iterator, collect into new Vec
let evens: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
```

Use `retain` when you want to modify the existing `Vec`.
Use `filter` when you want to produce a new collection without touching the original.
