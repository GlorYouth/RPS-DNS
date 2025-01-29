#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::RawRequest;
use crate::dns::types::processed::header::Header;
use crate::dns::types::processed::question::Question;
use smallvec::SmallVec;

pub struct Request {
    header: Header,
    question: SmallVec<[Question; 5]>,
}

impl Request {
    #[inline]
    pub fn new(request: &RawRequest) -> Option<Request> {
        request.into()
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
