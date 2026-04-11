// ============================================================
// Exercise 1 — Struct and impl
// ============================================================
pub struct Wallet {
    owner: String,
    balance: u64,
}

impl Wallet {
    pub fn new(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
            balance: 0,
        }
    }
    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }
    pub fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        true
    }
    pub fn balance(&self) -> u64 {
        self.balance
    }
}

// ============================================================
// Exercise 2 — Trait basics
// ============================================================
pub trait Describable {
    fn describe(&self) -> String;

    // Exercise 4 — default implementation
    // Wallet uses this as-is, Transaction overrides it
    fn summarise(&self) -> String {
        format!("Item: {}", self.describe())
    }
}

pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: u64) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            amount,
        }
    }
}

impl Describable for Wallet {
    fn describe(&self) -> String {
        let owner = &self.owner;
        let balance = self.balance;
        format!("Owner: {owner} | Balance: {balance}")
    }
    // uses default summarise()
}

impl Describable for Transaction {
    fn describe(&self) -> String {
        let from = &self.from;
        let to = &self.to;
        let amount = self.amount;
        format!("From: {from} | To: {to} | Total: {amount}")
    }

    // Exercise 4 — override default summarise() for Transaction
    fn summarise(&self) -> String {
        let amount = self.amount;
        let from = &self.from;
        let to = &self.to;
        format!("Transfer of {amount} sats: {from} -> {to}")
    }
}

// ============================================================
// Exercise 3 — Static vs Dynamic dispatch
// ============================================================
pub fn print_description<T: Describable>(desc: &T) {
    println!("{}", desc.describe());
}

pub fn print_static<T: Describable>(item: &T) {
    print_description(item);
}

pub fn print_dynamic(item: &dyn Describable) {
    println!("{}", item.describe());
}

// heterogeneous collection — only possible with dynamic dispatch
pub fn print_all(items: &[Box<dyn Describable>]) {
    for item in items {
        println!("{}", item.describe());
    }
}

// ============================================================
// Exercise 5 — Visibility
// ============================================================
pub mod bitcoin {
    pub struct Block {
        pub height: u64,
        nonce: u64,     // private — not accessible outside this module
    }

    impl Block {
        pub fn new(height: u64) -> Self {
            Self { height, nonce: 0 }
        }

        pub(crate) fn mine(&mut self) {
            self.nonce += 1;
        }

        pub fn height(&self) -> u64 {
            self.height
        }
    }
}

// ============================================================
// Stretch — Supertraits
// ============================================================
// Printable requires Describable — can't implement one without the other
pub trait Printable: Describable {
    fn print(&self) {
        println!("{}", self.describe());
    }
}

// empty impl — print() comes from the default implementation
impl Printable for Wallet {}
impl Printable for Transaction {}

// To verify supertraits are enforced, uncomment the block below.
// It will produce: error[E0277]: the trait bound `Orphan: Describable` is not satisfied
//
// struct Orphan;
// impl Printable for Orphan {}
