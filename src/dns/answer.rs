#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use crate::dns::parts::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct DNSAnswer {
    header: DNSHeader,
    question: QuestionBody,
    answer: RecordBody,
    authority: RecordBody,
    additional: RecordBody,

    domain_map: HashMap<u16, Rc<Domain>>,
}

impl DNSAnswer {
    pub fn from_reader(reader: &mut SliceReader) -> DNSAnswer {
        let mut map = HashMap::with_capacity(5);
        let header = DNSHeader::from_reader(reader);
        let question = QuestionBody::from_reader(reader, &mut map, header.QDCOUNT);
        let answer = RecordBody::from_reader(reader, &mut map, header.ANCOUNT);
        let authority = RecordBody::from_reader(reader, &mut map, header.NSCOUNT);
        let additional = RecordBody::from_reader(reader, &mut map, header.ARCOUNT);
        DNSAnswer {
            header,
            question,
            answer,
            authority,
            additional,
            domain_map: map,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_answer() {
        let reader = &mut SliceReader::from(&[
            0xb4_u8, 0xdb, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x03, 0x64,
            0x6e, 0x73, 0x06, 0x77, 0x65, 0x69, 0x78, 0x69, 0x6e, 0x02, 0x71, 0x71, 0x03, 0x63,
            0x6f, 0x6d, 0x02, 0x63, 0x6e, 0x00, 0x00, 0x1c, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x1c,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00, 0x02,
            0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0xc0, 0x0c, 0x00, 0x1c,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00, 0x10,
            0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0xc0, 0x0c, 0x00, 0x1c,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00, 0x02,
            0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0xc0, 0x0c, 0x00, 0x1c,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00, 0x10,
            0x10, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25,
        ][..]);
        let mut answer = DNSAnswer::from_reader(reader);
        println!("{:?}", answer);
    }
}
