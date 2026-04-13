fn main() {
    example_1_scalars_even();
    example_2_scalars_dedup();
    example_3_strings();
    example_4_tuples_filter_by_field();
    example_5_tuples_destructure_ignore();
    example_6_mempool_simulation();
    example_7_global_allocator_explicit();
    example_8_arena_allocator();
    example_9_arena_many_short_lived();
}

// ============================================================
// Example 1 — retain on a Vec of scalars
// Keep only even numbers
// ============================================================
fn example_1_scalars_even() {
    println!("--- 1. keep only even numbers ---");

    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("before: {:?}", numbers);

    numbers.retain(|x| x % 2 == 0);
    println!("after:  {:?}", numbers);
    // [2, 4, 6, 8, 10]
}

// ============================================================
// Example 2 — retain to deduplicate (keep first occurrence)
// retain can simulate a dedup when combined with a seen set
// ============================================================
fn example_2_scalars_dedup() {
    println!("--- 2. deduplicate keeping first occurrence ---");

    let mut words = vec!["bitcoin", "rust", "bitcoin", "node", "rust", "wallet"];
    println!("before: {:?}", words);

    let mut seen = std::collections::HashSet::new();
    words.retain(|w| seen.insert(*w)); // insert returns false if already present
    println!("after:  {:?}", words);
    // ["bitcoin", "rust", "node", "wallet"]
}

// ============================================================
// Example 3 — retain on a Vec of Strings
// Remove strings that start with a capital letter
// ============================================================
fn example_3_strings() {
    println!("--- 3. keep only lowercase-starting strings ---");

    let mut names = vec![
        String::from("alice"),
        String::from("Bob"),
        String::from("carol"),
        String::from("Dave"),
        String::from("eve"),
    ];
    println!("before: {:?}", names);

    names.retain(|s| s.chars().next().map(|c| c.is_lowercase()).unwrap_or(false));
    println!("after:  {:?}", names);
    // ["alice", "carol", "eve"]
}

// ============================================================
// Example 4 — retain on a Vec of tuples, filter by one field
// Keep only transactions above a threshold amount
// ============================================================
fn example_4_tuples_filter_by_field() {
    println!("--- 4. keep transactions above threshold ---");

    let mut transactions: Vec<(String, u64)> = vec![
        (String::from("alice -> bob"),    5_000),
        (String::from("bob -> carol"),   50_000),
        (String::from("carol -> dave"),   1_000),
        (String::from("dave -> eve"),   100_000),
        (String::from("eve -> alice"),      500),
    ];
    println!("before: {:?}", transactions);

    let threshold = 10_000u64;
    transactions.retain(|(_, amount)| *amount >= threshold);
    //                    ^^^^^^^^^^ destructure — ignore label, bind amount
    //                               * dereferences the &u64 to compare

    println!("after:  {:?}", transactions);
    // [("bob -> carol", 50000), ("dave -> eve", 100000)]
}

// ============================================================
// Example 5 — retain with tuple destructuring, use both fields
// Keep only confirmed transactions (second field = true)
// and print what was removed
// ============================================================
fn example_5_tuples_destructure_ignore() {
    println!("--- 5. keep only confirmed transactions ---");

    let mut mempool: Vec<(u64, &str, bool)> = vec![
        (1, "alice -> bob",   true),
        (2, "bob -> carol",   false),
        (3, "carol -> dave",  true),
        (4, "dave -> eve",    false),
        (5, "eve -> alice",   true),
    ];
    println!("before: {:?}", mempool);

    mempool.retain(|(id, _, confirmed)| {
        if !confirmed {
            println!("  removing unconfirmed tx #{id}");
        }
        *confirmed
    });

    println!("after:  {:?}", mempool);
}

