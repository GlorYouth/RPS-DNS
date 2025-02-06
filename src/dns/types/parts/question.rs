#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::raw::RawQuestion;
use std::rc::Rc;

#[derive(Debug)]
pub struct Question {
    pub qname: Rc<String>,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    #[inline]
    pub fn new(question: &RawQuestion) -> Option<Question> {
        question.into()
    }
}

impl From<&RawQuestion<'_>> for Option<Question> {
    #[inline]
    fn from(question: &RawQuestion) -> Option<Question> {
        Some(Question {
            qname: Rc::from(question.get_name()?),
            qtype: question.get_qtype(),
            qclass: question.get_qclass(),
        })
    }
}
