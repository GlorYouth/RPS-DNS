#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::types::raw::domain::RawDomain;
use crate::dns::types::raw::header::RawHeader;
use crate::dns::types::raw::question::{RawQuestion};
use crate::dns::utils::SliceReader;
use small_map::SmallMap;
use smallvec::SmallVec;

pub struct RawRequest<'a> {
    reader: SliceReader<'a>,

    raw_header: RawHeader<'a>,
    raw_question: SmallVec<[RawQuestion<'a>; 5]>,
}

impl<'a> RawRequest<'a> {
    #[inline]
    pub fn new(slice: &'a [u8]) -> Option<RawRequest<'a>> {
        if slice.len() < RawHeader::SIZE + RawQuestion::LEAST_SIZE {
            return None;
        }
        let mut reader = SliceReader::from_slice(slice);
        let raw_header = RawHeader::new(&mut reader);
        Some(RawRequest {
            reader,
            raw_header,
            raw_question: SmallVec::new(),
        })
    }

    pub fn init<F: FnMut(&RawHeader<'a>) -> Option<()>>(
        &mut self,
        map: &mut SmallMap<32, u16, RawDomain<'a>>,
        mut check: F,
    ) -> Option<()> {
        check(&self.raw_header)?;
        let qdcount = self.raw_header.get_qdcount();
        for _ in 0..qdcount {
            self.raw_question.push(RawQuestion::new(&mut self.reader,map)?)
        }
        Some(())
    }
    
    #[inline]
    pub fn get_raw_header(&self) -> &RawHeader<'a> {
        &self.raw_header
    }
    
    #[inline]
    pub fn get_raw_question(&self) -> &SmallVec<[RawQuestion<'a>; 5]> {
        &self.raw_question
    }
}
