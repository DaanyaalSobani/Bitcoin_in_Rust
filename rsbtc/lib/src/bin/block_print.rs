use btclib::types::Block;
use btclib::util::Saveable;
use std::env;
use std::fs::File;
use std::process::exit;
fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("\x1b[31mUsage: block_print <block_file>\x1b[0m");
        exit(1);
    });
    if let Ok(file) = File::open(path) {
        let block = Block::load(file).expect("Failed to load block");
        println!("{:#?}", block);
    }
}
