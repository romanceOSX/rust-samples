
struct MyStruct<A, B> {
    a: A,
    b: B,
}

// Default implementation for all A, B
impl<A, B> MyStruct<A, B> {
    fn describe(&self) {
        println!("Generic case: two possibly different types");
    }
}

// More specific implementation when A and B are the same type
impl<T> MyStruct<T, T> {
    fn describe(&self) {
        println!("Special case: both types are the same");
    }
}

fn main() {
    let x = MyStruct { a: 1, b: 2 };
    x.describe(); // Special case: both types are the same

    let y = MyStruct { a: 1, b: 2.0 };
    y.describe(); // Generic case: two possibly different types
}
