#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::DnsTypeNum;
use crate::dns::RawDomain;
#[cfg(feature = "fmt")]
use crate::dns::types::base::DnsClass;
#[cfg(feature = "fmt")]
use crate::dns::types::base::DnsTTL;
#[cfg(feature = "fmt")]
use crate::dns::types::base::DnsType;
use crate::dns::types::base::SOA;
use crate::dns::utils::SliceReader;
#[cfg(feature = "logger")]
use log::{debug, trace};
#[cfg(feature = "fmt")]
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
            DnsTypeNum::A => RecordDataType::A(Ipv4Addr::from(
                <[u8; 4]>::try_from(reader.read_slice(data_length)).unwrap(),
            )),
            DnsTypeNum::NS => RecordDataType::NS(Rc::from(RawDomain::from_reader_with_size(
                reader,
                data_length,
            )?)),
            DnsTypeNum::CNAME => RecordDataType::CNAME(Rc::from(RawDomain::from_reader_with_size(
                reader,
                data_length,
            )?)),
            DnsTypeNum::SOA => RecordDataType::SOA(SOA::from_reader(reader, data_length)?),
            DnsTypeNum::AAAA => RecordDataType::AAAA(Ipv6Addr::from(
                <[u8; 16]>::try_from(reader.read_slice(data_length)).unwrap(),
            )),
            _ => {
                #[cfg(feature = "logger")]
                trace!("Unsupported Type: {}", rtype);
                return None;
            }
        };

        Some(Record {
            name,
            rtype,
            class,
            ttl,
            data,
        })
    }

    #[cfg(feature = "fmt")]
    pub fn get_fmt_type(&self) -> RecordFmtType {
        match self.data {
            RecordDataType::A(_)
            | RecordDataType::AAAA(_)
            | RecordDataType::CNAME(_)
            | RecordDataType::NS(_) => RecordFmtType::Answers,
            RecordDataType::SOA(_) => RecordFmtType::Authoritative,
        }
    }
}

#[cfg(feature = "fmt")]
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
            RecordDataType::NS(_str) => {
                writeln!(f, "\t\tType: NS ({})", DnsTypeNum::NS)?;
                write_other(self, f)?;
                writeln!(
                    f,
                    "\t\tNS: {}",
                    _str.to_string().unwrap_or("???".to_owned())
                )
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
            RecordDataType::SOA(soa) => {
                writeln!(f, "\t\tType: SOA ({})", DnsTypeNum::SOA)?;
                write_other(self, f)?;
                soa.fmt_with_suffix(f, "\t\t")
            }
            RecordDataType::AAAA(addr) => {
                writeln!(f, "\t\tType: AAAA ({})", DnsTypeNum::AAAA)?;
                write_other(self, f)?;
                writeln!(f, "\t\tAAAA: {}", addr)
            }
        }
    }
}

#[cfg(feature = "fmt")]
pub enum RecordFmtType {
    Answers,
    Authoritative,
}

#[derive(Debug, Clone)]
pub enum RecordDataType {
    A(Ipv4Addr),
    NS(Rc<RawDomain>),
    CNAME(Rc<RawDomain>),
    SOA(SOA),
    AAAA(Ipv6Addr),
}

impl RecordDataType {
    #[cfg(feature = "fmt")]
    pub fn len(&self) -> usize {
        match self {
            RecordDataType::A(_) => 4,
            RecordDataType::NS(str) => str.raw_len(),
            RecordDataType::CNAME(str) => str.raw_len(),
            RecordDataType::SOA(soa) => soa.raw_len(),
            RecordDataType::AAAA(_) => 16,
        }
    }

    pub fn get_rtype(&self) -> u16 {
        match self {
            RecordDataType::A(_) => DnsTypeNum::A,
            RecordDataType::NS(_) => DnsTypeNum::NS,
            RecordDataType::CNAME(_) => DnsTypeNum::CNAME,
            RecordDataType::SOA(_) => DnsTypeNum::SOA,
            RecordDataType::AAAA(_) => DnsTypeNum::AAAA,
        }
    }

    #[cfg(feature = "fmt")]
    pub fn get_dns_type(&self) -> DnsType {
        match self {
            RecordDataType::A(_) => DnsType::A,
            RecordDataType::NS(_) => DnsType::NS,
            RecordDataType::CNAME(_) => DnsType::CNAME,
            RecordDataType::SOA(_) => DnsType::SOA,
            RecordDataType::AAAA(_) => DnsType::AAAA,
        }
    }
}
