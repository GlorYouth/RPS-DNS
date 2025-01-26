#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::error::Error;
use crate::{DNSRecord, Domain, RecordData};
use std::rc::Rc;

#[allow(non_snake_case)]
pub struct DNSRecordConstructor {
    pub NAME: String, //HashMap和Name均需要用引用
    pub TYPE: u16,
    pub CLASS: u16,
    pub TTL: u32,
    pub RDATA: Vec<u8>,
}

impl DNSRecordConstructor {
    pub fn construct(self) -> Result<DNSRecord, Error> {
        Ok(DNSRecord {
            NAME: Rc::new(Domain::from(&self.NAME)),
            TYPE: self.TYPE,
            CLASS: self.CLASS,
            TTL: self.TTL,
            RDLENGTH: self.RDATA.len() as u16,
            RDATA: RecordData::from_vec(self.RDATA, self.TYPE)?,
        })
    }
}
