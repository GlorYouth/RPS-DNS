#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::processed::header::Header;
use crate::dns::types::processed::question::QuestionType;
use crate::dns::RawRequest;

pub struct Request {
    header: Header,
    question: QuestionType
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
        Some(Request{
            header: Header::from(request.get_raw_header()),
            question: QuestionType::new(request.get_raw_question())?,
        })
    }
}