// ============================================================
// Example 6 — mempool simulation (mirrors lib/src/types.rs)
// After a block is mined, remove its transactions from the mempool
// ============================================================
fn example_6_mempool_simulation() {
    println!("--- 6. mempool: remove mined transactions ---");

    // Simulated mempool: (tx_id, description)
    let mut mempool: Vec<(u64, &str)> = vec![
        (101, "alice -> bob 1 BTC"),
        (102, "bob -> carol 0.5 BTC"),
        (103, "carol -> dave 2 BTC"),
        (104, "dave -> eve 0.1 BTC"),
        (105, "eve -> alice 3 BTC"),
    ];
    println!("mempool before block: {:?}", mempool);

    // Simulated block: contains tx ids 102 and 104
    let mined_tx_ids: Vec<u64> = vec![102, 104];
    println!("block includes tx ids: {:?}", mined_tx_ids);

    // Remove mined transactions — mirrors the retain in Blockchain::add_block
    mempool.retain(|(id, _)| !mined_tx_ids.contains(id));

    println!("mempool after block:  {:?}", mempool);
    // [(101, ...), (103, ...), (105, ...)]
}

// ============================================================
// Example 7 — the Global allocator (conceptual)
//
// Vec<T, Global> is the full type of every normal Vec<T>.
// Global = the system allocator (malloc/free under the hood).
// You never write it explicitly because it's the default.
//
// Writing Vec<T, Global> explicitly requires nightly Rust
// (#![feature(allocator_api)]) so this example just shows
// that a normal Vec is already using it invisibly:
//
//   let v: Vec<u64>         = vec![1,2,3];  // Vec<u64, Global>
//   let v: Vec<u64, Global> = ...;          // same — nightly only
//
// The A: Allocator bound you see in the retain signature is how
// the standard library makes retain work for BOTH Vec<T, Global>
// and Vec<T, &Bump> (or any other allocator) without duplicating code.
// ============================================================
fn example_7_global_allocator_explicit() {
    println!("--- 7. Global allocator (every normal Vec uses this) ---");

    // This is Vec<u64, Global> — the allocator is just implicit
    let mut v: Vec<u64> = vec![1, 2, 3, 4, 5];
    v.retain(|x| x % 2 != 0);
    println!("retained odds: {:?}", v);
    println!("  (backed by Global = system malloc/free, A param is hidden)");
}

// ============================================================
// Example 8 — arena (bump) allocator with bumpalo
//
// An arena allocator hands out memory from a single large block.
// Allocations are O(1) — just bump a pointer forward.
// The entire arena is freed at once when the Bump is dropped —
// no individual frees, no malloc overhead per element.
//
// Use case: many short-lived allocations that all die together,
// e.g. per-request data in a server, per-block data in a miner.
// ============================================================
fn example_8_arena_allocator() {
    println!("--- 8. arena allocator (bumpalo) ---");

    let bump = bumpalo::Bump::new(); // one big memory slab

    // Vec backed by the arena instead of Global
    let mut transactions = bumpalo::collections::Vec::new_in(&bump);
    transactions.push(("alice -> bob",   50_000u64));
    transactions.push(("bob -> carol",    1_000u64));
    transactions.push(("carol -> dave", 200_000u64));
    transactions.push(("dave -> eve",       500u64));

    println!("before: {:?}", transactions);

    transactions.retain(|(_, amount)| *amount >= 10_000);

    println!("after:  {:?}", transactions);
    // [("alice -> bob", 50000), ("carol -> dave", 200000)]

    // when `bump` is dropped here, ALL memory is freed in one shot —
    // no per-element free calls
}

// ============================================================
// Example 9 — why arenas matter: many short-lived vecs
//
// With Global: each Vec does N malloc + N free calls
// With Bump:   all allocations are O(1) pointer bumps,
//              one free when the Bump drops
// ============================================================
fn example_9_arena_many_short_lived() {
    println!("--- 9. arena: many short-lived Vecs, one free ---");

    let bump = bumpalo::Bump::new();

    // imagine these are transactions from 5 different blocks
    let block_data = vec![
        vec![1u64, 2, 3],
        vec![4, 5],
        vec![6, 7, 8, 9],
        vec![10],
        vec![11, 12, 13, 14, 15],
    ];

    let mut total_kept = 0;
    for txs in &block_data {
        // allocate in the arena — no malloc per Vec
        let mut v = bumpalo::collections::Vec::new_in(&bump);
        v.extend(txs.iter().copied());
        v.retain(|x| x % 2 == 0); // keep even tx ids
        total_kept += v.len();
        // v goes out of scope but memory stays in the bump slab
    }

    println!("total even tx ids across all blocks: {}", total_kept);
    // bump drops here — entire slab freed in one call
    println!("bump freed — all arena memory released at once");
}
