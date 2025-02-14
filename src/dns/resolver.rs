#![cfg_attr(debug_assertions, allow(unused_variables, dead_code))]

use crate::dns::error::ResultAndError;
#[cfg(feature = "result_error")]
use crate::dns::error::{ErrorFormat, NetError, error_trait};
use crate::dns::net::NetQuery;
#[cfg(feature = "result_error")]
use crate::dns::net::NetQueryError;
use crate::dns::types::base::{DnsTypeNum, RawDomain};
use crate::dns::types::parts::{RecordDataType, Request, Response};
use crate::dns::utils::ServerType;
#[cfg(feature = "logger")]
use log::debug;
use paste::paste;
use smallvec::SmallVec;
#[cfg(feature = "result_error")]
use std::fmt::{Debug, Display};
use std::iter::FilterMap;

use std::slice::Iter;

pub struct Resolver {
    server: SmallVec<[ServerType; 5]>,
}

pub struct ResolveConfig {
    pub server: Vec<String>,
    pub target: String,
}

#[cfg(feature = "result_error")]
pub enum ResolverQueryError {
    TargetParseError(ErrorFormat),
    NetError(Vec<NetError>),
}

#[cfg(feature = "result_error")]
impl Display for ResolverQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverQueryError::TargetParseError(err) => std::fmt::Display::fmt(err, f),
            ResolverQueryError::NetError(errs) => std::fmt::Display::fmt(
                &ErrorFormat::from_vec(errs.iter().map(|x| x.get_index()).collect()),
                f,
            ),
        }
    }
}

#[cfg(feature = "result_error")]
impl Debug for ResolverQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverQueryError::TargetParseError(err) => std::fmt::Debug::fmt(err, f),
            ResolverQueryError::NetError(errs) => std::fmt::Debug::fmt(
                &ErrorFormat::from_vec(errs.iter().map(|x| x.get_index()).collect()),
                f,
            ),
        }
    }
}

// 没什么营养的东西
#[cfg(feature = "result_error")]
fn convert_err(value: NetQueryError, trace: &str) -> NetError {
    match value {
        NetQueryError::ConnectTcpAddrError(mut e) => {
            e.add_trace(trace);
            NetError::ConnectTcpAddrError(e)
        }
        NetQueryError::UdpNotConnected(mut e) => {
            e.add_trace(trace);
            NetError::UdpNotConnected(e)
        }
        NetQueryError::SendUdpPacketError(mut e) => {
            e.add_trace(trace);
            NetError::SendUdpPacketError(e)
        }
        NetQueryError::RecvUdpPacketError(mut e) => {
            e.add_trace(trace);
            NetError::RecvUdpPacketError(e)
        }
        NetQueryError::RecvTcpPacketError(mut e) => {
            e.add_trace(trace);
            NetError::RecvTcpPacketError(e)
        }
        NetQueryError::WriteTcpConnectError(mut e) => {
            e.add_trace(trace);
            NetError::WriteTcpConnectError(e)
        }
    }
}
#[cfg(feature = "result_error")]
impl error_trait::B for ResolverQueryError {}

impl Resolver {
    pub fn new(server: &mut Vec<String>) -> Result<Resolver, std::net::AddrParseError> {
        let vec = server
            .iter_mut()
            .try_fold(SmallVec::new(), |mut vec, str| {
                vec.push(ServerType::from_string(str)?);
                Ok(vec)
            })?;
        Ok(Resolver { server: vec })
    }

