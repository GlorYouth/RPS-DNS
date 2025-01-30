#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr, UdpSocket};
use crate::{Answer, DnsType, Request};

pub struct DNSClient {
    server: Vec<SocketAddr>, //后期考虑支持https,quic,h3,tls等类型地址,相关的支持放入net包中
}
//在这一层的抽象中，由于已经足够平铺了，可以实现Error类型了，放入error包中
//error相关的建议自写，不要用this_error,snafu，都试过了(其实snafu还行)

//client后期在需要多模块的时候可以单独开个目录
impl DNSClient {
    pub fn new(server: Vec<SocketAddr>) -> DNSClient {
        DNSClient { server }
    }
    
    //后期需要做多server下轮询/并发
    //以及获取返回最快dns服务器的结构/返回所有结果中最快的ip
    //详见smart_dns
    pub fn query(domain: String) -> IpAddr {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap(); //这玩意得看情况先监听还是非监听，或者再想想
        socket.connect("223.5.5.5:53").unwrap();
        let mut buf = [0_u8; 1500];
        let len = Request::new(domain, DnsType::A.into()).encode_into(&mut buf).unwrap();
        socket.send(&buf[0..len]).unwrap();
        let number_of_bytes = socket.recv(&mut buf)
            .expect("Didn't receive data");
        let answer = Answer::new(&buf[..number_of_bytes]).unwrap();
        todo!()
        // todo get_answer
    }
}