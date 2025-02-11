#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::base::record::{A, AAAA, CNAME, NS, SOA};
#[cfg(feature = "fmt")]
use crate::dns::types::base::{DnsClass, DnsTTL};
use crate::dns::types::base::{DnsTypeNum, RawDomain};
use crate::dns::utils::SliceReader;
#[cfg(feature = "logger")]
use log::{debug, trace};
#[cfg(feature = "fmt")]
use std::fmt::Display;

#[derive(Debug)]
pub struct Record {
    pub name: RawDomain,
    pub rtype: u16,
    pub class: u16,
    pub ttl: u32,
    #[cfg(feature = "fmt")]
    pub data_len: u16,
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
        let data_len = reader.read_u16();
        let data_len_usize = data_len as usize;

        if reader.pos() + data_len_usize > len {
            #[cfg(feature = "logger")]
            debug!(
                "读取到Record中Data可变部分长度为{:x},需要总Slice长度为{:x},实际Slice长度{:x}",
                data_len_usize,
                reader.pos() + data_len_usize,
                len
            );
            return None;
        }

        macro_rules! match_rtype {
            { $($field:ident),* } => {
                // Define a match block. This expands to:

                //match rtype {
                //      DnsTypeNum::A => RecordDataType::A(A::from_reader_with_size(reader, data_len_usize)?),
                //      DnsTypeNum::NS => RecordDataType::NS(NS::from_reader_with_size(reader, data_len_usize)?),
                //      DnsTypeNum::CNAME => RecordDataType::CNAME(CNAME::from_reader_with_size(reader, data_len_usize)?),
                //      DnsTypeNum::SOA => RecordDataType::SOA(SOA::from_reader_with_size(reader, data_len_usize)?)
                //      RecordDataType::AAAA(AAAA::from_reader_with_size(reader, data_len_usize)?)
                //      _ => {
                //          #[cfg(feature = "logger")]
                //          trace!("Unsupported Type: {}", rtype);
                //          return None;
                //      }
                // }
                //

                match rtype {
                    $(
                        DnsTypeNum::$field => RecordDataType::$field($field::from_reader_with_size(reader, data_len_usize)?),
                    )*
                    _ => {
                        #[cfg(feature = "logger")]
                        trace!("Unsupported Type: {}", rtype);
                        return None;
                    }
                }
            }
        }

        let data = match_rtype! {A,NS,CNAME,SOA,AAAA};

        // todo

        Some(Record {
            name,
            rtype,
            class,
            ttl,
            #[cfg(feature = "fmt")]
            data_len,
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
        write!(f, "\t{}: type ", self.name)?;
        let (type_name, dns_type) = self.data.get_type_info();
        write!(f, "{}", type_name)?;
        writeln!(
            f,
            ", Class: {} ({:#06X})",
            DnsClass::get_str(self.class),
            self.class
        )?;

        writeln!(f, "\t\tName: {}", self.name)?;

        writeln!(f, "\t\tType: {} ({})", type_name, dns_type)?;
        writeln!(
            f,
            "\t\tClass: {} ({:#06X})",
            DnsClass::get_str(self.class),
            self.class
        )?;
        writeln!(f, "\t\tTTL: {} ({})", self.ttl, DnsTTL::get_str(self.ttl))?;
        writeln!(f, "\t\tData length: {}", self.data_len)?;

        macro_rules! match_data {
            { $($field:ident),* } => {
                // Define a match block. This expands to:

                //match &self.data {
                //      RecordDataType::A(v) => v.fmt_with_suffix(f, "\t\t"),
                //      RecordDataType::NS(v) => v.fmt_with_suffix(f, "\t\t"),
                //      RecordDataType::CNAME(v) => v.fmt_with_suffix(f, "\t\t"),
                //      RecordDataType::SOA(v) => v.fmt_with_suffix(f, "\t\t"),
                //      RecordDataType::AAAA(v) => v.fmt_with_suffix(f, "\t\t"),
                // }

                match &self.data {
                    $(
                        RecordDataType::$field(v) => v.fmt_with_suffix(f, "\t\t"),
                    )*
                }
            }
        }

        match_data! {A,NS,CNAME,SOA,AAAA}
    }
}

macro_rules! impl_record {
    { $($field:ident),* } => {
        // Define a func. This expands to:

        //impl RecordDataType {
        //     fn get_type_info(&self) -> (&'static str, u16) {
        //         match self {
        //             Self::A(_) => ("A", DnsTypeNum::A),
        //             Self::NS(_) => ("NS", DnsTypeNum::NS),
        //             Self::CNAME(_) => ("CNAME", DnsTypeNum::CNAME),
        //             Self::SOA(_) => ("SOA", DnsTypeNum::SOA),
        //             Self::AAAA(_) => ("AAAA", DnsTypeNum::AAAA),
        //         }
        //     }
        // }
        impl RecordDataType {

            #[cfg(feature = "fmt")]
            pub fn get_type_info(&self) -> (&'static str, u16) {
                match self {
                    $(
                        Self::$field(_) => (stringify!($field), DnsTypeNum::$field),
                    )*
                }
            }
        }
    }
}

#[cfg(feature = "fmt")]
pub enum RecordFmtType {
    Answers,
    Authoritative,
}

// todo

#[derive(Debug, Clone)]
pub enum RecordDataType {
    A(A),
    NS(NS),
    CNAME(CNAME),
    SOA(SOA),
    AAAA(AAAA),
}

impl_record! {A,NS,CNAME,SOA,AAAA}
