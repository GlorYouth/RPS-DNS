#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use crate::dns::component::*;
use std::collections::HashMap;
use std::rc::Rc;
use snafu::ResultExt;
use crate::dns::error::{DomainSnafu, Error};

#[derive(Debug)]
pub struct DNSAnswer {
    pub header: DNSHeader,
    pub question: QuestionBody,
    pub answer: RecordBody,
    pub authority: RecordBody,
    pub additional: RecordBody,

    pub domain_map: HashMap<u16, Rc<Domain>>,
}

impl DNSAnswer {
    pub fn from_reader(reader: &mut SliceReader) -> Result<DNSAnswer,Error> {
        let mut map = HashMap::with_capacity(5);
        let header = DNSHeader::from_reader(reader);
        let question = QuestionBody::from_reader(reader, &mut map, header.QDCOUNT).context(ReadSnafu).context(DomainSnafu)?;
        let answer = RecordBody::from_reader(reader, &mut map, header.ANCOUNT)?;
        let authority = RecordBody::from_reader(reader, &mut map, header.NSCOUNT)?;
        let additional = RecordBody::from_reader(reader, &mut map, header.ARCOUNT)?;
        Ok(DNSAnswer {
            header,
            question,
            answer,
            authority,
            additional,
            domain_map: map,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_answer() {
        let reader = &mut SliceReader::from(
            &[
                0xa8, 0xe1, 0x81, 0x80, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x04, 0x6f,
                0x63, 0x73, 0x70, 0x07, 0x73, 0x65, 0x63, 0x74, 0x69, 0x67, 0x6f, 0x03, 0x63, 0x6f,
                0x6d, 0x00, 0x00, 0x1c, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00,
                0x0e, 0x06, 0x00, 0x26, 0x04, 0x6f, 0x63, 0x73, 0x70, 0x08, 0x63, 0x6f, 0x6d, 0x6f,
                0x64, 0x6f, 0x63, 0x61, 0x03, 0x63, 0x6f, 0x6d, 0x03, 0x63, 0x64, 0x6e, 0x0a, 0x63,
                0x6c, 0x6f, 0x75, 0x64, 0x66, 0x6c, 0x61, 0x72, 0x65, 0x03, 0x6e, 0x65, 0x74, 0x00,
                0xc0, 0x2e, 0x00, 0x1c, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2c, 0x00, 0x10, 0x26, 0x06,
                0x47, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x12, 0x26, 0xe9,
                0xc0, 0x2e, 0x00, 0x1c, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2c, 0x00, 0x10, 0x26, 0x06,
                0x47, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xac, 0x40, 0x95, 0x17,
                0x0,
            ][..],
        );
        let mut answer = DNSAnswer::from_reader(reader).unwrap();
        assert_eq!(answer.header.ID, 0xa8e1);
        let flags = answer.header.FLAGS.resolve();
        assert_eq!(flags.QR, 1);
        assert_eq!(flags.Opcode, 0);
        assert_eq!(flags.AA, 0);
        assert_eq!(flags.TC, 0);
        assert_eq!(flags.RD, 1);
        assert_eq!(flags.RA, 1);
        assert_eq!(flags.Z, 0);
        assert_eq!(flags.RCODE, 0);
        assert_eq!(answer.header.QDCOUNT, 1);
        assert_eq!(answer.header.ANCOUNT, 3);
        assert_eq!(answer.header.NSCOUNT, 0);
        assert_eq!(answer.header.ARCOUNT, 0);
        // todo test 多record类型合成
    }
}