    pub fn query(&self, domain: String, qtype: u16) -> ResolverQueryResult {
        #[cfg(feature = "result_error")]
        let mut error_vec = Vec::new();
        if let Some(domain) = RawDomain::from_str(domain.as_str()) {
            let domain = std::rc::Rc::new(domain);
            let mut buf = [0_u8; 1500];
            for server in &self.server {
                return match server {
                    ServerType::Tcp(addr) => {
                        //后面可以考虑复用连接
                        match std::net::TcpStream::connect(addr) {
                            Ok(stream) => {
                                let request = Request::new(domain.clone(), qtype);
                                #[cfg(feature = "result_error")]
                                match NetQuery::query_tcp(stream, request, &mut buf).into_index() {
                                    Ok(response) => response.into(),
                                    Err(e) => {
                                        error_vec.push(convert_err(
                                            e,
                                            "Resolver::query => ServerType::Tcp",
                                        ));
                                        continue;
                                    }
                                }
                                #[cfg(not(feature = "result_error"))]
                                ResolverQueryResult::from(NetQuery::query_tcp(
                                    stream, request, &mut buf,
                                ))
                            }
                            Err(err) => {
                                #[cfg(feature = "logger")]
                                debug!("连接到对应的tcp server失败");
                                #[cfg(feature = "result_error")]
                                error_vec.push(NetError::ConnectTcpAddrError(ErrorFormat::new(
                                    format!("ConnectTcpAddrError, target {}, {}", addr, err),
                                    "Resolver::query => ServerType::Tcp",
                                )));
                                continue; //连接到server失败, 则尝试备用server
                            }
                        }
                    }
                    ServerType::Udp(addr) => {
                        match std::net::UdpSocket::bind("0.0.0.0:0") {
                            Ok(socket) => match socket.connect(addr) {
                                Ok(stream) => {
                                    let request = Request::new(domain.clone(), qtype);
                                    #[cfg(feature = "result_error")]
                                    match NetQuery::query_udp(socket, request, &mut buf)
                                        .into_index()
                                    {
                                        Ok(response) => response.into(),
                                        Err(e) => {
                                            error_vec.push(convert_err(
                                                e,
                                                "Resolver::query => ServerType::Udp",
                                            ));
                                            continue;
                                        }
                                    }
                                    #[cfg(not(feature = "result_error"))]
                                    ResolverQueryResult::from(NetQuery::query_udp(
                                        socket, request, &mut buf,
                                    ))
                                }
                                Err(err) => {
                                    #[cfg(feature = "logger")]
                                    debug!("连接到对应的udp server失败");
                                    #[cfg(feature = "result_error")]
                                    error_vec.push(NetError::ConnectUdpAddrError(
                                        ErrorFormat::new(
                                            format!(
                                                "ConnectTcpAddrError, target {}, {}",
                                                addr, err
                                            ),
                                            "Resolver::query => ServerType::Udp",
                                        ),
                                    ));
                                    continue;
                                }
                            },
                            Err(err) => {
                                #[cfg(feature = "logger")]
                                debug!("监听udp端口失败");
                                #[cfg(feature = "result_error")]
                                error_vec.push(NetError::BindUdpAddrError(ErrorFormat::new(
                                    format!("BindUdpAddrError, target {}, {}", addr, err),
                                    "Resolver::query => ServerType::Udp",
                                )));
                                continue; //监听udp失败，尝试备用
                            }
                        }
                    }
                };
            }
            #[cfg(feature = "result_error")]
            return ResolverQueryError::NetError(error_vec).into();
            #[cfg(not(feature = "result_error"))]
            ResolverQueryResult::from(None)
        } else {
            #[cfg(feature = "result_error")]
            return ResolverQueryError::TargetParseError(ErrorFormat::new(
                format!("TargetParseError, domain: {}", domain),
                "Resolver::query",
            ))
            .into();
            #[cfg(not(feature = "result_error"))]
            ResolverQueryResult::from(None)
        }
    }
}

#[cfg(feature = "result_error")]
#[derive(Debug)]
pub struct ResolverQueryResult(pub ResultAndError<Response, ResolverQueryError>);

impl ResolverQueryResult {
    #[inline]
    pub fn get_result(&self) -> Option<&Response> {
        self.0.get_result()
    }

    #[inline]
    pub fn into_result(self) -> Option<Response> {
        self.0.into_result()
    }
}

#[cfg(not(feature = "result_error"))]
#[derive(Debug)]
pub struct ResolverQueryResult(pub ResultAndError<Response>);

#[macro_export]
macro_rules! query_type_map {
    (A) => { std::net::Ipv4Addr };
    (NS) => { std::string::String };
    (CNAME) => { std::string::String };
    (SOA) => { $crate::dns::types::base::record::SOA };
    (TXT) => { Vec<String> };
    (AAAA) => { std::net::Ipv6Addr }
}

// todo
#[macro_export]
macro_rules! query_result_map {
    (single,$query_type:ty) => {Option<$query_type>};
    (all,$query_type:ty) => {Vec<$query_type>};
    (iter,$query_type:ty) => {
        Option<FilterMap<Iter<crate::dns::types::parts::Record>,
            fn(&crate::dns::types::parts::Record) -> Option<$query_type>>>
    };
    (into_iter,$query_type:ty) => {
        Option<std::iter::FilterMap<std::vec::IntoIter<$crate::dns::types::parts::Record>,
            fn($crate::dns::types::parts::Record) -> Option<$query_type>>>
    };
}

