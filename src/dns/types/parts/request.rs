#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::header::RequestHeader;
use crate::dns::types::parts::question::Question;
use crate::dns::types::parts::raw::RawRequest;
use crate::dns::utils::SliceOperator;
use rand::Rng;
use smallvec::SmallVec;

const SUFFIX: &[u8] = "xn--".as_bytes();

pub struct Request {
    header: RequestHeader,
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
            header: RequestHeader {
                id: rng.random(),
                response: 0,
                opcode: 0,
                truncated: 0,
                rec_desired: 0,
                z: 0,
                check_disable: 0,
            },
            question,
        }
    }

    pub fn encode_into(self, buffer: &mut [u8]) -> Option<&[u8]> {
        let mut operator = SliceOperator::from_slice(buffer);
        operator.set_pos(2);
        operator.write_u16(self.header.id);
        operator.skip(1);
        operator.write_u8(self.header.z << 6 | self.header.check_disable << 4);
        operator.write_u16(self.question.len() as u16);
        operator.write_u32(0);
        operator.write_u16(0);
        self.encode_question(&mut operator);
        let pos = operator.pos();
        buffer[4] =
            self.header.response << 7 | self.header.opcode << 3 | self.header.truncated << 1 | self.header.rec_desired;
        if pos - 2 > 512 {
            buffer[0..2].copy_from_slice(((pos - 2) as u16).to_be_bytes().as_ref());
            return Some(buffer[..pos].as_ref());
        }
        Some(buffer[2..pos].as_ref())
    }
    
    pub fn encode_into_tcp(self, buffer: &mut [u8]) -> Option<&[u8]> {
        let mut operator = SliceOperator::from_slice(buffer);
        operator.set_pos(2);
        operator.write_u16(self.header.id);
        operator.skip(1);
        operator.write_u8(self.header.z << 6 | self.header.check_disable << 4);
        operator.write_u16(self.question.len() as u16);
        operator.write_u32(0);
        operator.write_u16(0);
        self.encode_question(&mut operator);
        let pos = operator.pos();
        buffer[4] =
            self.header.response << 7 | self.header.opcode << 3 | self.header.truncated << 1 | self.header.rec_desired;
        buffer[0..2].copy_from_slice(((pos - 2) as u16).to_be_bytes().as_ref());
        Some(buffer[..pos].as_ref())
    }
    
    fn encode_question(&self, operator: &mut SliceOperator) -> Option<()> {
        for q in &self.question {
            let mut vec = q.qname.split('.').try_fold(
                SmallVec::new(),
                |mut v: SmallVec<[u8; 10]>, str| {
                    if str.is_ascii() {
                        v.push(str.len() as u8);
                        v.extend_from_slice(str.as_bytes());
                    } else {
                        match punycode::encode(str) {
                            Ok(s) => {
                                let mut len = SUFFIX.len() as u8;
                                let bytes = s.as_bytes();
                                len += bytes.len() as u8;
                                v.push(len);
                                v.extend_from_slice(SUFFIX);
                                v.extend_from_slice(bytes);
                            }
                            Err(_) => return None,
                        }
                    }
                    Some(v)
                },
            )?;
            vec.push(0x0);
            operator.write_slice(&vec);
            operator.write_u16(q.qtype);
            operator.write_u16(q.qclass);
        }
        Some(())
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
            header: RequestHeader::from(request.get_raw_header()),
            question,
        })
    }
}
