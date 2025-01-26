use crate::dns::component::small_parts::question::question::DNSQuestion;
use crate::dns::component::*;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(unused)]
#[derive(Debug)]
pub enum QuestionBody {
    Single(DNSQuestion),
    Multi(Vec<DNSQuestion>),
}

impl QuestionBody {
    pub const ESTIMATE_SIZE_FOR_ONE: usize = DNSQuestion::ESTIMATE_SIZE;

    pub fn from_reader(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
        qdcount: u16,
    ) -> Result<QuestionBody, DomainReadError> {
        if qdcount == 1 {
            return Ok(QuestionBody::Single(DNSQuestion::from_reader(reader, map)?));
        }
        let mut vec = Vec::with_capacity(qdcount as usize);
        for _ in 0..qdcount {
            vec.push(DNSQuestion::from_reader(reader, map)?);
        }
        Ok(QuestionBody::Multi(vec))
    }

    pub fn get_domains(&self) -> Result<Vec<String>, DomainDecodeError> {
        match self {
            QuestionBody::Single(question) => Ok(vec![question.get_domain()?]),
            QuestionBody::Multi(questions) => {
                let mut vec = Vec::with_capacity(questions.len());
                for question in questions {
                    vec.push(question.get_domain()?);
                }
                Ok(vec)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test() {
        let mut map: HashMap<u16, Rc<Domain>> = HashMap::new();
        let single_question = QuestionBody::from_reader(
            &mut SliceReader::from_slice(&[
                0x03, 0x69, 0x70, 0x77, 0x02, 0x63, 0x6e, 0x00, 0x00, 0x1c, 0x00, 0x01,
            ]),
            &mut map,
            1,
        ).unwrap();
        assert_eq!(single_question.get_domains().unwrap()[0], "ipw.cn");
        if let QuestionBody::Single(question) = single_question {
            assert_eq!(question.get_domain().unwrap(), "ipw.cn");
            assert_eq!(question.QTYPE, 0x1c);
            assert_eq!(question.QCLASS, 0x1);
        }

        map.clear();
        let multi_question = QuestionBody::from_reader(
            &mut SliceReader::from_slice(&[
                0x03, 0x69, 0x70, 0x77, 0x02, 0x63, 0x6e, 0x00, 0x00, 0x1c, 0x00, 0x01, 3, 119,
                119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0x0, 0x1, 0x00,
                0x01,
            ]),
            &mut map,
            2,
        ).unwrap();
        assert_eq!(multi_question.get_domains().unwrap()[0], "ipw.cn");
        assert_eq!(multi_question.get_domains().unwrap()[1], "www.google.com");
        if let QuestionBody::Multi(questions) = multi_question {
            assert_eq!(questions[0].get_domain().unwrap(), "ipw.cn");
            assert_eq!(questions[0].QTYPE, 0x1c);
            assert_eq!(questions[0].QCLASS, 0x1);
            assert_eq!(questions[1].get_domain().unwrap(), "www.google.com");
            assert_eq!(questions[1].QTYPE, 0x1);
            assert_eq!(questions[1].QCLASS, 0x1);
        }
    }
}
