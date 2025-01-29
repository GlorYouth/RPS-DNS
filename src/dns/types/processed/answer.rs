#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::RawAnswer;
use crate::dns::types::processed::header::Header;
use crate::dns::types::processed::question::Question;
use crate::dns::types::processed::record::Record;
use smallvec::SmallVec;

#[derive(Debug)]
pub struct Answer {
    pub header: Header,
    pub question: SmallVec<[Question; 5]>,
    pub answer: SmallVec<[Record; 10]>,
    pub authority: SmallVec<[Record; 5]>,
    pub additional: SmallVec<[Record; 5]>,
}

impl Answer {
    pub fn new(value: &RawAnswer) -> Option<Answer> {
        let raw_question = value.get_raw_question();
        let raw_answer = value.get_raw_answer();
        let raw_authority = value.get_raw_authority();
        let raw_additional = value.get_raw_additional();

        let mut question = SmallVec::new();
        let mut answer = SmallVec::new();
        let mut authority = SmallVec::new();
        let mut additional = SmallVec::new();

        for v in raw_question {
            question.push(Question::new(v)?);
        }

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
            question,
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
