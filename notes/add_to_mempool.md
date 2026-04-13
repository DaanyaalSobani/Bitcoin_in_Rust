# `add_to_mempool` — walkthrough

Reference: `lib/src/types.rs` — `Blockchain::add_to_mempool`, lines 46–141

The function takes a new incoming `Transaction` and either adds it to the mempool
(the waiting room for unconfirmed transactions) or rejects it. There are five distinct
sections.

---

## Section 1 — Basic input validity (lines 47–58)

```rust
let mut known_inputs = HashSet::new();
for input in &transaction.inputs {
    if !self.utxos.contains_key(&input.prev_transaction_output_hash) {
        return Err(BtcError::InvalidTransaction);
    }
    if known_inputs.contains(&input.prev_transaction_output_hash) {
        return Err(BtcError::InvalidTransaction);
    }
    known_inputs.insert(input.prev_transaction_output_hash);
}
```

**What it does:** Two checks per input, in one pass:

1. **The input must exist in the UTXO set.** Every transaction input references a
   previous output (a coin) by its hash. If that hash isn't in `self.utxos`, the
   coin doesn't exist — reject.

2. **No input can appear twice in the same transaction.** `known_inputs` is a
   `HashSet` that accumulates hashes as we go. If we see the same hash a second
   time, the transaction is trying to spend the same coin twice within itself — reject.

**Background — what is a UTXO?**
UTXO = Unspent Transaction Output. `self.utxos` is the map of all coins that currently
exist and haven't been spent yet. Every transaction input must point to one. The bool
field `(bool, TransactionOutput)` tracks whether the coin is already claimed by a
pending mempool transaction (true = claimed, false = free).

---

## Section 2 — Conflict resolution with existing mempool transactions (lines 64–96)

This is the most complex section. It handles the case where a UTXO is already marked
as claimed (`true`) by an existing mempool transaction — meaning two transactions are
trying to spend the same coin.

```rust
for input in &transaction.inputs {
    if let Some((true, _)) = self.utxos.get(&input.prev_transaction_output_hash) {
```

For each input, check if the UTXO's bool is `true` (already claimed by someone in
the mempool).

**If it is claimed (the `if` branch, lines 66–86):**

```rust
// Step 1 — find which mempool transaction is using this UTXO
let referencing_transaction =           // type: Option<(usize, &(Hash, Transaction))>
    self.mempool
        .iter()
        .enumerate()
        .find(|(_, (_, transaction))| {
            transaction.outputs.iter()
                .any(|output| output.hash() == input.prev_transaction_output_hash)
        });
```

Search the mempool to find which transaction produced this UTXO as one of its outputs.
`enumerate()` gives us the index `idx` so we can remove it later.

Note: the closure parameter `transaction` shadows the outer `transaction` parameter
(the new incoming one). Inside the closure, `transaction` refers to each mempool entry,
not the new one being added.

```rust
// Step 2 — evict that transaction and unmark its UTXOs
if let Some((idx, (_, referencing_transaction))) = referencing_transaction {
```

`referencing_transaction` is used twice here with different types (variable shadowing):
- First binding (line 66): `Option<(usize, &(Hash, Transaction))>` — the raw find result
- Second binding (line 76): `&Transaction` — destructured out of the Option

```rust
    for input in &referencing_transaction.inputs {
        self.utxos
            .entry(input.prev_transaction_output_hash)
            .and_modify(|(marked, _)| { *marked = false; });
    }
    self.mempool.remove(idx);
```

The existing mempool transaction is evicted. All UTXOs it was claiming are unmarked
(`false`), freeing them up. The new incoming transaction will claim them instead.

This implements a "last writer wins" or "replace by fee" style policy — a newer
transaction can bump an older one out of the mempool.

**If it is NOT claimed (the `else` branch, lines 87–95):**

```rust
} else {
    self.utxos
        .entry(input.prev_transaction_output_hash)
        .and_modify(|(marked, _)| { *marked = false; });
}
```

This branch handles a defensive edge case: the UTXO bool was `true` but no matching
mempool transaction was found. This shouldn't happen in normal operation (hence the
comment "if, somehow"). The bool is reset to `false` to clean up inconsistent state.

---

## Section 3 — Value check: inputs must cover outputs (lines 98–114)

```rust
let all_inputs = transaction.inputs.iter()
    .map(|input| {
        self.utxos
            .get(&input.prev_transaction_output_hash)
            .expect("BUG: impossible")
            .1       // the TransactionOutput (index 1 of the tuple)
            .value   // the u64 satoshi amount
    })
    .sum::<u64>();

let all_outputs = transaction.outputs.iter().map(|output| output.value).sum();

if all_inputs < all_outputs {
    return Err(BtcError::InvalidTransaction);
}
```

Look up the actual coin value of each input from the UTXO set and sum them. Then sum
the outputs. The inputs must be >= the outputs.

The difference (`all_inputs - all_outputs`) is the **miner fee** — it goes to whoever
mines the block. A transaction cannot create coins out of thin air.

`.expect("BUG: impossible")` — by this point we already verified every input exists
in the UTXO set (Section 1), so this `.get()` cannot return `None`. The `.expect` is
just a safety net in case of a logic bug in this code.

`.1` — the UTXO map value is `(bool, TransactionOutput)`. Index `.1` gets the
`TransactionOutput`. `.value` is the satoshi amount on that output.

---

## Section 4 — Mark UTXOs as claimed (lines 116–122)

```rust
for input in &transaction.inputs {
    self.utxos
        .entry(input.prev_transaction_output_hash)
        .and_modify(|(marked, _)| { *marked = true; });
}
```

Now that the transaction has passed all validation, mark every UTXO it spends as
claimed (`true`). This prevents any other mempool transaction from spending the same
coins while this one is pending.

This is the counterpart to Section 2 — where Section 2 may clear these flags for
an evicted transaction, Section 4 sets them for the newly accepted one.

---

## Section 5 — Push to mempool and sort by miner fee (lines 123–140)

```rust
self.mempool.push((Utc::now(), transaction));
self.mempool.sort_by_key(|(_, transaction)| {
    let all_inputs = transaction.inputs.iter()
        .map(|input| {
            self.utxos.get(&input.prev_transaction_output_hash)
                .expect("Bug impossible").1.value
        })
        .sum::<u64>();
    let all_outputs: u64 = transaction.outputs.iter().map(|output| output.value).sum();
    let miner_fee = all_inputs - all_outputs;
    miner_fee
});
Ok(())
```

The transaction is pushed onto the mempool `Vec` along with the current timestamp.
Then the entire mempool is re-sorted so that **higher fee transactions come last**
(ascending sort by miner fee).

When a miner builds a block they will take transactions from the end of this Vec
first — highest fee transactions get included first, which is how miners maximise
their reward.

The fee calculation is the same as Section 3: `inputs - outputs`. The difference goes
to the miner.

---

## Summary — the full decision path

```
new transaction arrives
        │
        ▼
[Section 1] Does every input exist in the UTXO set?
            Are all inputs unique within this transaction?
        │ no → reject
        │ yes ↓
[Section 2] Is any input already claimed by a mempool transaction?
            If yes → evict that transaction, unmark its UTXOs
        │
        ▼
[Section 3] Do inputs cover outputs? (no coins created from nothing)
        │ no → reject
        │ yes ↓
[Section 4] Mark all spent UTXOs as claimed
        │
        ▼
[Section 5] Add to mempool, re-sort by miner fee
```
