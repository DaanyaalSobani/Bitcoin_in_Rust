use btclib::types::Block;
use btclib::util::Saveable;
use std::process::exit;
use std::{env, usize};
fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we have the right amount of args (program name + 2 args)
    if args.len() < 3 {
        eprintln!("Usage: {} <block_file> <steps>", args[0]);
        exit(1);
    }

    let path = &args[1];

    let steps: usize = args[2].parse().ok().filter(|&s| s > 0).unwrap_or_else(|| {
        eprintln!("<steps> should be a positive integer");
        exit(1)
    });
    println!("Mining {} for {} steps...", path, steps);

    let og_block = Block::load_from_file(path).expect("Failed to load block");
    let mut block = og_block.clone();

    while !block.header.mine(steps) {
        println!("mining...");
    }
    // print original block and its hash
    println!("original: {:#?}", og_block);
    println!("hash: {}", og_block.header.hash());
    // print mined block and its hash
    println!("final: {:#?}", block);
    println!("hash: {}", block.header.hash());
}
