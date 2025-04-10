use std::net::UdpSocket;
use std::thread::Thread;

/* TX thread */
fn _tx_thread() {
    let socket = UdpSocket::bind("127.0.0.1:12345").unwrap();
    let mut buf = [0; 10];
    let (len, addr) = socket.recv_from(&mut buf).unwrap();

    println!("Received {} bytes from {:?}", len, addr);
}

/* RX thread */
fn _rx_thread() {
    let socket = UdpSocket::bind("127.0.0.1:12346").unwrap();
    let mut buf = [1, 2, 3, 4, 5, 6];
    buf.reverse();
    let addr =  "127.0.0.1:12345";
    let len = socket.send_to(&buf, addr).unwrap();
    println!("Sent {} bytes to {}", len, addr);
}

fn main() {
    let rx_thread = std::thread::spawn(_rx_thread);
    let tx_thread = std::thread::spawn(_tx_thread);

    let thread_list = vec![rx_thread, tx_thread];

    for thread in thread_list {
        thread.join().unwrap();
    }
}

