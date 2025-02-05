#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::types::parts::raw::RawRequestHeader;
use crate::dns::types::parts::raw::header::RawResponseHeader;
use crate::dns::types::parts::raw::question::RawQuestion;
use crate::dns::utils::SliceReader;
use smallvec::SmallVec;

pub struct RawRequest<'a> {
    reader: SliceReader<'a>,

    raw_header: RawRequestHeader<'a>,
    raw_question: SmallVec<[RawQuestion<'a>; 5]>,
}

impl<'a> RawRequest<'a> {
    #[inline]
    pub fn new(slice: &'a [u8]) -> Option<RawRequest<'a>> {
        if slice.len() < RawResponseHeader::SIZE + RawQuestion::LEAST_SIZE {
            return None;
        }
        let mut reader = SliceReader::from_slice(slice);
        let raw_header = RawRequestHeader::new(&mut reader);
        Some(RawRequest {
            reader,
            raw_header,
            raw_question: SmallVec::new(),
        })
    }

    pub fn init<F: FnMut(&RawRequestHeader<'a>) -> Option<()>>(
        &mut self,
        mut check: F,
    ) -> Option<()> {
        check(&self.raw_header)?;
        let qdcount = self.raw_header.get_questions();
        for _ in 0..qdcount {
            self.raw_question.push(RawQuestion::new(&mut self.reader)?)
        }
        Some(())
    }

    #[inline]
    pub fn get_raw_header(&self) -> &RawRequestHeader<'a> {
        &self.raw_header
    }

    #[inline]
    pub fn get_raw_question(&self) -> &SmallVec<[RawQuestion<'a>; 5]> {
        &self.raw_question
    }
}
