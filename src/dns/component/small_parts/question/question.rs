#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::component::small_parts::base::*;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct DNSQuestion {
    pub QNAME: Rc<Domain>, //HashMap和Name均需要用引用
    pub QTYPE: u16,
    pub QCLASS: u16,
}

impl DNSQuestion {
    pub const ESTIMATE_SIZE: usize = Domain::ESTIMATE_DOMAIN_SIZE + 4;

    pub fn get_size(&self) -> usize {
        self.QNAME.len() + 4
    }

    pub fn get_domain(&self) -> Result<String, DomainDecodeError> {
        self.QNAME.to_string()
    }
    pub fn get_domain_uncheck(&self) -> String {
        self.QNAME.to_string_uncheck()
    }
}

impl DNSQuestion {
    pub fn from_reader(reader: &mut SliceReader, map: &mut HashMap<u16, Rc<Domain>>) -> Result<DNSQuestion, DomainReadError> {
        Ok(DNSQuestion {
            QNAME: Domain::from_reader_and_check_map(reader, map)?,
            QTYPE: reader.read_u16(),
            QCLASS: reader.read_u16(),
        })
    }

    pub fn from_reader_uncheck(reader: &mut SliceReader, map: &mut HashMap<u16, Rc<Domain>>) -> DNSQuestion {
        DNSQuestion {
            QNAME: Domain::from_reader_and_check_map_uncheck(reader, map),
            QTYPE: reader.read_u16(),
            QCLASS: reader.read_u16(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_reader() {
        let slice = &[
            0x03, 0x69, 0x70, 0x77, 0x02, 0x63, 0x6e, 0x00, 0x00, 0x1c, 0x00, 0x01,
        ];
        let mut map: HashMap<u16, Rc<Domain>> = HashMap::new();
        let reader = &mut SliceReader::from(&slice[..]);
        let question = DNSQuestion::from_reader(reader, &mut map).unwrap();
        assert_eq!(
            question.QNAME,
            Domain::from_reader_and_check_map(&mut SliceReader::from(&slice[..]), &mut map).unwrap()
        );
        assert_eq!(question.QTYPE, DNSType::AAAA.to_u16());
        assert_eq!(question.QCLASS, 0x1)
    }
}
