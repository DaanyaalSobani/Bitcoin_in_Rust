// Import the trait and types from the traits module
// super:: means "go up one level to the parent module (main.rs)"
// then into the traits module
use super::traits::{Duck, FormalDuck, Quack};

// STATIC DISPATCH
// The compiler generates a separate version of this function for
// each concrete type — ducks_say::<Duck> and ducks_say::<FormalDuck>
// exist as separate functions in the compiled binary.
// No runtime cost, but larger binary.
pub fn ducks_say_static<T: Quack>(quacker: T) {
    quacker.quack();
    quacker.describe();
}

// DYNAMIC DISPATCH
// One version of this function exists. At runtime, the correct
// quack() is looked up in a vtable (virtual method table).
// Small runtime cost, smaller binary, more flexible.
pub fn ducks_say_dynamic(quacker: &dyn Quack) {
    quacker.quack();
}

// Dynamic dispatch enables heterogeneous collections —
// a Vec of different concrete types, unified by a shared trait.
// This is impossible with static dispatch.
pub fn all_quack(quackers: &[Box<dyn Quack>]) {
    for q in quackers {
        q.quack();
    }
}

pub fn run() {
    println!("-- static dispatch --");
    ducks_say_static(Duck);
    ducks_say_static(FormalDuck::new("Ernesto"));

    println!("-- dynamic dispatch --");
    let duck = Duck;
    let formal = FormalDuck::new("Gerald");
    ducks_say_dynamic(&duck);
    ducks_say_dynamic(&formal);

    println!("-- heterogeneous collection (only possible with dyn) --");
    let quackers: Vec<Box<dyn Quack>> = vec![
        Box::new(Duck),
        Box::new(FormalDuck::new("Sir Reginald")),
        Box::new(Duck),
    ];
    all_quack(&quackers);
}
