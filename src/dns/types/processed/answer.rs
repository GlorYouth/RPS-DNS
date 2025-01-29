#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::types::processed::header::Header;
use crate::dns::types::processed::question::QuestionType;
use crate::dns::types::processed::record::Record;
use crate::dns::RawAnswer;

#[derive(Debug)]
pub struct Answer {
    pub header: Header,
    pub question: QuestionType,
    pub answer: Vec<Record>,
    pub authority: Vec<Record>,
    pub additional: Vec<Record>,
}

impl Answer {
    pub fn new(value: &RawAnswer) -> Option<Answer> {
        let raw_answer = value.get_raw_answer();
        let raw_authority = value.get_raw_authority();
        let raw_additional = value.get_raw_additional();

        let mut answer = Vec::with_capacity(raw_answer.len());
        let mut authority = Vec::with_capacity(raw_authority.len());
        let mut additional = Vec::with_capacity(raw_additional.len());

        for v in raw_answer {
            answer.push(Record::new(v)?);
        }

        for v in raw_authority {
            authority.push(Record::new(v)?);
        }

        for v in raw_additional {
            additional.push(Record::new(v)?);
        }

        Some(Answer {
            header: Header::from(value.get_raw_header()),
            question: QuestionType::new(value.get_raw_question())?,
            answer,
            authority,
            additional,
        })
    }
}

impl From<&RawAnswer<'_>> for Option<Answer> {
    #[inline]
    fn from(value: &RawAnswer) -> Option<Answer> {
        Answer::new(value)
    }
}
