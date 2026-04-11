# oop_examples

Demonstrates OOP concepts in Rust — visibility, structs, traits, and dispatch.

## Run

```bash
# from rsbtc/
cargo run -p oop_examples

# from anywhere
cargo run --manifest-path rsbtc/oop_examples/Cargo.toml
```

## Structure

```
src/
  main.rs         entry point — declares and calls each module
  visibility.rs   pub / private / pub(crate) examples
  structs.rs      struct + impl, private fields, associated functions
  traits.rs       trait definition, Duck / FormalDuck / Human
  dispatch.rs     static dispatch (generics) vs dynamic dispatch (dyn)
```
