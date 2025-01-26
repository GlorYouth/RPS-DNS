#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use crate::dns::constructor::answer::DNSAnswerConstructor;
use crate::dns::constructor::small_parts::{DNSHeaderConstructor, DNSQuestionConstructor};
use crate::{DNSAnswer, DNSRequest, QuestionBody, RecordBody};
use std::collections::HashMap;
use crate::dns::error::Error;
use crate::dns::error::Error::InvalidVecLength;

pub struct DNSRequestConstructor {
    pub header: DNSHeaderConstructor,
    pub questions: Vec<DNSQuestionConstructor>,
}

impl DNSRequestConstructor {
    pub fn construct(self) -> Result<DNSRequest, Error> {
        let header = self.header.construct();
        if self.questions.len() == 1 {
            return Ok(DNSRequest {
                header,
                question: QuestionBody::Single(
                    self.questions.into_iter().next().unwrap().construct(),
                ),
                map: HashMap::new(),
            });
        }
        if self.questions.len() > 2 {
            return Ok(DNSRequest {
                header,
                question: QuestionBody::Multi(
                    self.questions.into_iter().map(|q| q.construct()).collect(),
                ),
                map: HashMap::new(),
            });
        }
        Err(InvalidVecLength { length: self.questions.len() })?
    }
}
