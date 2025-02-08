#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::raw::{DnsClass, DnsTTL, RawRecord, RecordDataType};
use std::fmt::Display;
use crate::DnsTypeNum;

#[derive(Debug)]
pub struct Record {
    pub name: String,
    pub class: u16,
    pub ttl: u32,
    pub data: RecordDataType,
}

impl Record {
    #[inline]
    pub fn new(record: &RawRecord) -> Option<Record> {
        Some(Record {
            name: record.get_name()?,
            class: record.get_class(),
            ttl: record.get_ttl(),
            data: record.get_data()?,
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
        write!(f, "\t{}: type ", self.name)?;
        Display::fmt(&self.data.get_dns_type(), f)?;
        writeln!(
            f,
            ", Class: {} ({:#06X})",
            DnsClass::get_str(self.class),
            self.class
        )?;

        writeln!(f, "\t\tName: {}", self.name)?;

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
                writeln!(f, "\t\tType: A ({})",DnsTypeNum::A)?;
                write_other(self, f)?;
                writeln!(f, "\t\tA: {}", addr)
            }
            RecordDataType::AAAA(addr) => {
                writeln!(f, "\t\tType: AAAA ({})",DnsTypeNum::AAAA)?;
                write_other(self, f)?;
                writeln!(f, "\t\tAAAA: {}", addr)
            }
            RecordDataType::CNAME(str) => {
                writeln!(f, "\t\tType: CNAME ({})",DnsTypeNum::CNAME)?;
                write_other(self, f)?;
                writeln!(f, "\t\tCNAME: {}", str.0)
            }
        }
    }
}

impl From<&RawRecord<'_>> for Option<Record> {
    #[inline]
    fn from(record: &RawRecord) -> Option<Record> {
        Record::new(record)
    }
}

pub enum RecordFmtType {
    Answers,
}
