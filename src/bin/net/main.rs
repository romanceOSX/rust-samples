use std::net::{self, ToSocketAddrs};

struct MyStruct {
    foo: u32,
    bar: String,
}

impl MyStruct {
    fn new() -> Self {
        Self {
            foo: 0,
            bar: String::new(), 
        }
    }
}

fn test_std_net() { 
    //let s1 = std::net::SocketAddrV4;

    //let sock = net::SocketAddrV4::new(std::net::Ipv4Addr::new(172, 0, 0, 1));
    //sock.to_socket_addrs();
    //let conn = net::UdpSocket::bind(net::SocketAddrV4::new());
    //std::option;
}

fn get_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn test_lifetimes() {
    let res;

    {
        let a = String::from("this is longggggg");
        let b = String::from("short");
        res = get_longest(&a, &b);
    }

    /* 
     * The compiler does not really care about the lifetime of the res binding,
     * at the end it only assigns it the lifetime of one of the input arguments,
     * if the binding uses it outside the lifetime of the assigned variable then it is an error
     */
    //println!("{res}");
}

fn main() {
    //test_some_iterator();
    test_std_net();
    test_lifetimes();
}

