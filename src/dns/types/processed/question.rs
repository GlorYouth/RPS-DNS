#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::RawQuestion;
use crate::dns::RawQuestionType;

#[derive(Debug)]
pub enum QuestionType {
    Single(Question),
    Multiple(Vec<Question>),
}

impl QuestionType {
    #[inline]
    pub fn new(value: &RawQuestionType) -> Option<QuestionType> {
        value.into()
    }
}

impl From<&RawQuestionType<'_>> for Option<QuestionType> {
    #[inline]
    fn from(value: &RawQuestionType) -> Option<QuestionType> {
        match value {
            RawQuestionType::Single(v) => {
                Some(QuestionType::Single(Question::new(v)?))
            }
            RawQuestionType::Multiple(v) => {
                let mut vec = Vec::with_capacity(v.len());
                for q in v {
                    vec.push(Question::new(q)?);
                }
                Some(QuestionType::Multiple(vec))
            }
            RawQuestionType::None => {
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct Question {
    pub qname: String,
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
            qname: question.get_name()?,
            qtype: question.get_qtype(),
            qclass: question.get_qclass(),
        })
    }
}