#![cfg_attr(debug_assertions, allow(dead_code))]

use small_map::SmallMap;
use crate::dns::types::parts::header::Header;
use crate::dns::types::parts::question::Question;
use crate::dns::types::parts::record::Record;
use smallvec::SmallVec;
use crate::dns::types::parts::raw::RawAnswer;

#[derive(Debug)]
pub struct Answer {
    pub header: Header,
    pub question: SmallVec<[Question; 5]>,
    pub answer: SmallVec<[Record; 10]>,
    pub authority: SmallVec<[Record; 5]>,
    pub additional: SmallVec<[Record; 5]>,
}

impl Answer {
    pub fn new(slice: &[u8]) -> Option<Answer> {
        let mut raw = RawAnswer::new(slice)?;
        let mut map = SmallMap::new();
        raw.init(&mut map, |_h| Some(()))?;
        Some(Answer::from_raw(&raw)?)
    }
    
    
    pub fn from_raw(value: &RawAnswer) -> Option<Answer> {
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
        Answer::from_raw(value)
    }
}
