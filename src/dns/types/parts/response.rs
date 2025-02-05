#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::header::ResponseHeader;
use crate::dns::types::parts::question::Question;
use crate::dns::types::parts::raw::RawResponse;
use crate::dns::types::parts::record::Record;
#[cfg(debug_assertions)]
use log::trace;
use smallvec::SmallVec;

#[derive(Debug)]
pub struct Response {
    pub header: ResponseHeader,
    pub question: SmallVec<[Question; 5]>,
    pub response: SmallVec<[Record; 10]>,
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
        raw.init(|_h| Some(()))?;
        #[cfg(debug_assertions)]
        {
            trace!("开始全解析RawResponse");
        }
        Some(Response::from_raw(&raw)?)
    }

    pub fn from_slice(slice: &[u8]) -> Option<Response> {
        let mut raw = RawResponse::new(slice)?;
        raw.init(|_h| Some(()))?;
        Some(Response::from_raw(&raw)?)
    }

    pub fn from_raw(value: &RawResponse) -> Option<Response> {
        let raw_question = value.get_raw_question();
        let raw_response = value.get_raw_response();
        let raw_authority = value.get_raw_authority();
        let raw_additional = value.get_raw_additional();

        let mut question = SmallVec::new();
        let mut response = SmallVec::new();
        let mut authority = SmallVec::new();
        let mut additional = SmallVec::new();

        for v in raw_question {
            #[cfg(debug_assertions)]
            {
                trace!("开始全解析Question {}", question.len());
            }
            question.push(Question::new(v)?);
        }

        for v in raw_response {
            #[cfg(debug_assertions)]
            {
                trace!("开始全解析response => Record {}", response.len());
            }
            response.push(Record::new(v)?);
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
            response,
            authority,
            additional,
        })
    }
}

impl From<&RawResponse<'_>> for Option<Response> {
    #[inline]
    fn from(value: &RawResponse) -> Option<Response> {
        Response::from_raw(value)
    }
}
