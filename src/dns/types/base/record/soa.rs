#[cfg(feature = "fmt")]
use crate::dns::types::base::DnsTTL;
use crate::dns::types::base::RawDomain;
use crate::dns::utils::SliceReader;
#[cfg(feature = "fmt")]
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct SOA {
    primary_name: Rc<RawDomain>,
    //名称服务器的 <domain-name>，该名称服务器是这个区域的数据起源或主要源。
    rname: Rc<RawDomain>,
    //一个<domain-name>，它规定负责这个区域的个人的邮箱。
    serial_number: u32,
    // 该区域的原始副本的无符号 32 位版本号。区域传递保存这个值。这个值叠起(wrap)，并且应当使用系列空间算法比较这个值。
    refresh_interval: u32,
    // REFRESH 区域应当被刷新前的 32 位时间间隔
    retry_interval: u32,
    // 在重试失败的刷新前，应当等待的 32 位时间间隔
    expire_limit: u32,
    // 32 位时间值，它规定在区域不再是权威的之前可以等待的时间间隔的上限
    minimum_ttl: u32,
    // 无符号 32 位最小值 TTL 字段，应当用来自这个区域的任何 RR 输出它。
}

impl SOA {
    pub fn from_reader_with_size(reader: &mut SliceReader, _raw_len: usize) -> Option<Self> {
        let primary_name = Rc::new(RawDomain::from_reader(reader)?);
        let rname = Rc::new(RawDomain::from_reader(reader)?);
        Some(Self {
            primary_name,
            rname,
            serial_number: reader.read_u32(),
            refresh_interval: reader.read_u32(),
            retry_interval: reader.read_u32(),
            expire_limit: reader.read_u32(),
            minimum_ttl: reader.read_u32(),
        })
    }

    #[cfg(feature = "fmt")]
    pub fn fmt_with_suffix(&self, f: &mut Formatter, _indent: &str) -> std::fmt::Result {
        macro_rules! write_field {
            ($label:expr, $($arg:expr),*) => {
                write!(f, "{_indent}")?;
                writeln!(f, $label, $($arg),*)?;
            };
        }
        writeln!(f, "SOA: ")?;

        write_field!("\tPrimary name server: {}", self.primary_name);
        write_field!("\tResponsible authority's mailbox: {}", self.rname);
        write_field!("\tSerial number: {}", self.serial_number);
        write_field!(
            "\tRefresh interval: {} ({})",
            self.refresh_interval,
            DnsTTL::get_str(self.refresh_interval)
        );
        write_field!(
            "\tRetry interval: {} ({})",
            self.retry_interval,
            DnsTTL::get_str(self.retry_interval)
        );
        write_field!(
            "\tExpire limit: {} ({})",
            self.expire_limit,
            DnsTTL::get_str(self.expire_limit)
        );
        write_field!(
            "\tMinimum ttl: {} ({})",
            self.minimum_ttl,
            DnsTTL::get_str(self.minimum_ttl)
        );

        Ok(())
    }
}

#[cfg(feature = "fmt")]
impl Display for SOA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_suffix(f, "")
    }
}
