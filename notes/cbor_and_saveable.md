# CBOR, ciborium, and the Saveable trait

Reference: `lib/src/types/transaction.rs` — `impl Saveable for Transaction`
Reference: `lib/src/util.rs` — `Saveable` trait
Reference: `lib/src/bin/tx_gen.rs`, `lib/src/bin/tx_print.rs`

---

## Is the file binary?

Yes. When `tx_gen` saves a transaction to `tx.cbor`, the file on disk contains raw
binary bytes — not human-readable text. If you opened it in a text editor you'd see
garbage characters.

Compare the same data in three formats:

```
JSON  (text):    {"value":5000000000,"unique_id":"abc-123",...}
CBOR  (binary):  a2 65 76 61 6c 75 65 1a 12 a0 5f 00 ...
```

Both represent the same `Transaction` struct. CBOR is more compact and faster to
read/write — important when a Bitcoin node processes thousands of transactions.

---

## What is serialisation?

Your `Transaction` struct lives in memory while the program runs:

```
RAM:
┌─────────────────────────────┐
│ Transaction                 │
│   inputs: Vec [...]         │
│   outputs: Vec [            │
│     TransactionOutput {     │
│       value: 5000000000     │
│       unique_id: Uuid       │
│       pubkey: PublicKey     │
│     }                       │
│   ]                         │
└─────────────────────────────┘
```

When the program exits, this is gone. **Serialisation** converts it into a sequence of
bytes that can be written to disk or sent over a network and reconstructed later.
**Deserialisation** is the reverse — bytes back into a struct.

```
Transaction struct  ──serialise──►  bytes on disk
bytes on disk       ──deserialise──► Transaction struct
```

---

## What ciborium does

`ciborium` is a Rust crate that implements CBOR serialisation and deserialisation.
It works with `serde` — which is why your struct has `#[derive(Serialize, Deserialize)]`.
`serde` generates code describing the shape of your struct; ciborium uses that
description to encode/decode it as CBOR bytes.

**Saving** — `ciborium::ser::into_writer`:

```rust
fn save<O: Write>(&self, writer: O) -> IoResult<()> {
    ciborium::ser::into_writer(self, writer)
        .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to serialize Transaction"))
}
```

- `self` — the `Transaction` to serialise
- `writer` — anything that implements `Write` (a `File`, a `Vec<u8>`, a socket...)
- ciborium walks the struct field by field (via serde) and writes CBOR bytes into `writer`
- returns `Ok(())` on success, or the mapped `IoError` on failure

**Loading** — `ciborium::de::from_reader`:

```rust
fn load<I: Read>(reader: I) -> IoResult<Self> {
    ciborium::de::from_reader(reader).map_err(|_| {
        IoError::new(IoErrorKind::InvalidData, "Failed to deserialize Transaction")
    })
}
```

- `reader` — anything that implements `Read` (a `File`, a byte slice...)
- ciborium reads bytes from `reader` and reconstructs a `Transaction` struct
- returns `Ok(Transaction)` on success, or the mapped `IoError` on failure

---

## Why `.map_err`?

`ciborium` returns its own error type, not `std::io::Error`. But `Saveable` requires
`IoResult<T>` which is `Result<T, std::io::Error>`. `.map_err` converts the ciborium
error into an `IoError` so the types line up:

```rust
ciborium_result           // Result<T, ciborium::Error>
    .map_err(|_| IoError::new(...))  // Result<T, std::io::Error>
```

The `|_|` ignores the original ciborium error detail — a more thorough implementation
might include it in the message.

---

## The full flow: tx_gen → file → tx_print

```
tx_gen
  │
  ├── creates Transaction struct in memory
  ├── calls transaction.save_to_file("tx.cbor")
  │         │
  │         ├── Saveable::save_to_file opens File
  │         └── calls Transaction::save(file)
  │                   │
  │                   └── ciborium::ser::into_writer
  │                             │
  │                             └── writes binary CBOR bytes
  │                                         │
  │                                    [ tx.cbor ]  ← binary file on disk
  │                                         │
tx_print                                    │
  │                                         │
  ├── opens File                            │
  ├── calls Transaction::load(file) ────────┘
  │         │
  │         └── ciborium::de::from_reader
  │                   │
  │                   └── reads bytes, reconstructs Transaction struct
  │
  └── println!("{:#?}", tx)  ← prints the struct
```

The two binaries never communicate directly — the file is the only connection.
`tx_gen` and `tx_print` don't know about CBOR at all; they just call `save_to_file`
and `load`. The `Saveable` implementation on `Transaction` is the only place that
touches ciborium.

---

## Why not just use JSON?

You could — swap ciborium for `serde_json` and the pattern is identical. CBOR is
chosen here because:

- **Smaller files** — binary is more compact than text (no quotes, brackets, field
  name strings repeated for every record)
- **Faster** — no text parsing
- **Bitcoin-appropriate** — Bitcoin's own wire format is binary; using CBOR for
  storage is consistent with that

The `.cbor` extension is just a convention to signal the format — the OS treats it
like any other file.
