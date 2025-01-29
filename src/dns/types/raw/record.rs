#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::types::raw::domain::RawDomain;
use crate::dns::utils::SliceReader;
use std::collections::HashMap;
use std::rc::Rc;

pub struct RawRecord<'a> {
    name: Rc<RawDomain<'a>>,
    other: &'a [u8],
    // no data length, but you can use data.len() instead
    data: &'a [u8],
}

impl<'a> RawRecord<'a> {
    pub const FIX_SIZE: usize = 10;
    pub const LEAST_SIZE: usize = 12;

    pub fn new<'b>( // 'b为引用存在的周期，比'a对象存在的周期短或等于
        reader: &'b mut SliceReader<'a>,
        map: &'b mut HashMap<u16, RawDomain<'a>>,
    ) -> Option<RawRecord<'a>> {
        let name = RawDomain::new(reader, map)?;
        let len = reader.len();
        
        if reader.pos() + Self::FIX_SIZE > len {
            return None;
        }
        
        let other = reader.read_slice(8);
        let data_length = reader.read_u16() as usize;

        if reader.pos() + data_length > len {
            return None;
        }

        Some(RawRecord {
            name: Rc::from(name),
            other,
            data: reader.read_slice(data_length),
        })
    }
}
