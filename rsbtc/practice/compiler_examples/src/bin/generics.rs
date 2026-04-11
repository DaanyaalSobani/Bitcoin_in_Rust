use std::fmt::Display;

// --- User defined trait ---

trait Summary {
    fn summarise(&self) -> String;
}

// Two unrelated types that both implement Summary
struct Block {
    height: u64,
    hash: String,
}

struct Transaction {
    amount: u64,
    from: String,
    to: String,
}

impl Summary for Block {
    fn summarise(&self) -> String {
        format!("Block #{} hash={}", self.height, self.hash)
    }
}

impl Summary for Transaction {
    fn summarise(&self) -> String {
        format!("{} -> {} : {} sats", self.from, self.to, self.amount)
    }
}

// --- Generic functions with trait bounds ---

// T must support > (PartialOrd) — works for u64, f64, char, String etc.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// T must support both > and printing with {}
// The where clause is an alternative syntax to T: PartialOrd + Display
// useful when bounds get long
fn print_largest<T>(list: &[T])
where
    T: PartialOrd + Display,
{
    let result = largest(list);
    println!("Largest: {}", result);
}

// T must implement our own Summary trait
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarise());
}

fn main() {
    // largest works on any type that supports comparison
    let numbers = vec![3u64, 7, 2, 9, 4];
    let words = vec!["bitcoin", "satoshi", "nakamoto"];

    println!("=== largest ===");
    println!("{}", largest(&numbers));   // 9
    println!("{}", largest(&words));     // satoshi (alphabetical)

    println!("\n=== print_largest ===");
    print_largest(&numbers);
    print_largest(&words);

    println!("\n=== user defined trait ===");
    let block = Block {
        height: 840_000,
        hash: String::from("000000000000000000029b"),
    };
    let tx = Transaction {
        amount: 5000,
        from: String::from("Alice"),
        to: String::from("Bob"),
    };

    print_summary(&block);
    print_summary(&tx);
}
