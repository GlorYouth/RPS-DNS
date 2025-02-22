#![cfg_attr(debug_assertions, allow(unused_variables, dead_code))]

use crate::dns::error::ResultAndError;
#[cfg(feature = "result_error")]
use crate::dns::error::{ErrorFormat, NetError};
use crate::dns::net::NetQuery;
#[cfg(feature = "result_error")]
use crate::dns::net::NetQueryError;
use crate::dns::types::base::{DnsTypeNum, RawDomain};
use crate::dns::types::parts::{RecordDataType, Request, Response};
use crate::dns::utils::{RefWrapper, ServerType};
#[cfg(feature = "logger")]
use log::debug;
use paste::paste;
use smallvec::SmallVec;
#[cfg(feature = "fmt")]
use std::fmt::{Debug, Display};

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
        let domain = match RawDomain::from_str(domain.as_str()) {
            None => {
                #[cfg(feature = "result_error")]
                return ResolverQueryError::TargetParseError(ErrorFormat::new(
                    format!("TargetParseError, domain: {}", domain),
                    "Resolver::query",
                ))
                .into();
                #[cfg(not(feature = "result_error"))]
                ResolverQueryResult::from(None)
            }
            Some(domain) => domain,
        };
        #[cfg(feature = "result_error")]
        let mut error_vec = Vec::new();

        let domain = std::rc::Rc::new(domain);
        let mut buf = [0_u8; 1500];
        for server in &self.server {
            let res = match server {
                ServerType::Tcp(addr) => std::net::TcpStream::connect(addr)
                    .map_err(|err| {
                        #[cfg(feature = "logger")]
                        debug!("连接到对应的tcp server失败");
                        #[cfg(feature = "result_error")]
                        NetError::ConnectTcpAddrError(ErrorFormat::new(
                            format!("ConnectTcpAddrError, target {}, {}", addr, err),
                            "Resolver::query => ServerType::Tcp",
                        ))
                    })
                    .and_then(|stream| {
                        let request = Request::new(domain.clone(), qtype);
                        #[cfg(feature = "result_error")]
                        match NetQuery::query_tcp(stream, request, &mut buf).into_index() {
                            Ok(response) => Ok(response),
                            Err(e) => Err(convert_err(e, "Resolver::query => ServerType::Tcp")),
                        }
                        #[cfg(not(feature = "result_error"))]
                        Ok(NetQuery::query_tcp(stream, request, &mut buf).into())
                    }),
                ServerType::Udp(addr) => std::net::UdpSocket::bind("0.0.0.0:0")
                    .map_err(|err| {
                        #[cfg(feature = "logger")]
                        debug!("监听udp端口失败");
                        #[cfg(feature = "result_error")]
                        NetError::BindUdpAddrError(ErrorFormat::new(
                            format!("BindUdpAddrError, target {}, {}", addr, err),
                            "Resolver::query => ServerType::Udp",
                        ))
                    })
                    .and_then(|socket| match socket.connect(addr) {
                        Ok(_) => Ok(socket),
                        Err(err) => {
                            #[cfg(feature = "logger")]
                            debug!("连接到对应的udp server失败");
                            #[cfg(feature = "result_error")]
                            Err(NetError::ConnectUdpAddrError(ErrorFormat::new(
                                format!("ConnectTcpAddrError, target {}, {}", addr, err),
                                "Resolver::query => ServerType::Udp",
                            )))
                        }
                    })
                    .and_then(|socket| {
                        let request = Request::new(domain.clone(), qtype);
                        #[cfg(feature = "result_error")]
                        match NetQuery::query_udp(socket, request, &mut buf).into_index() {
                            Ok(response) => Ok(response),
                            Err(e) => Err(convert_err(e, "Resolver::query => ServerType::Udp")),
                        }
                        #[cfg(not(feature = "result_error"))]
                        Ok(NetQuery::query_udp(socket, request, &mut buf).into())
                    }),
            };
            match res {
                Ok(response) => return response.into(),
                Err(e) => {
                    #[cfg(feature = "result_error")]
                    error_vec.push(e);
                }
            };
        }
        #[cfg(feature = "result_error")]
        return ResolverQueryError::NetError(error_vec).into();
        #[cfg(not(feature = "result_error"))]
        ResolverQueryResult::from(None)
    }
}

#[cfg(feature = "result_error")]
#[derive(Debug)]
pub struct ResolverQueryResult(ResultAndError<Option<Response>, ResolverQueryError>);

//todo 二次封装
impl ResolverQueryResult {
    #[inline]
    pub fn result(&self) -> RefWrapper<Option<Response>> {
        self.0.result()
    }

    #[inline]
    pub fn into_result(self) -> Option<Response> {
        self.0.into_result()
    }

    #[inline]
    #[cfg(feature = "result_error")]
    pub fn into_index(self) -> Result<Option<Response>, ResolverQueryError> {
        self.0.into_index()
    }

    #[inline]
    #[cfg(not(feature = "result_error"))]
    pub fn into_index(self) -> Option<Response> {
        self.0.into_index()
    }

