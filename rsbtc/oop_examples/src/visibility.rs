// pub — visible everywhere this module is imported
pub fn public_function() {
    println!("visible everywhere");
}

// no pub — private to this module only
fn private_function() {
    println!("private to visibility.rs");
}

// pub(crate) — visible anywhere within the oop_examples crate
// but not to external crates that might depend on this one
pub(crate) fn crate_function() {
    println!("visible within this crate only");
}

// calls both public and private — private is accessible within
// the same file
pub fn run() {
    public_function();
    private_function();
    crate_function();
}
