#![allow(warnings)]
fn main() {
    example_1_basic_factory();
    example_2_factory_with_mutable_capture();
    example_3_mempool_simulation();
    example_4_multiple_closures_same_factory();
}

// ============================================================
// Example 1 — basic closure factory
//
// A function that takes a threshold and returns a closure
// that checks whether a number exceeds it.
// The closure owns `threshold` via `move`.
// ============================================================
fn example_1_basic_factory() {
    println!("--- 1. basic closure factory ---");

    fn make_threshold_checker(threshold: u64) -> impl Fn(u64) -> bool {
        move |value| value >= threshold
        //    ^^^^^ threshold is moved into the closure
        //          the closure owns its own copy — no borrow of the outer frame
    }

    let is_large = make_threshold_checker(10_000);
    let is_tiny  = make_threshold_checker(10);

    println!("5000 is_large: {}", is_large(5_000));   // false
    println!("5000 is_tiny:  {}", is_tiny(5_000));    // true
    println!("50   is_large: {}", is_large(50));      // false
    println!("50   is_tiny:  {}", is_tiny(50));       // true

    // Both closures are independent — each owns its own copy of threshold.
    // Calling one does not affect the other.
}

// ============================================================
// Example 2 — factory that captures a mutable reference
//
// The returned closure holds a &mut Vec and appends to it
// each time it's called. The '_ lifetime tells Rust: the
// returned closure must not outlive the Vec it borrows.
// ============================================================
fn example_2_factory_with_mutable_capture() {
    println!("--- 2. factory capturing &mut ---");

    fn make_collector<'a>(prefix: &'static str, log: &'a mut Vec<String>) -> impl FnMut(u64) -> bool + 'a {
        move |value| {
            if value >= 1_000 {
                log.push(format!("{prefix}: keeping {value}"));
                true
            } else {
                log.push(format!("{prefix}: dropping {value}"));
                false
            }
        }
    }

    let mut log: Vec<String> = vec![];
    let mut checker = make_collector("filter", &mut log);

    let values = vec![500u64, 2_000, 100, 5_000, 300];
    let kept: Vec<u64> = values.into_iter().filter(|v| checker(*v)).collect();

    // checker is dropped here — log borrow is released
    drop(checker);

    println!("kept: {:?}", kept);
    for entry in &log {
        println!("  {entry}");
    }
}

// ============================================================
// Example 3 — mempool simulation (mirrors cleanup_mempool)
//
// make_keep_checker returns a closure suitable for Vec::retain.
// The closure:
//   - owns `max_age` via move
//   - holds &mut expired_ids to record what was removed
//
// This is the pattern from Blockchain::cleanup_mempool in
// lib/src/types.rs — a function returning a closure so that
// retain's single-argument signature still lets us collect
// side-effect data.
// ============================================================
fn example_3_mempool_simulation() {
    println!("--- 3. mempool cleanup simulation ---");

    // Simulated transaction: (age_in_seconds, tx_id)
    let mut mempool: Vec<(u64, u32)> = vec![
        (10,  101),
        (120, 102),   // too old
        (30,  103),
        (200, 104),   // too old
        (5,   105),
    ];
    println!("mempool before: {:?}", mempool);

    let mut expired_ids: Vec<u32> = vec![];

    fn make_keep_checker(max_age: u64, expired: &mut Vec<u32>) -> impl FnMut(&(u64, u32)) -> bool + '_ {
        move |(age, id)| {
            if *age > max_age {
                expired.push(*id);
                false   // remove from mempool
            } else {
                true    // keep
            }
        }
    }

    mempool.retain(make_keep_checker(60, &mut expired_ids));

    println!("mempool after:  {:?}", mempool);
    println!("expired tx ids: {:?}", expired_ids);
}

// ============================================================
// Example 4 — two independent closures from the same factory
//
// Each call to the factory produces a separate closure that
// owns its own captured state. They don't share anything.
// ============================================================
fn example_4_multiple_closures_same_factory() {
    println!("--- 4. two independent closures from one factory ---");

    fn make_multiplier(factor: u64) -> impl Fn(u64) -> u64 {
        move |x| x * factor
    }

    let double = make_multiplier(2);
    let triple = make_multiplier(3);

    let numbers = vec![1u64, 2, 3, 4, 5];

    let doubled: Vec<u64> = numbers.iter().map(|x| double(*x)).collect();
    let tripled: Vec<u64> = numbers.iter().map(|x| triple(*x)).collect();

    println!("original: {:?}", numbers);
    println!("doubled:  {:?}", doubled);
    println!("tripled:  {:?}", tripled);
}
