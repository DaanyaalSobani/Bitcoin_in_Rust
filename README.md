# Building Bitcoin in Rust — Notes & Code

Personal notes and working code from the textbook **[Building Bitcoin in Rust](https://braiins.com/books/building-bitcoin-in-rust)** by Braiins.
The companion source repository is at [braiins/build-bitcoin-in-rust](https://github.com/braiins/build-bitcoin-in-rust).

The goal is to learn Rust and Bitcoin internals at the same time by implementing a Bitcoin node from scratch.

## Structure

```
rsbtc/
  lib/                          shared library — Bitcoin types and logic
  miner/                        block miner binary
  node/                         P2P node binary
  wallet/                       wallet binary
  practice/
    hello_world/                CLI text transformer — early Rust exercise
    compiler_examples/          ownership, borrowing, iterators, generics
    oop_examples/               OOP in Rust — traits, dispatch, visibility
notes/                          reference notes per chapter
exercises/                      practice exercises with solutions
```

The `practice/` crates are learning exercises from the early chapters, not part of the Bitcoin implementation itself.

## Installation

**1. Install Rust toolchain** (gives you `rustc`, `cargo`, `rustup`):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**2. Clone the repo:**
```bash
git clone https://github.com/DaanyaalSobani/Bitcoin_in_Rust.git
cd Bitcoin_in_Rust
```

**3. Open in VS Code** — it will prompt to install the recommended extensions:
- `rust-analyzer` — language server (autocomplete, type hints, errors)
- `CodeLLDB` — debugger

## Cargo Cheatsheet

All commands marked with `📁 rsbtc/` must be run from the `rsbtc/` directory.
Commands marked with `📁 any` can be run from anywhere using `--manifest-path`.

### Building

```bash
# 📁 rsbtc/
cargo build                         # debug build (fast compile, slow run)
cargo build --release               # release build (slow compile, fast run)
cargo build --workspace             # build all crates at once

# 📁 any
cargo build --manifest-path rsbtc/Cargo.toml --workspace
```

### Running

```bash
# 📁 rsbtc/
cargo run -p hello_world -- reverse "Hello World"   # run a specific crate with args
cargo run -p miner                                   # run a crate with no args
cargo run --bin bench --release                      # run a specific binary in release mode
cargo run --bin generics                             # run a specific binary in debug mode

# 📁 any (from repo root)
cargo run --manifest-path rsbtc/practice/hello_world/Cargo.toml -- reverse "Hello World"
```

### Checking & Testing

```bash
# 📁 rsbtc/ (or inside any crate)
cargo check                         # type-check only — fastest feedback, no binary
cargo test                          # run tests in current crate
cargo test --workspace              # run all tests across all crates
```

### Dependencies

```bash
# 📁 inside the crate you want to add the dependency to
cargo add serde                     # add latest version of a crate
cargo add serde --features derive   # add with specific features
```

### Cleaning

```bash
# 📁 rsbtc/
cargo clean                         # delete target/ — frees disk space
```

### Debugging (VS Code)

- Set a breakpoint by clicking the gutter (left of line numbers)
- Press `F5` to launch — pick a configuration from the dropdown
- `F10` step over, `F11` step into, `F5` continue
- Arguments passed to the binary are configured in `.vscode/launch.json`

## Running

```bash
# 📁 rsbtc/
cargo run -p hello_world -- reverse "Hello World"
cargo run -p hello_world -- leet "Satoshi Nakamoto"
cargo run -p hello_world -- acronym "unspent transaction output"
cargo run --bin bench --release     # iterator vs loop benchmark
cargo run --bin generics            # generics and traits demo
python3 practice/compiler_examples/bench.py  # Python equivalent benchmark
```

## Progress

- [x] Rust basics — ownership, borrowing, references
- [x] Iterators and closures
- [x] Generics and traits
- [x] Hashing — SHA-256 (`sha256.rs`)
- [x] Cryptography — ECDSA keys and signatures (`crypto.rs`)
- [x] Serialisation — `Saveable` trait with CBOR via ciborium (`util.rs`)
- [x] Bitcoin data structures — `Transaction`, `Block`, `Blockchain` types with save/load binaries
- [ ] Mining
- [ ] P2P networking
- [ ] Wallet and key management
