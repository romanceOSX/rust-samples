use std::ptr::read_unaligned;

#[repr(packed)]
struct Foo {
    a: i32,
}

fn parse_header(bytes: &[u8]) -> Foo {
    assert!(bytes.len() >= std::mem::size_of::<Foo>());
    unsafe {
        /* creates a copy, not actually edits the underlying struct */
        std::ptr::read_unaligned(bytes.as_ptr() as *const Foo)
    }
}

/* get a slice, return a reference to that slice but as a Foo struct */
fn cast_to_stub(bytes: &[u8]) -> Foo {

}

fn main() {
    let mut address = 0x012345usize;
    let r1 = address as *const i32;
    let r2 = &raw mut address;
    unsafe {
        //println!("This is an unsafe deref of a const pointer: {}", *r1);
        //println!("This is an unsafe deref of a mut pointer: {}", *r2);
    }

    /* slices unsafe test */
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &v[..];
    let (a, b) = r.split_at(3);

    println!("Printing complete vector {:?}", v);
    println!("First half: {:?}", a);
    println!("Second half: {:?}", b);

    /* type punning test */
    let rv = &v;
}

//fn split_at_mut(slice: &mut [i32], val: usize) -> (&mut [i32], &mut [i32]) {
//    //(&mut slice[..val], &mut slice[val..])
//}

