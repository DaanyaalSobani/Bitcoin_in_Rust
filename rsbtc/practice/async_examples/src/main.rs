async fn foobar() {
    println!("Back to the future");
}
fn main() {
    println!("Hello");
    let x = foobar();
    println!("What's your favorite movie?");
    futures::executor::block_on(x);
}
