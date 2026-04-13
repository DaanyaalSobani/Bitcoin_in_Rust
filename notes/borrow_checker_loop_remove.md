# Borrow Checker — Why You Can't Remove Inside the Loop

Reference: `lib/src/types.rs` — `Blockchain::add_to_mempool`, lines 64–96

---

## The code that failed

The original (broken) version had `self.mempool.remove(idx)` **inside** the `for` loop:

```rust
// types.rs lines 76–86 (broken version)
if let Some((idx, (_, referencing_transaction))) = referencing_transaction {
    for input in &referencing_transaction.inputs {   // line 77 — immutable borrow of self.mempool begins
        self.utxos
            .entry(input.prev_transaction_output_hash)
            .and_modify(|(marked, _)| {
                *marked = false;
            });
        self.mempool.remove(idx);   // ERROR — mutable borrow of self.mempool while immutable is live
    }
}
```

**Compiler error:**
```
error[E0502]: cannot borrow `self.mempool` as mutable because it is also borrowed as immutable
  --> src/types.rs:85:21
   |
77 |         for input in &referencing_transaction.inputs {
   |                      --------------------------------
   |                      immutable borrow occurs here
85 |                     self.mempool.remove(idx);
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
   |                     immutable borrow later used here
```

---

## Untangling the two variables named `referencing_transaction`

Before tracing the borrow, it's important to notice that the name `referencing_transaction`
is used **twice** in this block — for two different variables with two different types.
This is variable shadowing and it makes the code easy to misread.

**First binding — line 66:**

```rust
// type: Option<(usize, &(Hash, Transaction))>
let referencing_transaction =
    self.mempool
        .iter()
        .enumerate()
        .find(|(_, (_, transaction))| { ... });
```

This is the raw result of `.find()` — an `Option` that either holds `None` (not found)
or `Some((index, &mempool_entry))`. The `&` matters: `.iter()` yields references into
the Vec, not copies. This variable is a pointer into `self.mempool`.

**Second binding — line 76 (shadows the first):**

```rust
// type: &Transaction  (unwrapped and destructured from the Option above)
if let Some((idx, (_, referencing_transaction))) = referencing_transaction {
```

This `if let` destructures the `Option` from line 66. It:
- unwraps the `Some` (the check)
- binds `idx` — the index position in the mempool Vec
- binds a new `referencing_transaction` — the `&Transaction` inside the tuple, shadowing the `Option` above

After line 76, `referencing_transaction` refers to this second binding (`&Transaction`),
not the `Option`. The first binding is gone. Both are still pointers into `self.mempool`'s
memory — the shadowing does not copy anything.

In clearer names the same code would be:

```rust
let search_result: Option<(usize, &(Hash, Transaction))> = self.mempool.iter()...find(...);

if let Some((idx, (_, found_tx))) = search_result {
    // found_tx: &Transaction — a reference into self.mempool
    for input in &found_tx.inputs { ... }
    self.mempool.remove(idx);
}
```

---

## How the borrow was created — tracing the chain

The immutable borrow starts at line 67 when `.iter()` is called:

```rust
// types.rs lines 66–75
let referencing_transaction =      // Option<(usize, &(Hash, Transaction))>
    self.mempool                   // .iter() borrows self.mempool immutably here
        .iter()
        .enumerate()
        .find(|(_, (_, transaction))| {
            transaction
                .outputs
                .iter()
                .any(|output| output.hash() == input.prev_transaction_output_hash)
        });
```

`.iter()` on a `Vec<T>` yields `&T` — references **into** the Vec. It does not copy
the data. The `&Transaction` inside the `Option` is a pointer into `self.mempool`'s
backing memory. When line 76 destructures it into the second `referencing_transaction`,
it is still the same pointer — just with a narrower type.

The chain of borrows:

```
self.mempool: Vec<(Hash, Transaction)>         (lives on the heap)
      │
      └── .iter() (line 67) ──────────────── immutable borrow of self.mempool begins
                │
                └── referencing_transaction (line 66)   = Option<(usize, &(Hash, Transaction))>
                              │
                └── referencing_transaction (line 76)   = &Transaction  ← still a pointer into mempool
                              │
                              └── .inputs               = &Vec<TransactionInput> ← pointer into mempool
                                        │
                                        └── for input in &...inputs (line 77)
                                            ↑ loop holds immutable borrow for its full duration
```

