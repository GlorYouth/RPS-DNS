use std::collections::HashMap;
use std::rc::Rc;
use crate::dns::parts::base::*;

#[derive(Debug)]
pub struct RecordData(pub Vec<u8>);

impl RecordData {
    pub const ESTIMATE_SIZE: usize = Domain::ESTIMATE_DOMAIN_SIZE;

    pub fn with_capacity(capacity: usize) -> RecordData {
        RecordData(Vec::with_capacity(capacity))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clone(&self) -> RecordData {
        RecordData(self.0.clone())
    }

    pub fn from_reader(reader: &mut SliceReader,map: &mut HashMap<u16, Rc<Domain>>, rtype: u16) -> RecordData {
        match rtype {
            1 => RecordData(addr_read::from_ipv4(reader)),
            5 => {
                let result = Domain::from_reader_and_check_map(reader,map);
                RecordData(result.0.clone())
            }
            28 => RecordData(addr_read::from_ipv6(reader)),
            _ => {
                panic!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_ipv4() {
        let map = &mut HashMap::new();
        assert_eq!(
            RecordData::from_reader(
                &mut SliceReader::from(&[61, 240, 220, 6][..]),
                map,
                DNSType::to_u16(&DNSType::A)
            )
            .0,
            &[61, 240, 220, 6]
        )
    }

    #[test]
    fn test_read_ipv6() {
        let map = &mut HashMap::new();
        assert_eq!(
            RecordData::from_reader(
                &mut SliceReader::from(
                    &[
                        0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x59
                    ][..]
                ),
                map,
                DNSType::to_u16(&DNSType::AAAA)
            )
            .0,
            &[
                0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x59
            ]
        )
    }

    #[test]
    fn test_read_cname() {
        let map = &mut HashMap::new();
        assert_eq!(
            RecordData::from_reader(
                &mut SliceReader::from(
                    &[
                        0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65,
                        0x0a, 0x78, 0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
                    ][..]
                ),
                map,
                DNSType::to_u16(&DNSType::CNAME)
            )
            .0,
            &[
                0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a, 0x78,
                0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
            ]
        );
        // todo test c00c类压缩包
    }
}
