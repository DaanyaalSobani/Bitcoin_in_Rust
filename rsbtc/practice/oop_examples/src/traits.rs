// A trait defines a contract — a set of methods a type must provide
pub trait Quack {
    fn quack(&self);

    // Default implementation — types can override this or use it as-is
    fn describe(&self) {
        println!("I am something that quacks");
    }
}

pub struct Duck;

pub struct Human;

pub struct FormalDuck {
    pub name: String,
}

impl FormalDuck {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

// Duck explicitly implements the Quack trait
impl Quack for Duck {
    fn quack(&self) {
        println!("quack");
    }
}

// Human has a quack() method but does NOT implement the Quack trait
// this is the key difference from duck typing — the method existing
// is not enough, you must explicitly declare impl Quack for Human
impl Human {
    pub fn quack(&self) {
        println!("I quack, therefore I am");
    }
}

impl Quack for FormalDuck {
    fn quack(&self) {
        println!("Good evening. My name is {}. quack.", self.name);
    }

    // Overriding the default describe()
    fn describe(&self) {
        println!("I am FormalDuck {}", self.name);
    }
}
