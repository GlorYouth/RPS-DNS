#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::parts::record::record::DNSRecord;
use crate::dns::parts::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct RecordBody(Vec<DNSRecord>);

impl RecordBody {
    pub fn from_reader(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
        count: u16,
    ) -> RecordBody {
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..count {
            records.push(DNSRecord::from_reader(reader, map));
        }
        RecordBody(records)
    }
}