#[macro_export]
macro_rules! query_result_map_err {
    (single,$query_type:ty) => {$crate::dns::resolver::QueryResult<$query_type>};
    (all,$query_type:ty) => {$crate::dns::resolver::QueryResult<Vec<$query_type>>};
    (into_iter,$query_type:ty) => {
        $crate::dns::resolver::QueryResult<std::iter::FilterMap<std::vec::IntoIter<$crate::dns::types::parts::Record>,
            fn($crate::dns::types::parts::Record) -> Option<$query_type>>>
    };
}

#[macro_export]
macro_rules! record_filter {
    ($record_type:ident) => {
        |rec| {
            if let $crate::dns::types::parts::RecordDataType::$record_type(v) = rec.data {
                Some(v.get_general_output()?)
            } else {
                None
            }
        }
    };
}

//我真不想写了，用宏生成算了
macro_rules! define_get_record {
    ($fn_name:ident, $dns_type:expr) => {
        paste! {
            impl ResolverQueryResult {
                #[inline]
                pub fn [<get_ $fn_name _record>](&self) -> query_result_map!(single,query_type_map!($dns_type)) {
                    let response = self.0.get_result()?;
                    response.answer.iter().find_map(|rec| {
                        if let RecordDataType::$dns_type(v) = &rec.data {
                            Some(v.get_general_output()?)
                        } else {
                            None
                        }
                    })
                }

                #[inline]
                pub fn [<get_ $fn_name _record_iter>](&self) ->
                        query_result_map!(iter,query_type_map!($dns_type))  {
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
                pub fn [<query_ $fn_name>](&self, domain: String) -> ResolverQueryResult {
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
        || -> query_result_map!(single,query_type_map!($record_type)) {
            let mut config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            let resolver = $crate::dns::resolver::Resolver::new(&mut config.server).ok()?;
            let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type);
            let response = result.into_result()?;
            response.answer.into_iter().find_map(record_filter!($record_type))
        }()
    };
    ($record_type:ident,all,$(@$config:ident $server:expr),*) => {
        || -> query_result_map!(all,query_type_map!($record_type)) {
            let mut config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            if let Ok(resolver) = $crate::dns::resolver::Resolver::new(&mut config.server) {
                let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type);
                if let Some(res) = result.into_result() {
                    res.answer.into_iter().filter_map(record_filter!($record_type)).collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }

        }()
    };
    ($record_type:ident,into_iter,$(@$config:ident $server:expr),*) => {
        || -> query_result_map!(into_iter,query_type_map!($record_type)) {
            let mut config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            let resolver = $crate::dns::resolver::Resolver::new(&mut config.server).ok()?;
            let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type);
            let res = result.into_result()?;
            Some(res.answer.into_iter().filter_map(record_filter!($record_type)))
        }()
    };



    ($record_type:ident,$(@$config:ident $server:expr),*,-feature error) => {
        || -> query_result_map_err!(single,query_type_map!($record_type)) {
            let mut config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            match $crate::dns::resolver::Resolver::new(&mut config.server) {
                Ok(resolver) => {
                    let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type).0.into_index();
                    match result {
                        Ok(response) => match response {
                            Some(res) => $crate::dns::resolver::QueryResult::from_result(res.answer.into_iter().find_map(record_filter!($record_type))),
                            None => $crate::dns::resolver::QueryResult::from_result(None),
                        },
                        Err(e) => $crate::dns::resolver::QueryError::from(e).into(),
                    }
                }
                Err(err) => $crate::dns::resolver::QueryError::ServerParseError($crate::dns::error::ErrorFormat::new(
                    format!("ServerParseError, target {:?}, {}", config.server, err),
                    "query!()"
                )).into()
            }
        }()
    };
    ($record_type:ident,all,$(@$config:ident $server:expr),*,-feature error) => {
        || -> query_result_map_err!(all,query_type_map!($record_type)) {
            let mut config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            match $crate::dns::resolver::Resolver::new(&mut config.server) {
                Ok(resolver) => {
                    let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type).0.into_index();
                    match result {
                        Ok(response) => match response {
                            Some(res) => $crate::dns::resolver::QueryResult::from_result(Some(
                                res.answer.into_iter().filter_map(record_filter!($record_type)).collect()
                            )),
                            None => $crate::dns::resolver::QueryResult::from_result(None),
                        },
                        Err(e) => $crate::dns::resolver::QueryError::from(e).into(),
                    }
                }
                Err(err) => $crate::dns::resolver::QueryError::ServerParseError($crate::dns::error::ErrorFormat::new(
                    format!("ServerParseError, target {:?}, {}", config.server, err),
                    "query!()"
                )).into()
            }
        }()
    };
    ($record_type:ident,into_iter,$(@$config:ident $server:expr),*,-feature error) => {
        || -> query_result_map_err!(into_iter,query_type_map!($record_type)) {
            let mut config = $crate::dns::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            match $crate::dns::resolver::Resolver::new(&mut config.server) {
                Ok(resolver) => {
                    let result = resolver.query(config.target,$crate::dns::types::base::DnsTypeNum::$record_type).0.into_index();
                    match result {
                        Ok(response) => match response {
                            Some(res) => $crate::dns::resolver::QueryResult::from_result(Some(
                                res.answer.into_iter().filter_map(record_filter!($record_type))
                            )),
                            None => $crate::dns::resolver::QueryResult::from_result(None),
                        },
                        Err(e) => $crate::dns::resolver::QueryError::from(e).into(),
                    }
                }
                Err(err) => $crate::dns::resolver::QueryError::ServerParseError($crate::dns::error::ErrorFormat::new(
                    format!("ServerParseError, target {:?}, {}", config.server, err),
                    "query!()"
                )).into()
            }
        }()
    }
}

