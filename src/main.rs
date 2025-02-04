use std::io::{Read, Write};
use crate::dns::{init_logger, Answer, DnsType};
use dns_core::{Request};
use std::net::{TcpStream, UdpSocket};


mod dns;
fn main() {
    init_logger();
    let mut stream = TcpStream::connect("223.5.5.5:53").unwrap();
    let mut buf = [0_u8; 1500];
    stream.write_all(Request::new("www.baidu.com".to_string(), DnsType::A.into()).encode_into_tcp(&mut buf).unwrap()).unwrap();
    stream.read(&mut buf).unwrap();
    let len = u16::from_be_bytes([buf[0], buf[1]]);
    let answer = Answer::new(&buf.as_slice()[2..(len + 2) as usize]).unwrap();
    println!("{:?}",answer);
    return;
}

