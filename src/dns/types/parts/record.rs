#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::raw::{RawRecord, RecordDataType};

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

    pub fn is_a(&self) -> bool {
        if let RecordDataType::A(_) = self.data {
            true
        } else {
            false
        }
    }

    pub fn is_aaaa(&self) -> bool {
        if let RecordDataType::AAAA(_) = self.data {
            true
        } else {
            false
        }
    }

    pub fn is_cname(&self) -> bool {
        if let RecordDataType::CNAME(_) = self.data {
            true
        } else {
            false
        }
    }
}

impl From<&RawRecord<'_>> for Option<Record> {
    #[inline]
    fn from(record: &RawRecord) -> Option<Record> {
        Record::new(record)
    }
}
