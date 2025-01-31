#![allow(dead_code)]

use crate::{Answer, DnsType, Request};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

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
    pub fn query_a(domain: String) -> Option<Ipv4Addr> {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap(); //这玩意得看情况先监听还是非监听，或者再想想
        socket.connect("223.5.5.5:53").unwrap();
        let mut buf = [0_u8; 1500];
        let len = Request::new(domain, DnsType::A.into())
            .encode_into(&mut buf)
            .unwrap();
        socket.send(&buf[0..len]).unwrap();
        let number_of_bytes = socket.recv(&mut buf).expect("Didn't receive data");
        let mut answer = Answer::new(&buf[..number_of_bytes]).unwrap();

        // 权威的
        while let Some(record) = answer.authority.pop() {
            if record.is_a() {
                return record.data.get_addr_a();
            }
        }

        // 非权威的
        while let Some(record) = answer.answer.pop() {
            if record.is_a() {
                return record.data.get_addr_a();
            }
        }
        return None;
    }
}

#[test]
fn test_query_a() {
    let ip = DNSClient::query_a("baidu.com".to_string()).unwrap();
    println!("{:#?}", ip);
}