    #[inline]
    #[cfg(feature = "result_error")]
    pub fn error(&self) -> Option<&ResolverQueryError> {
        self.0.error()
    }

    #[inline]
    #[cfg(feature = "result_error")]
    pub fn into_error(self) -> Option<ResolverQueryError> {
        self.0.into_error()
    }
}

#[cfg(not(feature = "result_error"))]
#[derive(Debug)]
pub struct ResolverQueryResult(ResultAndError<Option<Response>>);

#[macro_export]
macro_rules! query_type_map {
    (a) => { std::net::Ipv4Addr };
    (ns) => { std::string::String };
    (cname) => { std::string::String };
    (soa) => { $crate::dns::types::base::record::SOA };
    (txt) => { Vec<String> };
    (aaaa) => { std::net::Ipv6Addr }
}

#[macro_export]
macro_rules! dns_type_num {
    (a) => {
        1
    };
    (ns) => {
        2
    };
    (cname) => {
        5
    };
    (soa) => {
        6
    };
    (txt) => {
        16
    };
    (aaaa) => {
        28
    };
}

// todo
#[macro_export]
macro_rules! query_result_map {
    (single,$query_type:ty) => {Option<$query_type>};
    (all,$query_type:ty) => {Vec<$query_type>};
    (iter,$query_type:ty) => {
        Option<$crate::resolver::Iter<$query_type>>
    };
    (into_iter,$query_type:ty) => {Option<$crate::resolver::IntoIter<$query_type>>};
}

#[macro_export]
macro_rules! query_result_map_err {
    (single,$query_type:ty) => {rps_dns::resolver::QueryResult<Option<$query_type>>};
    (all,$query_type:ty) => {rps_dns::resolver::QueryResult<Vec<$query_type>>};
    (into_iter,$query_type:ty) => {rps_dns::resolver::QueryResult<Option<IntoIter<$query_type>>>};
}

pub type Iter<'a, T> = std::iter::FilterMap<
    std::slice::Iter<'a, crate::dns::types::parts::Record>,
    fn(&crate::dns::types::parts::Record) -> Option<T>,
>;
pub type IntoIter<T> = std::iter::FilterMap<
    std::vec::IntoIter<crate::dns::types::parts::Record>,
    fn(crate::dns::types::parts::Record) -> Option<T>,
>;

