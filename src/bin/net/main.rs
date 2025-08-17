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

use std::fmt::Debug;

fn print_iterator<T>(val: T)
where 
    T: IntoIterator,
    T: Debug,
{
    for i in val.into_iter() {
        sldkfj
        println!("Value: {:?}", i);
    }
}

fn test_some_iterator() {
    let a = Some(21 as u32);
    for i in a.into_iter() {
        println!("{i}");
    }

    // testing with user-defined struct
    let m = Some(MyStruct::new());
    m.into_iter();
}

fn main() {
    test_some_iterator();
    test_std_net();
}

