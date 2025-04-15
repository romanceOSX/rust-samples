use std::ptr::read_unaligned;

fn parse_header(bytes: &[u8]) {
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
}

fn split_at_mut(slice: &mut [i32], val: usize) -> (&mut [i32], &mut [i32]) {
    (&mut slice[..val], &mut slice[val..])
}

