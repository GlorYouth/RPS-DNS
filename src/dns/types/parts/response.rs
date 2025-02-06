#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::Request;
use crate::dns::types::parts::header::ResponseHeader;
use crate::dns::types::parts::question::Question;
use crate::dns::types::parts::raw::{RawResponse, RecordDataType};
use crate::dns::types::parts::record::Record;
#[cfg(debug_assertions)]
use log::trace;
use smallvec::SmallVec;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Response {
    pub header: ResponseHeader,
    pub question: SmallVec<[Question; 5]>,
    pub answer: SmallVec<[Record; 10]>,
    pub authority: SmallVec<[Record; 5]>,
    pub additional: SmallVec<[Record; 5]>,
}

impl Response {
    pub fn new(slice: &[u8]) -> Option<Response> {
        #[cfg(debug_assertions)]
        {
            trace!("开始从Slice解析RawResponse");
        }
        let mut raw = RawResponse::new(slice)?;
        #[cfg(debug_assertions)]
        {
            trace!("从Slice解析RawResponse除Header外部分");
        }
        raw.init_without_check()?;
        #[cfg(debug_assertions)]
        {
            trace!("开始全解析RawResponse");
        }
        Some(Response::from_raw(&raw)?)
    }

    pub fn from_slice_uncheck(slice: &[u8]) -> Option<Response> {
        let mut raw = RawResponse::new(slice)?;
        raw.init_without_check()?;
        Some(Response::from_raw(&raw)?)
    }

    pub fn from_raw(value: &RawResponse) -> Option<Response> {
        let raw_question = value.get_raw_question();
        let raw_answer = value.get_raw_answer();
        let raw_authority = value.get_raw_authority();
        let raw_additional = value.get_raw_additional();

        let mut question = SmallVec::new();
        let mut answer = SmallVec::new();
        let mut authority = SmallVec::new();
        let mut additional = SmallVec::new();

        for v in raw_question {
            #[cfg(debug_assertions)]
            {
                trace!("开始全解析Question {}", question.len());
            }
            question.push(Question::new(v)?);
        }

        for v in raw_answer {
            #[cfg(debug_assertions)]
            {
                trace!("开始全解析answer => Record {}", answer.len());
            }
            answer.push(Record::new(v)?);
        }

        for v in raw_authority {
            #[cfg(debug_assertions)]
            {
                trace!("开始全解析authority => Record {}", authority.len());
            }
            authority.push(Record::new(v)?);
        }

        for v in raw_additional {
            #[cfg(debug_assertions)]
            {
                trace!("开始全解析additional => Record {}", additional.len());
            }
            additional.push(Record::new(v)?);
        }

        Some(Response {
            header: ResponseHeader::from(value.get_raw_header()),
            question,
            answer,
            authority,
            additional,
        })
    }

    pub fn get_a_record(&self) -> Option<Ipv4Addr> {
        for answer in &self.answer {
            match answer.data {
                RecordDataType::A(addr) => {
                    return Some(addr); //这里隐式clone了一下
                }
                _ => {}
            }
        }
        None
    }
}

impl From<&RawResponse<'_>> for Option<Response> {
    #[inline]
    fn from(value: &RawResponse) -> Option<Response> {
        Response::from_raw(value)
    }
}

pub struct ResponseCheck<'a> {
    request: &'a Request,
}

impl<'a> ResponseCheck<'a> {
    #[inline]
    pub fn new(request: &'a Request) -> Self {
        Self { request }
    }

    #[inline]
    pub fn check_into_response(self, response_slice: &[u8]) -> Option<Response> {
        #[cfg(debug_assertions)]
        {
            trace!("开始从Slice解析RawResponse");
        }
        let mut raw = RawResponse::new(response_slice)?;
        #[cfg(debug_assertions)]
        {
            trace!("从Slice解析RawResponse除Header外部分");
        }
        raw.init(|header| {
            if header.get_id() != self.request.header.id {
                #[cfg(debug_assertions)]
                {
                    trace!(
                        "请求id和响应id不同,分别为{},{}",
                        header.get_id(),
                        self.request.header.id
                    );
                }
                return None;
            }
            if header.get_response() != 0x1 {
                #[cfg(debug_assertions)]
                {
                    trace!("响应的response flag非0x1");
                }
                return None;
            }
            if header.get_opcode() != self.request.header.opcode {
                #[cfg(debug_assertions)]
                {
                    trace!(
                        "请求和响应的opcode不同,分别为{},{}",
                        header.get_opcode(),
                        self.request.header.opcode
                    );
                }
                return None;
            }
            if header.get_rec_desired() != self.request.header.rec_desired {
                #[cfg(debug_assertions)]
                {
                    trace!(
                        "请求和响应的rec_desired不同,分别为{},{}",
                        header.get_rec_desired(),
                        self.request.header.rec_desired
                    );
                }
                return None;
            }
            if header.get_rcode() != 0x0 {
                #[cfg(debug_assertions)]
                {
                    trace!("响应的opcode不为0x0,而是{}", header.get_rcode());
                }
                return None;
            }
            if header.get_questions() != self.request.question.len() as u16 {
                #[cfg(debug_assertions)]
                {
                    trace!("响应的opcode不为0x0,而是{}", header.get_rcode());
                }
            }
            // todo tc authenticated .etc
            Some(())
        })?;
        #[cfg(debug_assertions)]
        {
            trace!("开始全解析RawResponse");
        }
        Some(Response::from_raw(&raw)?)
    }
}
