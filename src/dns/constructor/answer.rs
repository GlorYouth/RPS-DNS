#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use crate::dns::constructor::small_parts::{
    DNSHeaderConstructor, DNSQuestionConstructor, DNSRecordConstructor,
};
use crate::dns::error::{Error, ErrorKind};
use crate::{DNSAnswer, DNSHeader, DNSRecord, QuestionBody, RecordBody};

pub struct DNSAnswerConstructor {
    pub header: DNSHeaderConstructor,
    pub questions: Vec<DNSQuestionConstructor>,
    pub answer: Vec<DNSRecordConstructor>,
    pub authority: Vec<DNSRecordConstructor>,
    pub additional: Vec<DNSRecordConstructor>,
}

impl DNSAnswerConstructor {
    pub fn construct(self) -> Result<DNSAnswer, Error> {
        let header = self.header.construct();
        let answers_len = self.answer.len();
        let authority_len = self.authority.len();
        let additional_len = self.additional.len();

        let answers = RecordBody(self.answer.into_iter().try_fold(
            Vec::with_capacity(answers_len),
            |mut acc, q| match q.construct() {
                Ok(t) => {
                    acc.push(t);
                    Ok(acc)
                }
                Err(e) => Err(e),
            },
        )?);
        let authority = RecordBody(self.authority.into_iter().try_fold(
            Vec::with_capacity(authority_len),
            |mut acc, q| match q.construct() {
                Ok(t) => {
                    acc.push(t);
                    Ok(acc)
                }
                Err(e) => Err(e),
            },
        )?);
        let additional = RecordBody(self.additional.into_iter().try_fold(
            Vec::with_capacity(additional_len),
            |mut acc, q| match q.construct() {
                Ok(t) => {
                    acc.push(t);
                    Ok(acc)
                }
                Err(e) => Err(e),
            },
        )?);
        if self.questions.len() == 1 {
            return Ok(DNSAnswer {
                header,
                question: QuestionBody::Single(
                    self.questions.into_iter().next().unwrap().construct(),
                ),
                answer: answers,
                authority,
                additional,
                domain_map: Default::default(),
            });
        }
        if self.questions.len() > 2 {
            return Ok(DNSAnswer {
                header,
                question: QuestionBody::Multi(
                    self.questions.into_iter().map(|q| q.construct()).collect(),
                ),
                answer: answers,
                authority,
                additional,
                domain_map: Default::default(),
            });
        }
        Err(ErrorKind::DNSQuestionNumber)?
    }
}
