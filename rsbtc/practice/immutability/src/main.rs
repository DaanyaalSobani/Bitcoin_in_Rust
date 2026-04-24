#![allow(warnings)]
fn main() {
    example_1_immutable_by_default();
    example_2_mut_keyword();
    example_3_mut_parameter();
    example_4_mut_param_equivalent();
    example_5_shadowing_with_mut();
    example_6_reference_mutability();
}

// ============================================================
// Example 1 — Immutability is the default
// Uncommenting the block below produces:
//   error[E0384]: cannot assign twice to immutable variable `x`
// ============================================================
fn example_1_immutable_by_default() {
    println!("--- 1. immutable by default ---");

    let x = 10;
    println!("x = {x}");

    // x = 5;  // COMPILE ERROR — uncomment to see:
    // error[E0384]: cannot assign twice to immutable variable `x`
    // run `rustc --explain E0384` for full explanation
}

// ============================================================
// Example 2 — mut keyword makes a binding mutable
// ============================================================
fn example_2_mut_keyword() {
    println!("--- 2. mut keyword ---");

    let mut x = 10;
    println!("x before = {x}");
    x = 5;
    println!("x after  = {x}");
}

// ============================================================
// Example 3 — mut on a function parameter
// The caller's value is unaffected — no out-parameters in Rust
// ============================================================
fn double_and_print(mut x: i32) {
    x *= 2;
    println!("inside function: {x}");
}

fn example_3_mut_parameter() {
    println!("--- 3. mut parameter ---");

    let val = 10;
    double_and_print(val);
    println!("outside function: {val}"); // unchanged — mut only affects the local binding
}

// ============================================================
// Example 4 — mut parameter is exactly equivalent to a local mut binding
// These two functions are identical in behaviour
// ============================================================
fn with_mut_param(mut x: i32) {
    x *= 2;
    println!("{x}");
}

fn with_local_mut(x: i32) {
    let mut other_x = x;
    other_x *= 2;
    println!("{other_x}");
}

fn example_4_mut_param_equivalent() {
    println!("--- 4. mut param vs local mut (identical) ---");
    with_mut_param(10);
    with_local_mut(10);
}

// ============================================================
// Example 5 — shadowing a binding with a mutable one
// The original immutable x is not changed — a new mutable x is created
// ============================================================
fn example_5_shadowing_with_mut() {
    println!("--- 5. shadowing with mut ---");

    let x = 5;
    println!("immutable x = {x}");

    let mut x = x; // shadow x with a mutable binding, initialised from the old x
    x += 1;
    println!("mutable x  = {x}");
}

// ============================================================
// Example 6 — mutability of a reference vs mutability of a binding
//
// These are two separate concepts:
//   mut binding  — whether you can point the variable at something else
//   &mut ref     — whether you can modify the data being pointed at
// ============================================================
fn example_6_reference_mutability() {
    println!("--- 6. mutable binding vs mutable reference ---");

    let mut x = String::from("Ahoj");
    let y = String::from("Moikka");

    // mut binding — string_ref itself can be reassigned to point elsewhere
    // the strings x and y are NOT modified
    let mut string_ref = &x;
    println!("string_ref points to: {string_ref}");
    string_ref = &y; // rebind — now points to y
    println!("string_ref now points to: {string_ref}");

    // &mut reference — can modify the data x points to
    // but string_mut itself cannot be rebound (no mut on the binding)
    let string_mut = &mut x;
    string_mut.push_str(" and Hello"); // modifies x's data in place

    // cannot use y or x directly here while string_mut is alive
    println!("x after mutation: {x}");
    println!("y unchanged:      {y}");
}
