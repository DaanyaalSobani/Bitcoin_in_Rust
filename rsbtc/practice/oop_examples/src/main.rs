#![allow(warnings)]
mod dispatch;
mod practice;
mod structs;
mod traits;
mod visibility;

use crate::practice::{
    Describable, Printable, print_all, print_description, print_dynamic, print_static,
};

fn main() {
    let mut w = practice::Wallet::new("Daanyaal");
    w.deposit(50);
    w.withdraw(25);

    let t = practice::Transaction::new("Alice", "Bob", 500);

    // Exercise 2 — print_description with both types
    println!("=== describe ===");
    print_description(&w);
    print_description(&t);

    // Exercise 3 — static and dynamic dispatch
    println!("\n=== static dispatch ===");
    print_static(&w);
    print_static(&t);

    println!("\n=== dynamic dispatch ===");
    print_dynamic(&w);
    print_dynamic(&t);

    println!("\n=== heterogeneous collection ===");
    let items: Vec<Box<dyn Describable>> = vec![
        Box::new(practice::Wallet::new("Bob")),
        Box::new(practice::Transaction::new("Carol", "Dave", 100)),
        Box::new(practice::Wallet::new("Eve")),
    ];
    print_all(&items);

    // Exercise 4 — default vs overridden summarise
    println!("\n=== summarise ===");
    println!("{}", w.summarise()); // uses default
    println!("{}", t.summarise()); // uses Transaction's override

    // Exercise 5 — visibility module
    println!("\n=== bitcoin module ===");
    let mut block = practice::bitcoin::Block::new(840_000);
    block.mine();
    block.mine();
    block.mine();
    println!("height: {}", block.height());
    // block.nonce  // COMPILE ERROR — nonce is private

    // Stretch — supertraits
    println!("\n=== printable supertrait ===");
    w.print();
    t.print();
}
