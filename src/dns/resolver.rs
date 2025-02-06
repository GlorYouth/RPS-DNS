#![cfg_attr(debug_assertions, allow(unused_variables, dead_code))]

use crate::dns::RecordDataType;
use crate::dns::Response;
use crate::dns::error::Error;
use crate::dns::net::{NetQuery, NetQueryError};
use crate::dns::utils::ServerType;
use crate::dns::{DnsType, Request};
#[cfg(debug_assertions)]
use log::debug;
use smallvec::SmallVec;
use std::fmt::{Debug, Display, Formatter};
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
    #[inline]
    pub fn query_a(&self, domain: String) -> QueryResult {
        self.query(domain, DnsType::A.into())
    }

    #[inline]
    pub fn query_aaaa(&self, domain: String) -> QueryResult {
        self.query(domain, DnsType::AAAA.into())
    }

    #[inline]
    pub fn query_cname(&self, domain: String) -> QueryResult {
        self.query(domain, DnsType::CNAME.into())
    }

    fn query(&self, domain: String, qtype: u16) -> QueryResult {
        let domain = Rc::new(domain);
        let mut error_vec = SmallVec::new();
        let buf = [0_u8; 1500];
        for server in &self.server {
            return match server {
                ServerType::Tcp(addr) => {
                    //后面可以考虑复用连接
                    if let Ok(stream) = TcpStream::connect(addr) {
                        let request = Request::new(domain.clone(), qtype);
                        match NetQuery::query_tcp(stream, request, buf) {
                            Ok(response) => response.into(),
                            Err(e) => {
                                error_vec.push(e.into());
                                continue;
                            }
                        }
                    } else {
                        #[cfg(debug_assertions)]
                        debug!("连接到对应的tcp server失败");
                        error_vec.push(Error::from(NetQueryError::ConnectTcpAddrError));
                        continue; //连接到server失败, 则尝试备用server
                    }
                }
                ServerType::Udp(addr) => {
                    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
                        if let Ok(addr) = socket.connect(addr) {
                            let request = Request::new(domain.clone(), qtype);
                            match NetQuery::query_udp(socket, request, buf) {
                                Ok(response) => response.into(),
                                Err(e) => {
                                    error_vec.push(e.into());
                                    continue;
                                }
                            }
                        } else {
                            #[cfg(debug_assertions)]
                            debug!("连接到对应的udp server失败");
                            error_vec.push(Error::from(NetQueryError::ConnectUdpAddrError));
                            continue;
                        }
                    } else {
                        #[cfg(debug_assertions)]
                        debug!("监听udp端口失败");
                        error_vec.push(Error::from(NetQueryError::BindUdpAddrError));
                        continue; //监听udp失败，尝试备用
                    }
                }
            };
        }
        error_vec.into()
    }
}

type ErrorVec = SmallVec<[Error; 3]>;

pub struct QueryResult {
    response: Option<Response>,
    error: ErrorVec,
}

impl QueryResult {
    #[inline]
    fn get_a_record(&self) -> Option<Ipv4Addr> {
        self.response
            .as_ref()
            .and_then(|res| res.get_record(DnsType::A.into())) // 尝试获取 A 记录
            .and_then(|record| {
                if let RecordDataType::A(addr) = record {
                    Some(addr)
                } else {
                    None
                }
            })
    }

    #[inline]
    fn get_aaaa_record(&self) -> Option<Ipv6Addr> {
        self.response
            .as_ref()
            .and_then(|res| res.get_record(DnsType::AAAA.into())) // 尝试获取 A 记录
            .and_then(|record| {
                if let RecordDataType::AAAA(addr) = record {
                    Some(addr)
                } else {
                    None
                }
            })
    }

    #[inline]
    fn get_cname_record(&self) -> Option<String> {
        self.response
            .as_ref()
            .and_then(|res| res.get_record(DnsType::CNAME.into()))
            .and_then(|record| {
                if let RecordDataType::CNAME(name) = record {
                    Some(name)
                } else {
                    None
                }
            })
    }
}

impl From<Option<Response>> for QueryResult {
    fn from(value: Option<Response>) -> Self {
        Self {
            response: value,
            error: Default::default(),
        }
    }
}

impl From<ErrorVec> for QueryResult {
    fn from(value: ErrorVec) -> Self {
        Self {
            response: None,
            error: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::error::init_logger;
    use crate::dns::resolver::Resolver;

    #[test]
    fn test_query_a() {
        init_logger();
        let server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver
            .query_a("www.baidu.com".to_string())
            .get_a_record()
            .unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn test_query_aaaa() {
        init_logger();
        let server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver
            .query_aaaa("www.baidu.com".to_string())
            .get_aaaa_record()
            .unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn test_query_cname() {
        init_logger();
        let server = vec!["9.9.9.9".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver
            .query_cname("www.baidu.com".to_string())
            .get_cname_record()
            .unwrap();
        println!("{:?}", result);
    }
}
