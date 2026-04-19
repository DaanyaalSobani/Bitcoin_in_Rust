use btclib::types::Transaction;
use btclib::util::Saveable;
use std::fs::File;
use std::process::exit;
use std::{env, path};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage tx_print <tx_file>");
        exit(1);
    });
    if let Ok(file) = File::open(path) {
        let tx = Transaction::load(file).expect("Failed to Load");
        println!("{:#?}", tx);
    }
}
