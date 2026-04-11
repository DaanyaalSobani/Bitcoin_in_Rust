// fn print_my_string(string: &String){
//     println!("{}",string);
// }
// // This function won't compile it returns a ref that is owned by the function
// // fn give_me_a_ref<'a>() -> &'a String {
// //  let temp  =
// //  String::from("Opps want an initiative - blow up their enti
// // re quadrant!");
// //    &temp // same as return &temp;
// //     // <- temp would be freed here,
// //    //    the returned reference cannot outlive it
// //  }
// fn main() {
//      let the_11th_commandment = String::from("Opps want an initiative - blow up their entire quadrant!");

//      print_my_string(&the_11th_commandment);
//      print_my_string(&the_11th_commandment);

//      let string_ref = &the_11th_commandment;
//      print_my_string(string_ref);
// }
//  fn completely_safe_storage(value: String) {
//    // <- value is immediately freed
//  }
//  fn main() {
//  let x = String::from("1337 US Dollars");
//  completely_safe_storage(x);
//    // ↑ ownership of x was moved to completely_safe_storage()
//  println!("{}", x);
//    // ↑ this does not compile, as we no longer have the ownership 
// // of x
//  }
// fn takes_reference(my_ref: &String) {
//    // <- reference is moved into this function
//  println!("{}", my_ref);
//    // ↑ this macro actually takes all arguments by reference
//    // so a &&String is created here, which is moved into the
//    // internals of the macro
//    // <- my_ref is destroyed here
//  }
//  fn main() {
//     let x = String::from("Hello, World!");
//     let reference = &x;

//     takes_reference(reference);
//  }

//  fn main() {
//     let my_str = "Hello, world!";
//     let hell = &my_str[..4];
//     println!("{} {}!", my_str, hell);
//  }

struct Hello;

trait SayHi {
    fn say_hi(self);
 }
 impl SayHi for Hello {
 fn say_hi(self) {
    println!("This hi! will cost me my life - I am owned value");
   }
 }
impl SayHi for &Hello {
    fn say_hi(self) {
        println!("Hi, I am a reference to Hello!");
    }
 }
impl SayHi for &&Hello {
 fn say_hi(self) {
    println!("Hi, I am a double reference to Hello!");
 }
 }
 fn main() {
 let hello = Hello;
   (&hello).say_hi();
   (&&hello).say_hi();
    hello.say_hi();
 }