#[cfg(feature = "result_error")]
pub type QueryResult<T> = ResultAndError<T, QueryError>;

#[cfg(not(feature = "result_error"))]
pub type QueryResult<T> = ResultAndError<T>;

#[cfg(feature = "result_error")]
pub enum QueryError {
    ServerParseError(ErrorFormat),
    TargetParseError(ErrorFormat),
    ResolverQueryError(ErrorFormat),
}

#[cfg(feature = "result_error")]
impl Display for QueryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            QueryError::ServerParseError(e)
            | QueryError::TargetParseError(e)
            | QueryError::ResolverQueryError(e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

#[cfg(feature = "result_error")]
impl Debug for QueryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            QueryError::ServerParseError(e)
            | QueryError::TargetParseError(e)
            | QueryError::ResolverQueryError(e) => std::fmt::Debug::fmt(&e, f),
        }
    }
}

#[cfg(feature = "result_error")]
impl error_trait::B for QueryError {}
#[cfg(feature = "result_error")]
impl From<ResolverQueryError> for QueryError {
    fn from(err: ResolverQueryError) -> Self {
        match err {
            ResolverQueryError::TargetParseError(err) => QueryError::TargetParseError(err),
            ResolverQueryError::NetError(err) => QueryError::ResolverQueryError(
                ErrorFormat::from_vec(err.iter().map(|x| x.get_index()).collect())
                    .add_trace_into("query!()"),
            ),
        }
    }
}

#[cfg(feature = "fmt")]
impl Display for ResolverQueryResult {
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

impl From<Option<Response>> for ResolverQueryResult {
    fn from(value: Option<Response>) -> ResolverQueryResult {
        ResolverQueryResult(ResultAndError::from_result(value))
    }
}

#[cfg(feature = "result_error")]
impl From<ResolverQueryError> for ResolverQueryResult {
    fn from(value: ResolverQueryError) -> Self {
        ResolverQueryResult(ResultAndError::from_error(value))
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
        let mut server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(&mut server).unwrap();
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
        let mut server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(&mut server).unwrap();
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
        let mut server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(&mut server).unwrap();
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
        let mut server = vec!["94.140.14.140".to_string()];
        let resolver = Resolver::new(&mut server).unwrap();
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
        let mut server = vec!["9.9.9.9".to_string()];
        let resolver = Resolver::new(&mut server).unwrap();
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
        let mut server = vec!["223.5.5.5".to_string()];
        let resolver = Resolver::new(&mut server).unwrap();
        let result = resolver.query_txt("fs.gloryouth.com".to_string());
        println!("{}", result);
    }

    #[test]
    fn test_special() {
        #[cfg(feature = "logger")]
        init_logger();
        let mut server = vec!["9.9.9.9".to_string()];
        let resolver = Resolver::new(&mut server).unwrap();
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
    #[cfg(feature = "result_error")]
    fn test_query() {
        let server = vec!["9.9.9.9".to_string()];
        let result = query! {
            A,
            @target "www.baidu.com".to_string(),
            @server server,
            -feature error
        };
        println!("{:?}", result.into_result().unwrap());
    }
}
