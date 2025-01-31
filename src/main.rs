use crate::dns::{Answer, DnsType};
use dns_core::{Request};
use std::net::UdpSocket;

mod dns;
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("223.5.5.5:53").unwrap();
    let mut buf = [0_u8; 1500];
    let arr = Request::new("www.google.com".to_string(), DnsType::A.into()).encode_into(&mut buf).unwrap();
    socket.send(arr).unwrap();
    let number_of_bytes = socket.recv(&mut buf)
        .expect("Didn't receive data");
    let answer = Answer::new(&buf[..number_of_bytes]).unwrap();
    println!("{:?}", answer);
    return;
}

