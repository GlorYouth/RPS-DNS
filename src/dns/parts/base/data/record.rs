use crate::dns::parts::base::*;
use crate::RecordDataType::NotResolved;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
#[derive(Clone)]
pub enum RecordDataType {
    A(std::net::Ipv4Addr),
    CNAME(Rc<Domain>),
    AAAA(std::net::Ipv6Addr),
    NotResolved,
}

#[derive(Debug)]
pub struct RecordData {
    pub vec: Vec<u8>,
    pub rtype: RecordDataType,
    pub rtype_u16: u16,
}

impl RecordData {
    pub const ESTIMATE_SIZE: usize = Domain::ESTIMATE_DOMAIN_SIZE;

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn from_reader(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
        rtype: u16,
    ) -> RecordData {
        match rtype {
            1 => RecordData {
                vec: addr_read::from_ipv4(reader),
                rtype: NotResolved,
                rtype_u16: rtype,
            },
            5 => {
                let result = Domain::from_reader_and_check_map(reader, map);
                RecordData {
                    vec: result.0.clone(),
                    rtype: RecordDataType::CNAME(result),
                    rtype_u16: rtype,
                }
            }
            28 => RecordData {
                vec: addr_read::from_ipv6(reader),
                rtype: NotResolved,
                rtype_u16: rtype,
            },
            _ => {
                panic!()
            }
        }
    }

    pub fn resolve(&self) -> RecordDataType {
        match self.rtype_u16 {
            1 => RecordDataType::A(std::net::Ipv4Addr::from(
                <[u8;4]>::try_from(&self.vec[..4]).unwrap()
            )),
            5 => self.rtype.to_owned(),
            28 => RecordDataType::AAAA(std::net::Ipv6Addr::from(
                <[u8;16]>::try_from(&self.vec[..16]).unwrap()
            )),
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
            .vec,
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
            .vec,
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
            .vec,
            &[
                0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a, 0x78,
                0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
            ]
        );
        // todo test c00c类压缩包
    }
}
