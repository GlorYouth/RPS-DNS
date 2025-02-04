#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::header::AnswerHeader;
use crate::dns::types::parts::question::Question;
use crate::dns::types::parts::raw::RawAnswer;
use crate::dns::types::parts::record::Record;
use log::trace;
use smallvec::SmallVec;

#[derive(Debug)]
pub struct Answer {
    pub header: AnswerHeader,
    pub question: SmallVec<[Question; 5]>,
    pub answer: SmallVec<[Record; 10]>,
    pub authority: SmallVec<[Record; 5]>,
    pub additional: SmallVec<[Record; 5]>,
}

impl Answer {
    pub fn new(slice: &[u8]) -> Option<Answer> {
        #[cfg(debug_assertions)]
        {
            trace!("开始从Slice解析RawAnswer");
        }
        let mut raw = RawAnswer::new(slice)?;
        #[cfg(debug_assertions)]
        {
            trace!("从Slice解析RawAnswer除Header外部分");
        }
        raw.init(|_h| Some(()))?;
        #[cfg(debug_assertions)]
        {
            trace!("开始全解析RawAnswer");
        }
        Some(Answer::from_raw(&raw)?)
    }

    pub fn from_slice(slice: &[u8]) -> Option<Answer> {
        let mut raw = RawAnswer::new(slice)?;
        raw.init(|_h| Some(()))?;
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

        Some(Answer {
            header: AnswerHeader::from(value.get_raw_header()),
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
