#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::parts::base::*;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DNSRecord {
    pub NAME: Rc<Domain>, //HashMap和Name均需要用引用
    pub TYPE: u16,
    pub CLASS: u16,
    pub TTL: u32,
    pub RDLENGTH: u16,
    pub RDATA: RecordData,
}

impl DNSRecord {
    const ENSURED_SIZE: usize = 10;
    pub const ESTIMATED_SIZE: usize =
        Self::ENSURED_SIZE + Domain::ESTIMATE_DOMAIN_SIZE + RecordData::ESTIMATE_SIZE;

    pub fn get_size(&self) -> usize {
        self.NAME.len() + self.RDATA.len() + 8
    }
}

impl DNSRecord {
    pub fn from_reader(reader: &mut SliceReader, map: &mut HashMap<u16, Rc<Domain>>) -> Self {
        let name = Domain::from_reader_and_check_map(reader, map);
        let rtype = reader.read_u16();
        let class = reader.read_u16();
        let ttl = reader.read_u32();
        let rdlength = reader.read_u16();
        let rdata = RecordData::from_reader(reader, map, rtype);
        DNSRecord {
            NAME: name,
            TYPE: rtype,
            CLASS: class,
            TTL: ttl,
            RDLENGTH: rdlength,
            RDATA: rdata,
        }
    }
}