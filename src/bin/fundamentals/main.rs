struct MyBox {
    data: Box<MyStruct>,
}

use std::ops::Deref;

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

fn take_slice(s: &[u32]) {
    println!("This is the slice provided: {s:?}");
}

fn test_deref_coercions_method_calls() {
    let _ms = MyStruct::new();
    println!("{}", _ms.len());

    let mb = MyBox::new();
    mb.len();
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 9, 0];
    take_slice(&arr);
    let _ = arr.len();
    let _slice = &arr[..].len();

    test_deref_coercions_method_calls();
    println!("Running fundamentals");
}

