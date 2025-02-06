#![cfg_attr(debug_assertions, allow(unused_variables, dead_code))]

use crate::dns::error::Error;
use crate::dns::net::NetQuery;
use crate::dns::utils::ServerType;
use crate::dns::{DnsType, Request};
use smallvec::SmallVec;
use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr, TcpStream, UdpSocket};
use std::rc::Rc;

pub struct Resolver {
    server: SmallVec<[ServerType; 5]>,
}

impl Resolver {
    pub fn new(mut server: Vec<String>) -> Result<Resolver, AddrParseError> {
        let vec = server
            .iter_mut()
            .try_fold(SmallVec::new(), |mut vec, str| {
                vec.push(ServerType::from_string(str)?);
                Ok(vec)
            })?;
        Ok(Resolver { server: vec })
    }

    //后期需要做多server下轮询/并发
    //以及获取返回最快dns服务器的结构/返回所有结果中最快的ip
    //详见smart_dns
    pub fn query_a(&self, domain: String) -> Result<Option<Ipv4Addr>, Error> {
        let domain = Rc::new(domain);
        for server in &self.server {
            return match server {
                ServerType::Tcp(addr) => {
                    //后面可以考虑复用连接
                    let stream = TcpStream::connect(addr).unwrap();
                    let buf = [0_u8; 1500];
                    let request = Request::new(domain.clone(), DnsType::A.into());
                    let response = NetQuery::query_tcp(stream, request, buf)?;
                    Ok(response.get_a_record())
                }
                ServerType::Udp(addr) => {
                    let socket = UdpSocket::bind("0.0.0.0:0").unwrap(); //这玩意得看情况先监听还是非监听，或者再想想
                    socket.connect(addr).unwrap();
                    let buf = [0_u8; 1500];
                    let request = Request::new(domain.clone(), DnsType::A.into());
                    let response = NetQuery::query_udp(socket, request, buf)?;
                    Ok(response.get_a_record())
                }
            };
        }
        Err(Error::NoServerAvailable)
    }
    
    pub fn query_aaaa(&self, domain: String) -> Result<Option<Ipv6Addr>, Error> {
        let domain = Rc::new(domain);
        for server in &self.server {
            return match server {
                ServerType::Tcp(addr) => {
                    //后面可以考虑复用连接
                    let stream = TcpStream::connect(addr).unwrap();
                    let buf = [0_u8; 1500];
                    let request = Request::new(domain.clone(), DnsType::AAAA.into());
                    let response = NetQuery::query_tcp(stream, request, buf)?;
                    Ok(response.get_aaaa_record())
                }
                ServerType::Udp(addr) => {
                    let socket = UdpSocket::bind("0.0.0.0:0").unwrap(); //这玩意得看情况先监听还是非监听，或者再想想
                    socket.connect(addr).unwrap();
                    let buf = [0_u8; 1500];
                    let request = Request::new(domain.clone(), DnsType::AAAA.into());
                    let response = NetQuery::query_udp(socket, request, buf)?;
                    Ok(response.get_aaaa_record())
                }
            };
        }
        Err(Error::NoServerAvailable)
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::resolver::Resolver;
    use crate::dns::error::init_logger;

    #[test]
    fn test_query_a() {
        init_logger();
        let server = vec![
            "223.5.5.5".to_string(),
        ];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_a("www.baidu.com".to_string()).unwrap().unwrap();
        println!("{:?}", result);
    }
    
    #[test]
    fn test_query_aaaa() {
        init_logger();
        let server = vec![
            "223.5.5.5".to_string(),
        ];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_aaaa("www.baidu.com".to_string()).unwrap().unwrap();
        println!("{:?}", result);
    }
}