#![cfg_attr(debug_assertions, allow(unused))]

use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, UdpSocket};
use crate::{init_logger, Answer, DnsType, Request};

pub struct Resolver {
    server: Vec<SocketAddr>, //后期考虑支持https,quic,h3,tls等类型地址,相关的支持放入net包中
}
//在这一层的抽象中，由于已经足够平铺了，可以实现Error类型了，放入error包中
//error相关的建议自写，不要用this_error,snafu，都试过了(其实snafu还行)

//client后期在需要多模块的时候可以单独开个目录
impl Resolver {
    pub fn new(server: Vec<SocketAddr>) -> Resolver {
        Resolver {
            server
        }
    }
    
    
    //后期需要做多server下轮询/并发
    //以及获取返回最快dns服务器的结构/返回所有结果中最快的ip
    //详见smart_dns
    pub fn query_a(domain: String) -> IpAddr {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap(); //这玩意得看情况先监听还是非监听，或者再想想
        socket.connect("223.5.5.5:53").unwrap();
        let mut buf = [0_u8; 1500];
        let arr = Request::new(domain, DnsType::A.into()).encode_into(&mut buf).unwrap();
        if arr.len() > 512 { 
            panic!("需要用tcp");
        }
        socket.send(arr).unwrap();
        let number_of_bytes = socket.recv(&mut buf)
            .expect("Didn't receive data");
        let answer = Answer::new(&buf[..number_of_bytes]).unwrap();
        todo!()
        // todo get_answer
    }

    pub fn query_a_tcp(domain: String) {
        init_logger();
        let mut stream = TcpStream::connect("223.5.5.5:53").unwrap();
        let mut buf = [0_u8; 1500];
        stream.write_all(Request::new("www.baidu.com".to_string(), DnsType::A.into()).encode_into_tcp(&mut buf).unwrap()).unwrap();
        stream.read(&mut buf).unwrap();
        let len = u16::from_be_bytes([buf[0], buf[1]]);
        let answer = Answer::new(&buf.as_slice()[2..]).unwrap();
        println!("{:?}",answer);
    }

    pub fn query_aaaa(domain: String) -> IpAddr {
        let mut stream = TcpStream::connect("223.5.5.5").unwrap();
        let mut buf = [0_u8; 1500];
        stream.write_all(Request::new(domain, DnsType::A.into()).encode_into(&mut buf).unwrap()).unwrap();
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();
        let answer = Answer::new(buffer.as_slice()).unwrap();
        todo!()
        // todo get_answer
    }
}