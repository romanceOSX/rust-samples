struct Counter {
    counter: usize,
}

impl Counter {
    fn new() -> Self {
        Self {counter: 0}
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;

        if self.counter < 6 {
            Some(self.counter)
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut a = Counter::new();
    println!("Printing nth value: {}", a.nth(34).unwrap())
}
