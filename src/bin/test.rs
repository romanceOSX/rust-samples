trait MyCustomTrait {
    fn my_custom_printer(&self);
}

impl<T> MyCustomTrait for T
    where T: std::fmt::Debug,
{
    fn my_custom_printer(&self) {
        println!("Uhmm.. this is my custom printer!");
        println!("{:?}", &self);
    }
}

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
    // this is the specialization feature of rust, only available in nightly builds
    //fn describe(&self) {
    fn describe_foo(&self) {
        println!("Special case: both types are the same");
    }
}

fn longer_string<'a>(str_a: &'a str, str_b: &'a str) -> &'a str {
    if str_a > str_b { str_a } else { str_b }
}

fn main() {
    32.my_custom_printer();

    let a = String::from("lkdjafsljf");
    let b = String::from ("aaa");

    let longer = longer_string(&a, &b);
    println!("The longer string is {:?}", longer);

    //3.my_custom_printer();
    //let x = MyStruct { a: 1, b: 2 };
    //x.describe(); // Special case: both types are the same

    //let y = MyStruct { a: 1, b: 2.0 };
    //y.describe(); // Generic case: two possibly different types
}
