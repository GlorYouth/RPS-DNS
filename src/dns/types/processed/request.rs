#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::RawRequest;
use crate::dns::types::processed::header::Header;
use crate::dns::types::processed::question::Question;
use smallvec::SmallVec;
use crate::dns::utils::SliceOperator;

pub struct Request {
    pub header: Header,
    pub question: SmallVec<[Question; 5]>,
}

impl Request {
    #[inline]
    pub fn new(request: &RawRequest) -> Option<Request> {
        request.into()
    }
    
    #[inline]
    pub fn encode_into(self, buffer: &mut [u8]) {
        let mut operator = SliceOperator::from_slice(buffer);
        operator.write_u16(self.header.id);
        operator.write_u8(self.header.qr << 7 | self.header.opcode << 3 |
            self.header.aa << 2 | self.header.tc << 1 | self.header.rd);
        operator.write_u8(self.header.ra << 7 | self.header.z << 4 | self.header.rcode);
        operator.write_u16(self.question.len() as u16);
        operator.write_u32(0);
        operator.write_u16(0);
        for q in self.question {
            
        }
    }
}

impl From<&RawRequest<'_>> for Option<Request> {
    #[inline]
    fn from(request: &RawRequest) -> Option<Request> {
        let raw_question = request.get_raw_question();
        let mut question = SmallVec::new();

        for v in raw_question {
            question.push(Question::new(v)?);
        }
        Some(Request {
            header: Header::from(request.get_raw_header()),
            question,
        })
    }
    
}
