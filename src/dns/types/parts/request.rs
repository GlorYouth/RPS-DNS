#![cfg_attr(debug_assertions, allow(dead_code))]

use rand::Rng;
use crate::dns::types::parts::header::Header;
use crate::dns::types::parts::question::Question;
use smallvec::SmallVec;
use crate::dns::types::parts::raw::RawRequest;
use crate::dns::utils::SliceOperator;

const SUFFIX:& [u8] = "xn--".as_bytes();

pub struct Request {
    header: Header,
    question: SmallVec<[Question; 5]>,
}

impl Request {
    #[inline]
    pub fn from_raw(request: &RawRequest) -> Option<Request> {
        request.into()
    }
    
    #[inline]
    pub fn new(domain: String, qtype: u16) -> Request {
        let mut rng = rand::rng();
        let mut question = SmallVec::new();
        question.push(Question {
            qname: domain,
            qtype,
            qclass: 1,
        });

        Request {
            header: Header {
                id: rng.random(),
                qr: 0,
                opcode: 0,
                aa: 0,
                rd: 0,
                ra: 0,
                z: 0,
                rcode: 0,
            },
            question,
        }
    }

    #[inline]
    pub fn encode_into(self, buffer: &mut [u8]) -> Option<usize> {
        let mut operator = SliceOperator::from_slice(buffer);
        operator.write_u16(self.header.id);
        operator.skip(1);
        operator.write_u8(self.header.ra << 7 | self.header.z << 4 | self.header.rcode);
        operator.write_u16(self.question.len() as u16);
        operator.write_u32(0);
        operator.write_u16(0);
        for q in self.question {
            let mut vec = q.qname.split('.').try_fold(SmallVec::new(), |mut v: SmallVec<[u8;10]>, str| {
                if str.is_ascii() {
                    v.push(str.len() as u8);
                    v.extend_from_slice(str.as_bytes());
                }
                 else {
                     match punycode::encode(str) {
                         Ok(s) => {
                             let mut len = SUFFIX.len() as u8;
                             let bytes = s.as_bytes();
                             len += bytes.len() as u8;
                             v.push(len);
                             v.extend_from_slice(SUFFIX);
                             v.extend_from_slice(bytes);
                         }
                         Err(_) => {
                             return None
                         }
                     }
                 }
                Some(v)
            })?;
            vec.push(0x0);
            operator.write_slice(&vec);
            operator.write_u16(q.qtype);
            operator.write_u16(q.qclass);
        }
        let len = operator.pos();
        if len > 512 {
            buffer[2] = self.header.qr << 7 | self.header.opcode << 3 |
                self.header.aa << 2 | 0b0000_0010 | self.header.rd;
        } else {
            buffer[2] = self.header.qr << 7 | self.header.opcode << 3 |
                self.header.aa << 2 | self.header.rd;
        }
        Some(len)
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
