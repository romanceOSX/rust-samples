struct MyBox {
    data: Box<MyStruct>,
}

use std::{fmt::Debug, ops::Deref};

impl Deref for MyBox {
    type Target = MyStruct;
    fn deref(&self) -> &Self::Target {
        &(*self.data)
    }
}

impl MyBox {
    fn new() -> Self {
        Self { 
            data: Box::new(MyStruct::new()),
        }
    }
    /* what happens if the Deref'able type implements a method of the same name? */
    // This first reference explains the overall candidate list mechanism
    // --> https://doc.rust-lang.org/stable/reference/expressions/method-call-expr.html
    // This second link explains how to disambiguate the call
    // --> https://doc.rust-lang.org/stable/reference/expressions/call-expr.html#disambiguating-function-calls
}

trait LenFoo {
    fn len(&self) -> u32;
}

trait LenBar {
    fn len(&self) -> u32;
}

struct MyStruct {
    content: u32,
}

impl MyStruct {
    fn new() -> Self {
        Self { content: 0 }
    }
}

//impl LenBar for MyStruct {
//    fn len(&self) -> u32 {
//        self.content
//    }
//}

impl LenFoo for MyStruct {
    fn len(&self) -> u32 {
        self.content * 2
    }
}

impl LenFoo for MyBox {
    fn len(&self) -> u32 {
        32
    }
}

fn take_slice(s: &[u32]) {
    println!("This is the slice provided: {s:?}");
}

fn test_deref_coercions_method_calls() {
    let _ms = MyStruct::new();
    println!("{}", _ms.len());

    let mb = MyBox::new();
    println!("{}", mb.len());
}

fn print_collection<T>(c: &T)
where 
    T: Debug,
{
    println!("Printing collection... {c:?}");
}

fn test_iters() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    for e in v.iter_mut() {
        println!("Calling vec");
        *e = 32;
    }
    print_collection(&v);

    let v2 = vec![2, 3, 4, 1, 2, 3, 4];
    let res: Vec<_> = v2.iter().map_while(|x| {
        println!("Testing somee");
        if *x > 3 {
            return None;
        }
        Some(x)
    })
        .collect();

    print_collection(&res);
}

fn test_prev() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 9, 0];
    take_slice(&arr);
    let _ = arr.len();
    let _slice = &arr[..].len();

    println!("Running fundamentals");
}

fn test_strings() {
    let s = String::from("Hey");
    for c in s.bytes() {
        println!("{c}");
    }
}

fn main() {
    test_strings();
    //test_iters();
    //test_deref_coercions_method_calls();
}

