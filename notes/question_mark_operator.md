# The `?` Operator

---

## What it does

`?` is shorthand for "if this is an error, return it immediately; otherwise unwrap the value and continue."

```rust
// with ?
let value = some_fallible_call()?;

// what ? expands to
let value = match some_fallible_call() {
    Ok(v)  => v,
    Err(e) => return Err(e),
};
```

It only works inside functions that return `Result` or `Option`.

---

## On `Result<T, E>`

```rust
fn add_block(&mut self, block: Block) -> Result<()> {
    block.verify_transactions(self.block_height(), &self.utxos)?;
    //                                                          ^
    // if Err(e) → return Err(e) from add_block immediately
    // if Ok(()) → continue to next line

    self.blocks.push(block);
    Ok(())
}
```

The block only gets pushed if `verify_transactions` succeeds. Any error propagates up
to the caller of `add_block` automatically.

---

## On `Option<T>`

`?` also works on `Option` — `None` becomes an early return of `None`:

```rust
fn first_char(s: &str) -> Option<char> {
    let c = s.chars().next()?;  // returns None if string is empty
    Some(c)
}
```

---

## Errors are values, not exceptions

Rust has no exceptions. There is no `throw` or `raise`. **Errors are just return values.**

A function that can fail must say so in its return type:
```rust
fn process() -> Result<String>  // contract: "I might fail, caller must deal with it"
fn process() -> String          // contract: "I always succeed"
```

`return Err(e)` is an ordinary return — it returns a value of type `Result` down the
call stack to the caller. The caller receives it and decides what to do.

`panic!()` is completely different — it unwinds the stack and crashes the program.
There is no catching it (in normal code). It is for bugs, not expected errors.

| | `return Err(e)` | `panic!()` |
|---|---|---|
| What it is | a normal return value | program crash |
| Caller can handle it | yes — it's just a `Result` | no |
| Use for | expected failures (invalid input, file not found) | bugs that should never happen |
| Equivalent in other languages | `return error` in Go, `Err` in Haskell | `throw` / `raise` / `assert` |

**Compare to Python/Java:**
```python
def process():
    raise ValueError("oops")  # bypasses call stack invisibly
                               # no indication in the signature this can happen
```
The caller has no idea `process` can fail unless they read the docs. In Rust, `-> Result`
in the signature is a compiler-enforced contract. If you call a function that returns
`Result` and ignore it, the compiler warns you.

**The call stack with `?`:**
```
main()             ← actually handles the error (match, log, exit gracefully)
  └─ add_block()   ← propagates with ?  (doesn't handle, just passes up)
       └─ verify_transactions()  ← produces the Err value
```

`process` returning `Err(e)` is the same as any other return — it just hands a value
back. Only the top of the call stack needs to actually decide what to do with it.

---

## Chaining

`?` is most powerful when chaining multiple fallible operations:

```rust
// without ? — both versions do the SAME thing: propagate the error up
// neither handles it, they just pass it to whoever called process()
fn process() -> Result<String> {
    let file = match open_file("data.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),  // return the error as a value — not a throw
    };
    let contents = match read_to_string(file) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let parsed = match parse(contents) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    Ok(parsed)
}

// with ? — identical behaviour, less noise
fn process() -> Result<String> {
    let file     = open_file("data.txt")?;
    let contents = read_to_string(file)?;
    let parsed   = parse(contents)?;
    Ok(parsed)
}
```

The actual handling happens further up:
```rust
fn main() {
    match process() {       // HERE is where the error is actually handled
        Ok(s)  => println!("{s}"),
        Err(e) => println!("failed: {e}"),
    }
}
```

---

## Error type conversion

`?` also calls `From` to convert the error type if needed. If `verify_transactions`
returns `Err(TransactionError)` but `add_block` returns `Result<(), BtcError>`, Rust
will automatically call `BtcError::from(transaction_error)` — as long as that `From`
impl exists. This lets different layers use different error types without manual conversion.

---

## `?` vs `unwrap()`

| | `?` | `.unwrap()` |
|---|---|---|
| On error | propagates to caller | panics |
| Use when | caller should handle the error | error is impossible / programming bug |
| Return type required | yes — function must return `Result` or `Option` | no |

The `unwrap()` on `self.blocks.last()` in `add_block` is safe because we already
returned early if the chain was empty — the compiler can't prove this but we know it.
`?` can't be used there because `last()` returns `Option`, not `Result`, and the
function is designed to never reach that line with an empty chain.

---

## Where you've seen it in this codebase

| Location | What it does |
|---|---|
| `lib/src/types.rs` — `add_block` | propagates transaction validation errors |
| `lib/src/sha256.rs` — `hash()` | propagates CBOR serialization errors (via `if let Err`) |
| `lib/src/crypto.rs` — `signkey_serde::deserialize` | propagates byte deserialization errors |
