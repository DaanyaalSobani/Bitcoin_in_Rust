// Fields are private by default — callers must go through methods
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // Associated function — no self parameter, used as a constructor
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0.0,
        }
    }

    // &mut self — mutable borrow of the struct
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // &self — immutable borrow, read only
    pub fn average(&self) -> f64 {
        self.average
    }

    // Private method — not visible outside this impl block
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub fn run() {
    let mut col = AveragedCollection::new();
    col.add(10);
    col.add(20);
    col.add(30);
    println!("average: {}", col.average());

    col.remove();
    println!("average after remove: {}", col.average());
}
