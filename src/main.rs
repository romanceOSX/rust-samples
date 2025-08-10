#[allow(dead_code)]

use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn test_even_shorter_version() -> Result<String, io::Error> {
    std::fs::read_to_string("Hello.txt")
}

fn test_propagating_errors() -> Result<String, io::Error> {
    let mut username_str = String::new();

    File::open("hello.txt")?.read_to_string(&mut username_str)?;

    /* The following statements do the same thing */
    /// let _file = File::open("filename.txt")?;
    /* '?' would be equivalent to the follwing */
    /// let _file = match File::open("filename.txt") {
    ///     Ok(f) => f,
    ///     Err(e) => return Err(e),
    /// };

    Ok(username_str)
}

fn test_opening_file_2() {
    match test_propagating_errors() {
        Ok(s) => println!("Name inside file: {s:?}"),
        Err(e) => println!("Error opening file: {e:?}"),
    }
}

fn test_opening_file() {
    let filename = String::from("this_file_does_not_exists.txt");

    let f = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create_new(&filename) {
                Ok(f) => f,
                Err(e) => panic!("Could not create file {e:?}"),
            }
            _ => {
                panic!("Problem opening the file: {e:?}");
            }
        }
    };
}

fn input_bytes(bytes: &[u8]) {
    println!("You inputted {:?}", bytes);
}

fn foo(arr: &[u32]) {
    println!("arr:?");
}

fn test_strings() {
    let str = String::from("This is just a message");
    let str_slice = "This is a fundamental str string";
    let _ = str_slice.as_bytes();
    input_bytes(str.as_bytes());
}

fn test_arrays() {
    let a = [10; 1];
    //a.as_slice().align_to();
}

fn test_vecs() {
    let vec = vec![1, 2, 3, 4, 5];
    vec[99];
}

fn main() {
    //test_vecs();
    test_opening_file_2()
}
