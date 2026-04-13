# Converting bytes to U256 — `from_big_endian`

Reference: `lib/src/sha256.rs` line 24

---

## The error

```
error[E0277]: the trait bound `U256: From<[u8; 32]>` is not satisfied
    Hash(U256::from(hash_array))
```

`U256::from(hash_array)` requires `U256` to implement `From<[u8; 32]>`. It doesn't.
The `uint` crate's `From` implementations only cover primitive integer types (`u64`, `u128`,
etc.) — not byte arrays.

---

## The fix

```rust
// before — doesn't compile
Hash(U256::from(hash_array))

// after — works
Hash(U256::from_big_endian(&hash_array))
```

`U256` provides two explicit byte conversion methods instead of `From`:

```rust
// uint-0.10.0/src/uint.rs, line 1255
pub fn from_big_endian(slice: &[u8]) -> Self { ... }
pub fn from_little_endian(slice: &[u8]) -> Self { ... }
```

Both take `&[u8]` (a byte slice). Passing `&hash_array` coerces `[u8; 32]` → `&[u8]`
automatically — the same coercion you saw with `&[T]` in `util.rs`.

---

## Why two methods? What is endianness?

When you store a multi-byte integer in memory, you have a choice of which byte comes first.

**Big-endian** — most significant byte first (like reading a number left to right):
```
the number 0x0102  stored as:  01 02
```

**Little-endian** — least significant byte first:
```
the number 0x0102  stored as:  02 01
```

This only matters for integers larger than one byte. For raw byte sequences (like strings
or hashes treated as blobs) endianness doesn't apply — but once you interpret bytes *as a
number*, you have to choose.

---

## Why `from_big_endian` specifically?

Bitcoin defines hash values as big-endian 256-bit integers. SHA-256 outputs bytes in
big-endian order — the first byte is the most significant. So `from_big_endian` is not
just a workaround, it is the correct interpretation.

Using `from_little_endian` here would produce a numerically different `U256` from the same
hash bytes — which would break proof-of-work comparison in `matches_target`.

---

## Converting U256 back to bytes — `to_little_endian`

Reference: `lib/src/sha256.rs` — `Hash::as_bytes()`

The reverse operation also changed between textbook and `uint` 0.10.

**Textbook version (old API) — buffer out-parameter:**
```rust
pub fn as_bytes(&self) -> [u8; 32] {
    let mut bytes: Vec<u8> = vec![0; 32];  // allocate buffer manually
    self.0.to_little_endian(&mut bytes);    // write into it via &mut
    bytes.as_slice().try_into().unwrap()    // convert slice → [u8; 32]
}
```
The old `to_little_endian` took a `&mut [u8]` buffer and wrote into it — the same
out-parameter via `&mut` pattern seen in `ciborium::into_writer`.

**`uint` 0.10 version — returns the array directly:**

Source: `~/.cargo/registry/.../uint-0.10.0/src/uint.rs`, line 766:
```rust
pub fn to_little_endian(&self) -> [u8; $n_words * 8] {
    let mut bytes = [0u8; $n_words * 8];
    self.write_as_little_endian(&mut bytes);
    bytes
}
```
`$n_words * 8 = 4 * 8 = 32` for `U256` — so it always returns `[u8; 32]`.
The buffer allocation still happens, just inside the method now instead of in your code.

**Fixed version:**
```rust
pub fn as_bytes(&self) -> [u8; 32] {
    self.0.to_little_endian()  // returns [u8; 32] directly
}
```

The behaviour is identical — it's a pure API ergonomics change between versions.
The old API also had `write_as_little_endian(&mut [u8])` if you wanted the buffer style,
and `uint` 0.10 still has `write_as_little_endian` for that (line 773).

---

## Why didn't `U256` just implement `From<[u8; 32]>`?

Because `From<[u8; 32]>` would be ambiguous — it would have to pick an endianness
silently. By requiring you to call `from_big_endian` or `from_little_endian` explicitly,
the `uint` crate forces you to be intentional. This is a common pattern in low-level
numeric libraries where endianness bugs are easy to introduce and hard to debug.
