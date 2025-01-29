#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::RawQuestion;
use crate::dns::RawQuestionType;

pub enum QuestionType {
    Single(Question),
    Multiple(Vec<Question>),
}

impl From<RawQuestionType<'_>> for Option<QuestionType> {
    fn from(value: RawQuestionType) -> Option<QuestionType> {
        match value {
            RawQuestionType::Single(v) => {
                Some(QuestionType::Single(<RawQuestion<'_> as Into<Option<Question>>>::into(v.into())?))
            }
            RawQuestionType::Multiple(v) => {
                let mut vec = Vec::with_capacity(v.len());
                for q in v {
                    vec.push(<RawQuestion<'_> as Into<Option<Question>>>::into(q.into())?);
                }
                Some(QuestionType::Multiple(vec))
            }
            RawQuestionType::None => {
                None
            }
        }
    }
}

pub struct Question {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl From<RawQuestion<'_>> for Option<Question> {
    #[inline]
    fn from(question: RawQuestion) -> Option<Question> {
        Some(Question {
            qname: question.get_name()?,
            qtype: question.get_qtype(),
            qclass: question.get_qclass(),
        })
    }
}