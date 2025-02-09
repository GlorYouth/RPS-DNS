#![cfg_attr(debug_assertions, allow(unused_variables, dead_code))]

use crate::dns::RawDomain;
use crate::dns::RecordDataType;
use crate::dns::Response;
use crate::dns::error::Error;
use crate::dns::net::{NetQuery, NetQueryError};
use crate::dns::utils::ServerType;
use crate::dns::{DnsTypeNum, Request};
#[cfg(feature = "logger")]
use log::debug;
use smallvec::SmallVec;
use std::fmt::Display;
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
        self.query(domain, DnsTypeNum::A)
    }

    #[inline]
    pub fn query_aaaa(&self, domain: String) -> QueryResult {
        self.query(domain, DnsTypeNum::AAAA)
    }

    #[inline]
    pub fn query_cname(&self, domain: String) -> QueryResult {
        self.query(domain, DnsTypeNum::CNAME)
    }

    fn query(&self, domain: String, qtype: u16) -> QueryResult {
        let mut error_vec = SmallVec::new();
        if let Some(domain) = RawDomain::from_str(domain.as_str()) {
            let domain = Rc::new(domain);
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
                            #[cfg(feature = "logger")]
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
                                #[cfg(feature = "logger")]
                                debug!("连接到对应的udp server失败");
                                error_vec.push(Error::from(NetQueryError::ConnectUdpAddrError));
                                continue;
                            }
                        } else {
                            #[cfg(feature = "logger")]
                            debug!("监听udp端口失败");
                            error_vec.push(Error::from(NetQueryError::BindUdpAddrError));
                            continue; //监听udp失败，尝试备用
                        }
                    }
                };
            }
            error_vec.into()
        } else {
            error_vec.push(Error::StringParseError(domain));
            error_vec.into()
        }
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
            .and_then(|res| res.get_record(DnsTypeNum::A)) // 尝试获取 A 记录
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
            .and_then(|res| res.get_record(DnsTypeNum::AAAA)) // 尝试获取 A 记录
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
            .and_then(|res| res.get_record(DnsTypeNum::CNAME))
            .and_then(|record| {
                if let RecordDataType::CNAME(name) = record {
                    Some(name.to_string()?)
                } else {
                    None
                }
            })
    }
}

impl Display for QueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(res) = &self.response {
            Display::fmt(&res, f)?;
        } else {
            for e in self.error.iter() {
                writeln!(f, "{}", e)?;
            }
        }
        Ok(())
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
    #[cfg(feature = "logger")]
    use crate::dns::error::init_logger;
    use crate::dns::resolver::Resolver;

    #[test]
    fn test_query_a() {
        #[cfg(feature = "logger")]
        init_logger();
        let server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_a("www.baidu.com".to_string());
        if let Some(answer) = result.get_a_record() {
            println!("{}", answer);
        } else {
            println!("No A record");
            println!("{}", result);
        }
    }

    #[test]
    fn test_query_aaaa() {
        #[cfg(feature = "logger")]
        init_logger();
        let server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_aaaa("www.google.com".to_string());
        if let Some(answer) = result.get_aaaa_record() {
            println!("{}", answer);
        } else {
            println!("No AAAA record");
            println!("{}", result);
        }
    }

    #[test]
    fn test_query_cname() {
        #[cfg(feature = "logger")]
        init_logger();
        let server = vec!["9.9.9.9".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_cname("www.baidu.com".to_string());
        if let Some(answer) = result.get_cname_record() {
            println!("{}", answer);
        } else {
            println!("No CNAME record");
            println!("{}", result);
        }
    }

    #[test]
    fn test_fmt() {
        #[cfg(feature = "logger")]
        init_logger();
        let server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_a("www.baidu.com".to_string());
        println!("{}", result);
    }
}
