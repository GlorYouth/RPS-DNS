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
use std::iter::FilterMap;
use std::net::{AddrParseError, TcpStream, UdpSocket};
use std::rc::Rc;
use std::slice::Iter;

pub struct Resolver {
    server: SmallVec<[ServerType; 5]>,
}


pub struct ResolveConfig {
    pub server: Vec<String>,
    pub target: String,
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
            let mut buf = [0_u8; 1500];
            for server in &self.server {
                return match server {
                    ServerType::Tcp(addr) => {
                        //后面可以考虑复用连接
                        if let Ok(stream) = TcpStream::connect(addr) {
                            let request = Request::new(domain.clone(), qtype);
                            #[cfg(feature = "result_error")]
                            match NetQuery::query_tcp(stream, request, &mut buf) {
                                Ok(response) => response.into(),
                                Err(e) => {
                                    error_vec.push(e.into());
                                    continue;
                                }
                            }
                            #[cfg(not(feature = "result_error"))]
                            QueryResult::from(NetQuery::query_tcp(stream, request, &mut buf))
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
                                match NetQuery::query_udp(socket, request, &mut buf) {
                                    Ok(response) => response.into(),
                                    Err(e) => {
                                        #[cfg(feature = "result_error")]
                                        error_vec.push(e.into());
                                        continue;
                                    }
                                }
                                #[cfg(not(feature = "result_error"))]
                                QueryResult::from(NetQuery::query_udp(socket, request, &mut buf))
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

impl QueryResult {
    #[inline]
    pub fn get_result(&self) -> &Option<Response> {
        self.0.get_result()
    }

    #[inline]
    pub fn into_result(self) -> Option<Response> {
        
        self.0.into_result()
    }
}

#[cfg(not(feature = "result_error"))]
#[derive(Debug)]
pub struct QueryResult(ErrorAndOption<Response>);

macro_rules! query_result_map {
    (A) => { std::net::Ipv4Addr };
    (NS) => { std::string::String };
    (CNAME) => { std::string::String };
    (SOA) => { $crate::dns::types::base::record::SOA };
    (TXT) => { Vec<String> };
    (AAAA) => { std::net::Ipv6Addr }
}

// todo

//我真不想写了，用宏生成算了
macro_rules! define_get_record {
    ($fn_name:ident, $dns_type:expr) => {
        paste! {
            impl QueryResult {
                #[inline]
                pub fn [<get_ $fn_name _record>](&self) -> Option<query_result_map!($dns_type)> {
                    let response = self.0.get_result().as_ref()?;
                    response.answer.iter().find_map(|rec| {
                        if let RecordDataType::$dns_type(v) = &rec.data {
                            Some(v.get_general_output()?)
                        } else {
                            None
                        }
                    })
                }

                #[inline]
                pub fn [<get_ $fn_name _record_iter>](&self) -> Option<FilterMap<Iter<crate::dns::types::parts::Record>, fn(&crate::dns::types::parts::Record) -> Option<query_result_map!($dns_type)>>>  {
                    if let Some(res) = self.0.get_result() {
                        Some(res.answer.iter().filter_map(|rec| {
                            if let RecordDataType::$dns_type(v) = &rec.data {
                                Some(v.get_general_output()?)
                            } else {
                                None
                            }})
                        )
                    } else {
                        None
                    }
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
define_get_record!(a, A);
define_get_record!(ns, NS);
define_get_record!(cname, CNAME);
define_get_record!(soa, SOA);
define_get_record!(txt, TXT);
define_get_record!(aaaa, AAAA);
// todo




#[macro_export]
macro_rules! query {
    ($record_type:ident,$(@$config:ident $server:expr),*) => {
        || -> Option<query_result_map!($record_type)> {
            let config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            let resolver = $crate::dns::resolver::Resolver::new(config.server).ok()?;
            let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type);
            let response = result.get_result().as_ref()?;
            response.answer.iter().find_map(|rec| {
                if let $crate::dns::types::parts::RecordDataType::$record_type(v) = &rec.data {
                    Some(v.get_general_output()?)
                } else {
                    None
                }
            })
        }()
    };
    ($record_type:ident,all,$(@$config:ident $server:expr),*) => {
        || -> Vec<query_result_map!($record_type)> {
            let config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            if let Ok(resolver) = $crate::dns::resolver::Resolver::new(config.server) {
                let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type);
                if let Some(res) = result.into_result() {
                    res.answer.into_iter().filter_map(|rec| { 
                        if let $crate::dns::types::parts::RecordDataType::$record_type(v) = rec.data {
                            Some(v.get_general_output()?)
                        } else {
                            None
                        }
                    }).collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }

        }()
    };
    ($record_type:ident,into_iter,$(@$config:ident $server:expr),*) => {
        || -> Option<std::iter::FilterMap<std::vec::IntoIter<$crate::dns::types::parts::Record>, fn($crate::dns::types::parts::Record) -> Option<query_result_map!($record_type)>>> {
            let config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            let resolver = $crate::dns::resolver::Resolver::new(config.server).ok()?;
            let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type);
            let res = result.into_result()?;
            Some(res.answer.into_iter().filter_map(|rec| {
                if let $crate::dns::types::parts::RecordDataType::$record_type(v) = rec.data {
                    Some(v.get_general_output()?)
                } else {
                    None
                }
            }))
        }()
    };
}

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
    fn from(value: Option<Response>) -> QueryResult {
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
    #[cfg(feature = "logger")]
    use crate::dns::error::set_println_enabled;
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
    fn test_query_txt() {
        #[cfg(feature = "logger")]
        init_logger();
        let server = vec!["9.9.9.9".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_txt("fs.gloryouth.com".to_string());
        if let Some(answer) = result.get_txt_record() {
            #[cfg(feature = "fmt")]
            println!("{:?}", answer);
            #[cfg(not(feature = "fmt"))]
            println!("{:?}", result);
        } else {
            println!("No TXT record");
            #[cfg(feature = "fmt")]
            println!("{}", result);
        }
    }

    #[test]
    #[cfg(feature = "fmt")]
    fn test_fmt() {
        #[cfg(feature = "logger")]
        init_logger();
        #[cfg(feature = "logger")]
        set_println_enabled(true);
        let server = vec!["223.5.5.5".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_txt("fs.gloryouth.com".to_string());
        println!("{}", result);
    }

    #[test]
    fn test_special() {
        #[cfg(feature = "logger")]
        init_logger();
        let server = vec!["9.9.9.9".to_string()];
        let resolver = Resolver::new(server).unwrap();
        let result = resolver.query_txt("gloryouth.com".to_string());
        println!(
            "{:?}",
            result
                .get_txt_record_iter()
                .unwrap()
                .flatten()
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_query() {
        let server = vec!["9.9.9.9".to_string()];
        let result = query! {
            A,
            into_iter,
            @target "www.baidu.com".to_string(),
            @server server
        };
        println!("{:?}", result.unwrap().next());
    }
}
