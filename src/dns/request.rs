#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use std::collections::HashMap;
use std::rc::Rc;
use crate::dns::parts::*;
use rand::Rng;



struct DNSRequest {
    header: DNSHeader,
    question: QuestionBody,
    
    map: HashMap<u16, Rc<Domain>>,
}

impl DNSRequest {
    pub const ESTIMATE_SIZE: usize = DNSHeader::SIZE+DNSQuestion::ESTIMATE_SIZE;
    
    fn new(host: &String, qtype: DNSType) -> DNSRequest {
        DNSRequest {
            header: DNSHeader {
                ID: rand::rng().random(),
                FLAGS: ArrayU8::from_bytes(&[0x1, 0x0]),
                QDCOUNT: 1,
                ANCOUNT: 0,
                NSCOUNT: 0,
                ARCOUNT: 0,
            },
            question: QuestionBody::Single(DNSQuestion {
                QNAME: Rc::from(Domain::from_str(host)),
                QTYPE: qtype.to_u16(),
                QCLASS: 1,
            }),
            map: Default::default(),
        }
    }

    fn new_with_vec(hosts: Vec<(String, DNSType)>) -> DNSRequest {
        DNSRequest {
            header: DNSHeader {
                ID: rand::rng().random(),
                FLAGS: ArrayU8::from_bytes(&[0x1, 0x0]),
                QDCOUNT: hosts.len() as u16,
                ANCOUNT: 0,
                NSCOUNT: 0,
                ARCOUNT: 0,
            },
            question: QuestionBody::Multi(
                hosts
                    .iter()
                    .map(|v| DNSQuestion {
                        QNAME: Rc::from(Domain::from_str(&v.0)),
                        QTYPE: v.1.to_u16(),
                        QCLASS: 1,
                    })
                    .collect(),
            ),
            map: Default::default(),
        }
    }

    fn estimate_size(&self) -> usize {
        DNSHeader::SIZE + DNSQuestion::ESTIMATE_SIZE
    }

    pub fn into_raw_data(self) -> RawData {
        let mut data = RawData::with_capacity(self.estimate_size());
        data.append_dns_header(&self.header);
        match self.question {
            QuestionBody::Single(question) => data.append_dns_question(&question),
            QuestionBody::Multi(questions) => {
                for question in questions {
                    data.append_dns_question(&question);
                }
            }
        };
        data
    }

    pub fn get_raw_data(&self) -> RawData {
        let mut data = RawData::with_capacity(self.estimate_size());
        data.append_dns_header(&self.header);
        match &self.question {
            QuestionBody::Single(question) => data.append_dns_question(question),
            QuestionBody::Multi(questions) => {
                for question in questions {
                    data.append_dns_question(question)
                }
            }
        };
        data
    }

    fn from_data(data: &[u8]) -> DNSRequest {
        let reader = &mut SliceReader::from(data);
        let header = DNSHeader::from_reader(reader);
        let mut map: HashMap<u16, Rc<Domain>> = HashMap::new();
        let question = QuestionBody::from_reader(reader, &mut map, header.QDCOUNT);
        DNSRequest { header, question, map }
    }
}
