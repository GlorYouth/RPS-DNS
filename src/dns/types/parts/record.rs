#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::DnsTypeNum;
use crate::dns::types::base::{DnsType, RawDomain};
use crate::dns::types::parts::{DnsClass, DnsTTL};
use crate::dns::utils::SliceReader;
#[cfg(feature = "logger")]
use log::{debug, trace};
use std::fmt::Display;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::rc::Rc;

#[derive(Debug)]
pub struct Record {
    pub name: RawDomain,
    pub rtype: u16,
    pub class: u16,
    pub ttl: u32,
    pub data: RecordDataType,
}

impl Record {
    #[inline]
    pub fn new(reader: &mut SliceReader) -> Option<Record> {
        #[cfg(feature = "logger")]
        {
            trace!("准备解析Record内的name");
        }
        let name = RawDomain::from_reader(reader)?;
        let len = reader.len();

        if reader.pos() + 10 > len {
            #[cfg(feature = "logger")]
            {
                trace!("解析完name后，剩余Slice不足以存放Record的其余部分");
            }
            return None;
        }
        let rtype = reader.read_u16();
        let class = reader.read_u16();
        let ttl = reader.read_u32();
        let data_length = reader.read_u16() as usize;

        if reader.pos() + data_length > len {
            #[cfg(feature = "logger")]
            debug!(
                "读取到Record中Data可变部分长度为{:x},需要总Slice长度为{:x},实际Slice长度{:x}",
                data_length,
                reader.pos() + data_length,
                len
            );
            return None;
        }

        let data = match rtype {
            DnsTypeNum::CNAME => RawRecordDataType::Domain(Rc::from(
                RawDomain::from_reader_with_size(reader, data_length)?,
            )),
            _ => RawRecordDataType::Other(reader.read_slice(data_length)),
        };

        Some(Record {
            name,
            rtype,
            class,
            ttl,
            data: RecordDataType::new(rtype, &data)?,
        })
    }

    pub fn get_fmt_type(&self) -> RecordFmtType {
        match self.data {
            RecordDataType::A(_) | RecordDataType::AAAA(_) | RecordDataType::CNAME(_) => {
                RecordFmtType::Answers
            }
        }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "\t{}: type ",
            self.name.to_string().unwrap_or("???".to_owned())
        )?;
        Display::fmt(&self.data.get_dns_type(), f)?;
        writeln!(
            f,
            ", Class: {} ({:#06X})",
            DnsClass::get_str(self.class),
            self.class
        )?;

        writeln!(
            f,
            "\t\tName: {}",
            self.name.to_string().unwrap_or("???".to_owned())
        )?;

        #[inline]
        fn write_other(r: &Record, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            writeln!(
                f,
                "\t\tClass: {} ({:#06X})",
                DnsClass::get_str(r.class),
                r.class
            )?;
            writeln!(f, "\t\tTTL: {} ({})", r.ttl, DnsTTL::get_str(r.ttl))?;
            writeln!(f, "\t\tData length: {}", r.data.len())
        }

        match &self.data {
            RecordDataType::A(addr) => {
                writeln!(f, "\t\tType: A ({})", DnsTypeNum::A)?;
                write_other(self, f)?;
                writeln!(f, "\t\tA: {}", addr)
            }
            RecordDataType::AAAA(addr) => {
                writeln!(f, "\t\tType: AAAA ({})", DnsTypeNum::AAAA)?;
                write_other(self, f)?;
                writeln!(f, "\t\tAAAA: {}", addr)
            }
            RecordDataType::CNAME(str) => {
                writeln!(f, "\t\tType: CNAME ({})", DnsTypeNum::CNAME)?;
                write_other(self, f)?;
                writeln!(
                    f,
                    "\t\tCNAME: {}",
                    str.to_string().unwrap_or("???".to_owned())
                )
            }
        }
    }
}

pub enum RecordFmtType {
    Answers,
}

enum RawRecordDataType<'a> {
    Domain(Rc<RawDomain>),
    Other(&'a [u8]),
}

#[derive(Debug, Clone)]
pub enum RecordDataType {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(Rc<RawDomain>),
}

impl RecordDataType {
    fn new(rtype: u16, data: &RawRecordDataType) -> Option<RecordDataType> {
        match (rtype, data) {
            (DnsTypeNum::A, RawRecordDataType::Other(d)) if d.len() >= 4 => {
                Some(RecordDataType::A(Ipv4Addr::new(d[0], d[1], d[2], d[3])))
            }

            (DnsTypeNum::CNAME, RawRecordDataType::Domain(d)) => {
                Some(RecordDataType::CNAME(d.clone()))
            }

            (DnsTypeNum::AAAA, RawRecordDataType::Other(d)) if d.len() >= 16 => {
                Some(RecordDataType::AAAA(Ipv6Addr::from(
                    <&[u8] as TryInto<[u8; 16]>>::try_into(d).ok()?,
                )))
            }

            _ => {
                #[cfg(feature = "logger")]
                debug!("RecordDataType未实现类型或数据格式错误: {}", rtype);
                None
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            RecordDataType::A(_) => 4,
            RecordDataType::AAAA(_) => 16,
            RecordDataType::CNAME(str) => str.raw_len(),
        }
    }

    pub fn get_rtype(&self) -> u16 {
        match self {
            RecordDataType::A(_) => DnsTypeNum::A,
            RecordDataType::AAAA(_) => DnsTypeNum::AAAA,
            RecordDataType::CNAME(_) => DnsTypeNum::CNAME,
        }
    }

    pub fn get_dns_type(&self) -> DnsType {
        match self {
            RecordDataType::A(_) => DnsType::A,
            RecordDataType::AAAA(_) => DnsType::AAAA,
            RecordDataType::CNAME(_) => DnsType::CNAME,
        }
    }
}
