extern crate core;

#[cfg(debug_assertions)]
use crate::dns::error::init_logger;
use crate::dns::{DnsType, ResponseCheck};
use dns_core::Request;
use std::io::{Read, Write};
use std::net::TcpStream;

mod dns;
fn main() {
    #[cfg(debug_assertions)]
    init_logger();
    let mut stream = TcpStream::connect("223.5.5.5:53").unwrap();
    let mut buf = [0_u8; 1500];
    let request = Request::new("www.baidu.com".to_string(), DnsType::A.into());
    stream
        .write_all(request.encode_to_tcp(&mut buf).unwrap())
        .unwrap();
    stream.read(&mut buf).unwrap();
    let len = u16::from_be_bytes([buf[0], buf[1]]);
    let response = ResponseCheck::new(&request)
        .check_into_response(&buf.as_slice()[2..(len + 2) as usize])
        .unwrap();
    println!("{:#?}", response);
    return;
}