At line 77, `for input in &referencing_transaction.inputs` begins. This `for` loop holds
an **immutable borrow of `self.mempool`** for every iteration — not just the first. The
borrow is not released until the loop body exits completely.

`self.mempool.remove(idx)` on line 85 (originally inside the loop) requires a **mutable
borrow** of `self.mempool`. Both borrows are simultaneously active.

Rust's invariant:

> **You cannot have an active mutable borrow while any immutable borrow of the same
> data is still alive.**

The borrow checker rejects it at compile time.

---

## Why this is a memory safety issue, not just a logic concern

If Rust allowed `remove` inside the loop:

1. **Line 77** — `for input in &referencing_transaction.inputs` obtains a pointer into
   `self.mempool`'s heap allocation.
2. **Line 85** — `self.mempool.remove(idx)` removes an element. `Vec::remove` shifts all
   subsequent elements left, and if the Vec decides to reallocate (to shrink capacity),
   the entire backing array moves to a new memory address.
3. The pointer from step 1 now points to **freed or shifted memory**. Any access to
   `input` in subsequent iterations is a **use-after-free**.

In C this is undefined behaviour and can corrupt memory silently or crash. In Python the
runtime manages memory via a garbage collector so you get a logic bug (the index shifts
under you) rather than a crash — but you'd still get wrong results or a `ValueError`.
Rust prevents either outcome at compile time.

---

## The fix — move `remove` before the loop (current code, lines 76–86)

```rust
// types.rs lines 76–86 (current, fixed version)
if let Some((idx, (_, referencing_transaction))) = referencing_transaction {
    for input in &referencing_transaction.inputs {   // line 77 — immutable borrow begins
        self.utxos
            .entry(input.prev_transaction_output_hash)
            .and_modify(|(marked, _)| {
                *marked = false;
            });
    }                                                // line 83 — immutable borrow ends here
    // remove the transaction from the mempool
    self.mempool.remove(idx);                        // line 85 — mutable borrow, safe: no live immutable borrow
}
```

The borrow checker tracks borrows at the **statement level**, not just the scope block
level. After line 83 (the closing `}` of the `for` loop), the loop's immutable borrow of
`self.mempool` is dead. `referencing_transaction` is never used again after that point.
By the time line 85 runs, `self.mempool` is fully released and the mutable borrow
is allowed.

---

## Why the alternative fix (clone) also works

An alternative is to clone `referencing_transaction.inputs` before the loop:

```rust
if let Some((idx, (_, referencing_transaction))) = referencing_transaction {
    let inputs = referencing_transaction.inputs.clone();  // independent copy
    self.mempool.remove(idx);   // safe — referencing_transaction no longer used
    for input in &inputs {      // iterates our own Vec, no mempool borrow at all
        self.utxos.entry(...).and_modify(...);
    }
}
```

`inputs` is a freshly allocated `Vec` — it does not borrow `self.mempool`. The loop
iterates over that independent copy, so the remove and the loop never compete for the
same borrow. The cost is a heap allocation and a copy of the inputs. The "move remove
before loop" version avoids that copy entirely — which is why the current code uses it.

---

## Compare to Python

```python
# Python — no compile error, but wrong behaviour
for input in referencing_transaction.inputs:
    self.utxos[...] = False
    self.mempool.remove(idx)  # succeeds on first iteration
                              # second iteration: idx no longer valid; wrong element
                              #                  or ValueError if not found
```

Python catches nothing at compile time. The bug surfaces at runtime, possibly only when
a transaction has more than one input. Rust catches it before the program runs, with an
error message that names the exact lines where the borrows conflict.

---

## The general rule

| Situation | Rust | Python |
|---|---|---|
| Mutate a collection while iterating it | compile error (E0502) | runtime bug or exception |
| Use a reference after the data moves | compile error | not applicable (GC) |
| Read and write same data concurrently | compile error | runtime error (with locks) |

Rust converts an entire class of memory-safety bugs into compile-time errors. The cost
is that you must reason about borrow lifetimes explicitly. The benefit is that these bugs
are structurally impossible in a compiled Rust binary.
