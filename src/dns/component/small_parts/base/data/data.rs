#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::component::*;

pub struct RawData(Vec<u8>);

pub trait Append<T> {
    fn append(&mut self, data: &T);
}

impl Append<DNSHeader> for RawData {
    fn append(&mut self, header: &DNSHeader) {
        self.0.extend_from_slice(&header.ID.to_be_bytes());
        self.0.extend_from_slice(&header.FLAGS.to_vec());

        self.0.extend_from_slice(&header.QDCOUNT.to_be_bytes());
        self.0.extend_from_slice(&header.ANCOUNT.to_be_bytes());
        self.0.extend_from_slice(&header.NSCOUNT.to_be_bytes());
        self.0.extend_from_slice(&header.ARCOUNT.to_be_bytes());
    }
}

impl Append<DNSQuestion> for RawData {
    fn append(&mut self, question: &DNSQuestion) {
        self.0.extend_from_slice(question.QNAME.as_ref().as_ref());
        self.0.extend_from_slice(&question.QTYPE.to_be_bytes());
        self.0.extend_from_slice(&question.QCLASS.to_be_bytes());
    }
}

impl Append<DNSRecord> for RawData {
    fn append(&mut self, record: &DNSRecord) {
        self.0.extend_from_slice(&record.NAME.as_ref().as_ref());
        self.0.extend_from_slice(&record.TYPE.to_be_bytes());
        self.0.extend_from_slice(&record.CLASS.to_be_bytes());
        self.0.extend_from_slice(&record.TTL.to_be_bytes());
        self.0.extend_from_slice(&record.RDLENGTH.to_be_bytes());
        self.0.append(&mut record.RDATA.to_bytes());
    }
}

impl RawData {
    pub fn with_capacity(capacity: usize) -> RawData {
        RawData(Vec::with_capacity(capacity))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    fn test_raw_data_append_dns_header() {
        let mut data = RawData::with_capacity(DNSHeader::SIZE);
        data.append(&DNSHeader {
            ID: 0x8ac8_u16,
            FLAGS: FlagsData::from(&[0x1, 0x0][..]),
            QDCOUNT: 1,
            ANCOUNT: 0,
            NSCOUNT: 0,
            ARCOUNT: 0,
        });
        assert_eq!(data.0, [138, 200, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_raw_data_append_dns_question() {
        let mut data = RawData::with_capacity(DNSQuestion::ESTIMATE_SIZE);
        let question = DNSQuestion {
            QNAME: Rc::from(Domain::from("www.google.com")),
            QTYPE: DNSType::A.to_u16(),
            QCLASS: 1,
        };
        data.append(&question);
        assert_eq!(
            data.0,
            [3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1]
        );
        assert_eq!(
            question,
            DNSQuestion {
                QNAME: Rc::from(Domain::from("www.google.com")),
                QTYPE: DNSType::A.to_u16(),
                QCLASS: 1,
            }
        );
    }

    #[test]
    fn test_raw_data_append_dns_record() {
        let mut data = RawData::with_capacity(DNSRecord::ESTIMATED_SIZE);
        let mut map = HashMap::new();
        data.append(&DNSRecord {
            NAME: Rc::new(Domain::from("www.google.com")),
            TYPE: DNSType::A.to_u16(),
            CLASS: 1,
            TTL: 2,
            RDLENGTH: 0,
            RDATA: RecordData::from_reader_ret_err(
                &mut SliceReader::from_slice(&[1, 1, 1, 1]),
                &mut map,
                DNSType::A.to_u16(),
            )
            .unwrap(),
        });

        assert_eq!(
            data.0,
            [
                3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1,
                0, 0, 0, 2, 0, 0, 1, 1, 1, 1
            ]
        )
    }
}
