use crate::dns::parts::base::*;
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::rc::Rc;

#[derive(Debug, Clone)]
enum RecordDataType {
    A(AddrReader),
    CNAME(Rc<Domain>),
    AAAA(AddrReader),
}

#[derive(Debug, PartialEq)]
pub enum RecordResolvedType {
    Ipv4(Ipv4Addr),
    Domain(String),
    Ipv6(Ipv6Addr),
}

impl From<AddrType> for RecordResolvedType {
    fn from(t: AddrType) -> Self {
        match t {
            AddrType::Ipv4(ipv4) => RecordResolvedType::Ipv4(ipv4),
            AddrType::Ipv6(ipv6) => RecordResolvedType::Ipv6(ipv6),
        }
    }
}

#[derive(Debug)]
pub struct RecordData {
    rtype: RecordDataType,
}

impl RecordData {
    pub const ESTIMATE_SIZE: usize = Domain::ESTIMATE_DOMAIN_SIZE;

    pub fn from_reader(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
        rtype: u16,
    ) -> RecordData {
        match rtype {
            1 => RecordData {
                rtype: RecordDataType::A(AddrReader::from_reader_ipv4(reader)),
            },
            5 => RecordData {
                rtype: RecordDataType::CNAME(Domain::from_reader_and_check_map(reader, map)),
            },
            28 => RecordData {
                rtype: RecordDataType::AAAA(AddrReader::from_reader_ipv6(reader)),
            },
            _ => {
                panic!()
            }
        }
    }
    

    pub fn resolve(&self) -> RecordResolvedType {
        match self.rtype.clone() {
            RecordDataType::A(mut reader) => RecordResolvedType::from(reader.get_addr()),
            RecordDataType::AAAA(mut reader) => RecordResolvedType::from(reader.get_addr()),
            RecordDataType::CNAME(domain) => {
                RecordResolvedType::Domain(domain.to_string().unwrap())
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self.rtype.clone() {
            RecordDataType::A(reader) => reader.vec,
            RecordDataType::CNAME(domain) => domain.0.to_vec(),
            RecordDataType::AAAA(reader) => reader.vec,
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
            .resolve(),
            RecordResolvedType::Ipv4(Ipv4Addr::new(61, 240, 220, 6))
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
            .resolve(),
            RecordResolvedType::Ipv6(Ipv6Addr::from([
                0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x59
            ]))
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
            .resolve(),
            RecordResolvedType::Domain(
                Domain::from(
                    [
                        0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65,
                        0x0a, 0x78, 0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
                    ]
                    .to_vec()
                )
                .to_string()
                .unwrap()
            )
        );
        // todo test c00c类压缩包
    }
}
