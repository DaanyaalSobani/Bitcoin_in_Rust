#![allow(warnings)]
// // in serious Rust, you'd use &str for flexibility,
// // as &String can convert to it automatically
// fn print_my_string(string: &String) {
//    // compare to `const char * const string`,
//    // which would be the C equivalent
//     println!("{}", string);
//    // the reference to string is destroyed here
// }
// // the print_my_string() function does not take the ownership
// // of the string, so you can pass it multiple times; for references
// // rust creates copies if necessary

// fn give_me_a_ref<'a>() -> &'a String {
//     let temp  =
//     String::from("Opps want an initiative - blow up their entire quadrant!");
//     &temp // same as return &temp;
//     // <- temp would be freed here,
//    //    the returned reference cannot outlive it
// }

// fn main() {
//     let the_11th_commandment =
//     String::from("Opps want an initiative - blow up their entire quadrant!");
//     print_my_string(&the_11th_commandment);
//     print_my_string(&the_11th_commandment);
//     // you can also create a reference and store it in a variable
//     let string_ref = &the_11th_commandment;
//     print_my_string(string_ref);
//     // <- string_ref is destroyed here
//     // <- the_11th_commandment is destroyed here
//     }

fn completely_safe_storage(value: String) -> String {
    // <- value is immediately freed
    value
}
fn main() {
    let x = String::from("1337 US Dollars");
    let x = completely_safe_storage(x);
    // ↑ ownership of x was moved to completely_safe_storage()
    println!("{}", x);
    // ↑ this does not compile, as we no longer have the ownership of x
}
