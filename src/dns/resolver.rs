#![cfg_attr(debug_assertions, allow(unused_variables, dead_code))]

#[cfg(feature = "result_error")]
use crate::dns::error::Error;
use crate::dns::error::ErrorAndOption;
use crate::dns::net::NetQuery;
#[cfg(feature = "result_error")]
use crate::dns::net::NetQueryError;
use crate::dns::types::base::{DnsTypeNum, RawDomain, record::SOA};
use crate::dns::types::parts::{RecordDataType, Request, Response};
use crate::dns::utils::ServerType;
#[cfg(feature = "logger")]
use log::debug;
use paste::paste;
use smallvec::SmallVec;
use std::net::{AddrParseError, TcpStream, UdpSocket};
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

    fn query(&self, domain: String, qtype: u16) -> QueryResult {
        #[cfg(feature = "result_error")]
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
                            #[cfg(feature = "result_error")]
                            match NetQuery::query_tcp(stream, request, buf) {
                                Ok(response) => response.into(),
                                Err(e) => {
                                    error_vec.push(e.into());
                                    continue;
                                }
                            }
                            #[cfg(not(feature = "result_error"))]
                            QueryResult::from(NetQuery::query_tcp(stream, request, buf))
                        } else {
                            #[cfg(feature = "logger")]
                            debug!("连接到对应的tcp server失败");
                            #[cfg(feature = "result_error")]
                            error_vec.push(Error::from(NetQueryError::ConnectTcpAddrError));
                            continue; //连接到server失败, 则尝试备用server
                        }
                    }
                    ServerType::Udp(addr) => {
                        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
                            if let Ok(addr) = socket.connect(addr) {
                                let request = Request::new(domain.clone(), qtype);
                                #[cfg(feature = "result_error")]
                                match NetQuery::query_udp(socket, request, buf) {
                                    Ok(response) => response.into(),
                                    Err(e) => {
                                        #[cfg(feature = "result_error")]
                                        error_vec.push(e.into());
                                        continue;
                                    }
                                }
                                #[cfg(not(feature = "result_error"))]
                                QueryResult::from(NetQuery::query_udp(socket, request, buf))
                            } else {
                                #[cfg(feature = "logger")]
                                debug!("连接到对应的udp server失败");
                                #[cfg(feature = "result_error")]
                                error_vec.push(Error::from(NetQueryError::ConnectUdpAddrError));
                                continue;
                            }
                        } else {
                            #[cfg(feature = "logger")]
                            debug!("监听udp端口失败");
                            #[cfg(feature = "result_error")]
                            error_vec.push(Error::from(NetQueryError::BindUdpAddrError));
                            continue; //监听udp失败，尝试备用
                        }
                    }
                };
            }
            #[cfg(feature = "result_error")]
            return error_vec.into();
            #[cfg(not(feature = "result_error"))]
            QueryResult::from(None)
        } else {
            #[cfg(feature = "result_error")]
            error_vec.push(Error::StringParseError(domain));
            #[cfg(feature = "result_error")]
            return error_vec.into();
            #[cfg(not(feature = "result_error"))]
            QueryResult::from(None)
        }
    }
}
#[cfg(feature = "result_error")]
type ErrorVec = SmallVec<[Error; 3]>;

#[cfg(feature = "result_error")]
#[derive(Debug)]
pub struct QueryResult(ErrorAndOption<Response, ErrorVec>);

#[cfg(not(feature = "result_error"))]
#[derive(Debug)]
pub struct QueryResult(ErrorAndOption<Response>);

//我真不想写了，用宏生成算了
macro_rules! define_get_record {
    ($fn_name:ident, $dns_type:expr, $result:ident, $result_expr:expr, $output_type:ty) => {
        paste! {
            impl QueryResult {
                #[inline]
                pub fn [<get_ $fn_name _record>](&self) -> Option<$output_type> {
                    self.0
                        .get_result()
                        .as_ref()
                        .and_then(|res| res.get_record(DnsTypeNum::$dns_type))  // 获取指定类型的 DNS 记录
                        .and_then(|record| {
                            if let RecordDataType::$dns_type($result) = record {
                                Some($result_expr)
                            } else {
                                None
                            }
                    })
                }
            }

            impl Resolver {
                #[inline]
                pub fn [<query_ $fn_name>](&self, domain: String) -> QueryResult {
                    self.query(domain, DnsTypeNum::$dns_type)
                }
            }
        }
    };
}

// the last attribute is func output type
define_get_record!(a, A, addr, addr.get_index(), std::net::Ipv4Addr);
define_get_record!(aaaa, AAAA, addr, addr.get_index(), std::net::Ipv6Addr);
define_get_record!(
    cname,
    CNAME,
    str,
    str.get_index().as_ref().to_string()?,
    String
);
define_get_record!(soa, SOA, soa, soa, SOA);
define_get_record!(ns, NS, str, str.get_index().as_ref().to_string()?, String);

#[cfg(feature = "fmt")]
impl std::fmt::Display for QueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(res) = &self.0.get_result() {
            std::fmt::Display::fmt(&res, f)?;
        } else {
            #[cfg(feature = "result_error")]
            for e in self.0.get_error().iter() {
                writeln!(f, "{}", e)?;
            }
        }
        Ok(())
    }
}

impl From<Option<Response>> for QueryResult {
    fn from(value: Option<Response>) -> Self {
        QueryResult(ErrorAndOption::from_result(value))
    }
}

#[cfg(feature = "result_error")]
impl From<ErrorVec> for QueryResult {
    fn from(value: ErrorVec) -> Self {
        QueryResult(ErrorAndOption::from_error(value))
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
            #[cfg(feature = "fmt")]
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
            #[cfg(feature = "fmt")]
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
            #[cfg(feature = "fmt")]
            println!("{}", result);
        }
    }

    #[test]
    fn test_query_soa() {
        #[cfg(feature = "logger")]
        init_logger();
        let server = vec!["9.9.9.9".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_soa("www.baidu.com".to_string());
        if let Some(answer) = result.get_soa_record() {
            #[cfg(feature = "fmt")]
            println!("{}", answer);
            #[cfg(not(feature = "fmt"))]
            println!("{:?}", result);
        } else {
            println!("No SOA record");
            #[cfg(feature = "fmt")]
            println!("{}", result);
        }
    }

    #[test]
    #[cfg(feature = "fmt")]
    fn test_fmt() {
        #[cfg(feature = "logger")]
        init_logger();
        let server = vec!["223.5.5.5".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_ns(".".to_string());
        println!("{}", result);
    }
}