//我真不想写了，用宏生成算了
macro_rules! define_get_record {
    ($fn_name:ident, $dns_type:ident) => {
        paste! {
            impl ResolverQueryResult {
                #[inline]
                pub fn $fn_name(&self) -> query_result_map!(single,query_type_map!($fn_name)) {
                    self.0.result().as_ref().as_ref()?.answers().iter().find_map(|rec| {
                        if let RecordDataType::$dns_type(v) = &rec.data {
                            Some(v.get_general_output()?)
                        } else {
                            None
                        }
                    })
                }

                #[inline]
                pub fn [<$fn_name _iter>](&self) ->
                        query_result_map!(iter,query_type_map!($fn_name))  {
                    Some(self.0.result().into_ref()?.as_ref()?.answers().iter().filter_map(|rec| {
                        if let RecordDataType::$dns_type(v) = &rec.data {
                            Some(v.get_general_output()?)
                        } else {
                            None
                        }})
                    )
                }

                #[inline]
                pub fn [<$fn_name _into_iter>](self) ->
                        query_result_map!(into_iter,query_type_map!($fn_name))  {
                    self.0.into_result().map(|res| {
                        res.into_answers().into_iter().filter_map(
                        (|rec: $crate::dns::types::parts::Record| { // 闭包套闭包需要显式转换闭包类型和声明闭包参数的类型
                            if let RecordDataType::$dns_type(v) = rec.data {
                                Some(v.get_general_output()?)
                            } else {
                                None
                            }
                        }) as _)
                    })
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
        || -> $crate::query_result_map!(single,$crate::query_type_map!($record_type)) {
            let mut config = $crate::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            let resolver = $crate::resolver::Resolver::new(&mut config.server).ok()?;
            let result = resolver.query(config.target,$crate::dns_type_num!($record_type));
            result.$record_type()
        }()
    };
    ($record_type:ident,all,$(@$config:ident $server:expr),*) => {
        || -> $crate::query_result_map!(all,$crate::query_type_map!($record_type)) {
            query!{
                $record_type,
                into_iter,
                $(@$config $server),*
            }.map_or(Vec::new(), |iter| iter.collect())
        }()
    };
    ($record_type:ident,into_iter,$(@$config:ident $server:expr),*) => {
        $crate::paste!{
            || -> $crate::query_result_map!(into_iter,$crate::query_type_map!($record_type)) {
                let mut config = $crate::resolver::ResolveConfig {
                    $(
                        $config: $server,
                    )*
                };
                let resolver = $crate::resolver::Resolver::new(&mut config.server).ok()?;
                let result = resolver.query(config.target,$crate::dns_type_num!($record_type));
                result.[<$record_type _into_iter>]()
            }()
        }
    };



    ($record_type:ident,$(@$config:ident $server:expr),*,-error) => {
        || -> $crate::query_result_map_err!(single,$crate::query_type_map!($record_type)) {
            let mut config = $crate::resolver::ResolveConfig {
                $(
                    $config: $server,
                )*
            };
            $crate::resolver::Resolver::new(&mut config.server).map_err(|err| {  // 先将错误类型提前转过来
                $crate::resolver::QueryError::ServerParseError($crate::error::ErrorFormat::new(
                    format!("ServerParseError, target {:?}, {}", config.server, err),
                    "query!()"
                ))
            }).map(|resolver| { // 获取的值进一步处理，这里是query后获得结果
                resolver.query(config.target,$crate::dns_type_num!($record_type))
            }).and_then(|result| {
                match result.error() { // 判断是否有err, 有则转换err类型, 无则获取第一个结果
                    Some(_) => Err($crate::resolver::QueryError::from(result.into_error().unwrap())),
                    None => Ok(result.$record_type()),
                }
            }).into() // Result<W,E>转换成QueryResult，以后进一步实现的话只需要修改Into就行了
        }()
    };
    /*
    is equal to:
    || -> QueryResult<Option<std::net::Ipv4Addr>> {
        Resolver::new(&mut vec![]).map_err(|err| {
            QueryError::ServerParseError(ErrorFormat::new(
                format!("ServerParseError, target {:?}, {}", "", err),
                "query!()"
            ))
        }).map(|resolver| {
            resolver.query("".into(),0)
        }).and_then(|result| {
            match result.error() {
                None => Ok(result.a()),
                Some(_) => Err(QueryError::from(result.into_error().unwrap()))
            }
        }).into()
    }();
     */

    ($record_type:ident,all,$(@$config:ident $server:expr),*,-error) => { //合并同类项，方便代码维护
        || -> $crate::query_result_map_err!(all,$crate::query_type_map!($record_type)) {
            $crate::query!{
                $record_type,
                into_iter,
                $(@$config $server),*,
                -error
            }.into_index().map(|result| { // Result<W,E>的E部分不变，只动W就行了
                match result { // 若有结果则直接collect，若无则返回空数组
                    None => Vec::new(),
                    Some(iter) => iter.collect()
                }
            }).into()
        }()
    };
    /*
    is equal to:
    || -> QueryResult<Vec<std::net::Ipv4Addr>> {
        Resolver::new(&mut vec![]).map_err(|err| {
            QueryError::ServerParseError(ErrorFormat::new(
                format!("ServerParseError, target {:?}, {}", "", err),
                "query!()"
            ))
        }).map(|resolver| {
            resolver.query("".into(),0)
        }).and_then(|result| {
            match result.error() {
                Some(_) => Err(QueryError::from(result.into_error().unwrap())),
                None => Ok(result.a_into_iter())
            }
        }).map(|result| {
            match result {
                None => Vec::new(),
                Some(iter) => iter.collect()
            }
        }).into()
    }();
     */
    ($record_type:ident,into_iter,$(@$config:ident $server:expr),*,-error) => {
        $crate::paste!{
            || -> $crate::query_result_map_err!(into_iter,$crate::query_type_map!($record_type)) {
                let mut config = $crate::resolver::ResolveConfig {
                    $(
                        $config: $server,
                    )*
                };
                $crate::resolver::Resolver::new(&mut config.server).map_err(|err| { // 也是先将错误类型提前转过来
                    $crate::resolver::QueryError::ServerParseError($crate::error::ErrorFormat::new(
                        format!("ServerParseError, target {:?}, {}", config.server, err),
                        "query!()"
                    ))
                }).map(|resolver| { // 获取的值进一步处理，这里是query后获得结果
                    resolver.query(config.target,$crate::dns_type_num!($record_type))
                }).and_then(|result| {
                    match result.error() { // 判断是否有err, 有则转换err类型, 无则获取结果的指针
                        Some(_) => Err($crate::resolver::QueryError::from(result.into_error().unwrap())),
                        None => Ok(result.[<$record_type _into_iter>]())
                    }
                }).into() // Result<W,E>转换成QueryResult，以后进一步实现的话只需要修改Into就行了
            }()
        }
    }
    /*
    is equal to:
    || -> QueryResult<Option<IntoIter<std::net::Ipv4Addr>>> {
        Resolver::new(&mut vec![]).map_err(|err| {
            QueryError::ServerParseError(ErrorFormat::new(
                format!("ServerParseError, target {:?}, {}", "", err),
                "query!()"
            ))
        }).map(|resolver| {
            resolver.query("".into(), 1)
        }).and_then(|result| {
            match result.error()  {
                Some(_) => Err(QueryError::TargetParseError(ErrorFormat::new("".to_string(), ""))),
                None => Ok(result.a_into_iter())
            }
        }).into()
    }();
     */
}

#[cfg(feature = "result_error")]
pub type QueryResult<W> = ResultAndError<W, QueryError>;

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
        if let Some(res) = &self.0.result().as_ref() {
            std::fmt::Display::fmt(&res, f)?;
        } else {
            #[cfg(feature = "result_error")]
            for e in self.0.error().iter() {
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
