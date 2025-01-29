use crate::dns::types::raw::domain::RawDomain;
use crate::dns::types::raw::header::RawHeader;
use crate::dns::types::raw::question::{RawQuestion, RawQuestionType};
use crate::dns::types::raw::record::RawRecord;
use crate::dns::utils::SliceReader;
use std::collections::HashMap;

pub struct RawAnswer<'a> {
    reader: SliceReader<'a>,

    raw_header: RawHeader<'a>,
    raw_question: RawQuestionType<'a>,
    answer: Vec<RawRecord<'a>>,
    authority: Vec<RawRecord<'a>>,
    additional: Vec<RawRecord<'a>>,
}

impl<'a> RawAnswer<'a> {
    #[inline]
    pub fn new(slice: &'a [u8]) -> Option<RawAnswer<'a>> {
        if slice.len() < RawHeader::SIZE + RawQuestion::LEAST_SIZE {
            return None;
        }
        let mut reader = SliceReader::from_slice(slice);
        let raw_header = RawHeader::new(&mut reader);
        Some(RawAnswer {
            reader,
            raw_header,
            raw_question: RawQuestionType::None,
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        })
    }

    pub fn init<F: FnMut(&RawHeader<'a>) -> Option<()>>(
        &'a mut self,
        mut map: &'a mut HashMap<u16, RawDomain<'a>>,
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
        let ancount = self.raw_header.get_ancount();
        let nscount = self.raw_header.get_nscount();
        let arcount = self.raw_header.get_arcount();

        self.answer = Vec::with_capacity(ancount as usize);
        self.authority = Vec::with_capacity(nscount as usize);
        self.additional = Vec::with_capacity(arcount as usize);

        for _ in 0..ancount {
            self.answer
                .push(RawRecord::new(&mut self.reader, &mut map)?);
        }

        for _ in 0..nscount {
            self.authority
                .push(RawRecord::new(&mut self.reader, &mut map)?);
        }

        for _ in 0..arcount {
            self.additional
                .push(RawRecord::new(&mut self.reader, &mut map)?);
        }

        Some(())
    }
}

#[cfg(test)]
mod test {
    use crate::dns::types::raw::answer::RawAnswer;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let mut raw = RawAnswer::new(
            &[
                0xb4_u8, 0xdb, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x03,
                0x64, 0x6e, 0x73, 0x06, 0x77, 0x65, 0x69, 0x78, 0x69, 0x6e, 0x02, 0x71, 0x71, 0x03,
                0x63, 0x6f, 0x6d, 0x02, 0x63, 0x6e, 0x00, 0x00, 0x1c, 0x00, 0x01, 0xc0, 0x0c, 0x00,
                0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00,
                0x02, 0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0xc0, 0x0c, 0x00,
                0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00,
                0x10, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0xc0, 0x0c, 0x00,
                0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00,
                0x02, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0xc0, 0x0c, 0x00,
                0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00,
                0x10, 0x10, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25,
            ][..],
        )
        .unwrap();
        let mut map = HashMap::new();
        raw.init(&mut map, |_h| Some(())).unwrap()
    }
}
