#![allow(warnings)]
use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    println!("{}", args[0]);
    println!("{}", env::current_dir().unwrap().display());
    if args.len() < 3 {
        eprintln!("Usage {} <op> <text>", args[0]);
        exit(1);
    }
    println!("hello");
}
