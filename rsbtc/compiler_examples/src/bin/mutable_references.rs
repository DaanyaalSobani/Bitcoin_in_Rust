// fn main() {
//     let mut bitcoin = String::from("bitcoin");
//     // Rust is actually pretty smart,
//     // so if it sees you are not using mut_ref
//     // after you have created ro_ref, it will
//     // destroy it early, this is a relatively
//     // recent change for ergonomics in Rust
//     // called Non-Lexical Lifetimes
//     let mut_ref = &mut bitcoin;
//     // ↑ borrow bitcoin mutably
//     // mut_ref is of type `&mut String`,
//     // given that the variable itself is immutable,
//     // this corresponds to `char* const ptr` in C
//     let ro_ref = &bitcoin;
//     // ↑ borrow bitcoin immutably
//     // this is what makes this example not compile
//     // as bitcoin is already borrowed mutably
//     println!("{}", ro_ref);
//     // ↑ use the immutable borrow
//     mut_ref.push_str(", the cryptocurrency");
//     // ↑ use the mutable borrow
// }
fn main() {
    'bitcoin_lifetime: {
        let mut bitcoin = String::from("bitcoin");
        'mut_ref_lifetime: {
            let mut_ref = &mut bitcoin;
            // ↑ borrow bitcoin mutably
            'ro_ref_lifetime: {
                let ro_ref = &bitcoin;
                // ↑ borrow bitcoin immutably
            }
            println!("{}", ro_ref); // <- use the immutable borrow
            mut_ref.push_str(", the cryptocurrency");
            // ↑ use the mutable borrow
        } // <- ro_ref goes out of scope here  ┐
        //                                   ├ these refs can't coexist,
    } // <- mut_ref goes out of scope here ┘ hence the issue
} // <- bitcoin goes out of scope her
