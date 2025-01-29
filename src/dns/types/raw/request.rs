#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::types::raw::domain::RawDomain;
use crate::dns::types::raw::header::RawHeader;
use crate::dns::types::raw::question::{RawQuestion, RawQuestionType};
use crate::dns::utils::SliceReader;
use small_map::SmallMap;

pub struct RawRequest<'a> {
    reader: SliceReader<'a>,

    raw_header: RawHeader<'a>,
    raw_question: RawQuestionType<'a>,
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
            raw_question: RawQuestionType::None,
        })
    }

    pub fn init<F: FnMut(&RawHeader<'a>) -> Option<()>>(
        &mut self,
        map: &mut SmallMap<32, u16, RawDomain<'a>>,
        mut check: F,
    ) -> Option<()> {
        check(&self.raw_header)?;
        let qdcount = self.raw_header.get_qdcount();
        if qdcount == 1 {
            self.raw_question = RawQuestionType::Single(RawQuestion::new(&mut self.reader, map)?);
        } else if qdcount > 1 {
            if self.reader.len() < RawHeader::SIZE + RawQuestion::LEAST_SIZE * qdcount as usize {
                return None;
            }
            let mut vec = Vec::with_capacity(self.raw_header.get_qdcount() as usize);
            vec.push(RawQuestion::new(&mut self.reader, map)?);
            self.raw_question = RawQuestionType::Multiple(vec);
        } else {
            return None;
        }
        Some(())
    }
    
    #[inline]
    pub fn get_raw_header(&self) -> &RawHeader<'a> {
        &self.raw_header
    }
    
    #[inline]
    pub fn get_raw_question(&self) -> &RawQuestionType<'a> {
        &self.raw_question
    }
}
