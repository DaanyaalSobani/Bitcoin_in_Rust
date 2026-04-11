# Building Bitcoin in Rust — Notes & Code

Personal notes and working code from the textbook **"Building Bitcoin in Rust"**.

The goal is to learn Rust and Bitcoin internals at the same time by implementing a Bitcoin node from scratch.

## Structure

```
rsbtc/
  lib/                  shared library — Bitcoin types and logic
  miner/                block miner binary
  node/                 P2P node binary
  wallet/               wallet binary
  hello_world/          early Rust exercise — CLI text transformer
  compiler_examples/    Rust language experiments — ownership, borrowing, generics
```

`hello_world` and `compiler_examples` are learning exercises from the early chapters, not part of the Bitcoin implementation itself.

## Running

Each crate can be run individually:

```bash
cargo run -p hello_world -- reverse "Hello World"
cargo run -p compiler_examples
cargo run --bin bench --release   # benchmarking exercise
cargo run --bin generics          # generics and traits exercise
```

Or build everything at once from the `rsbtc/` folder:

```bash
cd rsbtc
cargo build --workspace
```

## Progress

- [x] Rust basics — ownership, borrowing, references
- [x] Iterators and closures
- [x] Generics and traits
- [ ] Bitcoin data structures (Block, Transaction, UTXO)
- [ ] Hashing (SHA-256, RIPEMD-160)
- [ ] Mining
- [ ] P2P networking
- [ ] Wallet and key management
