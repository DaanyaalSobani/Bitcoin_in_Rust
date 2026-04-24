#![allow(warnings)]
use std::hint::black_box;
use std::time::Instant;

const SIZE: usize = 100_000_000;
const RUNS: u32 = 100;

// Iterator chain: filter evens, multiply by 3, sum — single pass, no intermediate Vec
fn with_iterator(data: &[u64]) -> u64 {
    data.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 3).sum()
}

// For loop doing the exact same work
fn with_loop(data: &[u64]) -> u64 {
    let mut sum = 0u64;
    for &x in data {
        if x % 2 == 0 {
            sum += x * 3;
        }
    }
    sum
}

fn time_it(label: &str, f: impl Fn() -> u64) {
    // warmup run so caches are warm before we start timing
    black_box(f());

    let start = Instant::now();
    for _ in 0..RUNS {
        // black_box prevents the compiler from optimising the call away
        // entirely because it sees the result is "used"
        black_box(f());
    }
    let avg = start.elapsed() / RUNS;
    println!("{label}: {:>10?} avg over {RUNS} runs", avg);
}

fn main() {
    let data: Vec<u64> = (0..SIZE as u64).collect();

    println!("Dataset : {} elements", SIZE);
    println!("Operation: filter evens → multiply by 3 → sum\n");

    time_it("Iterator chain", || with_iterator(black_box(&data)));
    time_it("For loop      ", || with_loop(black_box(&data)));

    // sanity check — both should produce the same result
    assert_eq!(with_iterator(&data), with_loop(&data));
    println!("\nResults match ✓");
}
