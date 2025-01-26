use crate::dns::component::small_parts::base::*;
use crate::dns::error::{AddrSnafu, DomainSnafu, Error};
use snafu::ResultExt;
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
    #[inline]
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
    ) -> Result<RecordData, Error> {
        match rtype {
            1 => Ok(RecordData {
                rtype: RecordDataType::A(AddrReader::from_reader_ipv4(reader)),
            }),
            5 => Ok(RecordData {
                rtype: RecordDataType::CNAME(Domain::from_reader_and_check_map(reader, map).context(ReadSnafu).context(DomainSnafu)?),
            }),
            28 => Ok(RecordData {
                rtype: RecordDataType::AAAA(AddrReader::from_reader_ipv6(reader)),
            }),
            _ => Err(AddrReaderError::UnknownAddrType {
                addr_type: rtype as usize,
            }).context(AddrSnafu),
        }
    }

    pub fn from_vec(vec: Vec<u8>, rtype: u16) -> Result<RecordData, Error> {
        match rtype {
            1 => Ok(RecordData {
                rtype: RecordDataType::A(AddrReader::from_vec(vec, rtype).context(AddrSnafu)?),
            }),
            5 => Ok(RecordData {
                rtype: RecordDataType::CNAME(Rc::from(Domain::from(vec))),
            }),
            28 => Ok(RecordData {
                rtype: RecordDataType::AAAA(AddrReader::from_vec(vec, rtype).context(AddrSnafu)?),
            }),
            _ => Err(AddrReaderError::UnknownAddrType {
                addr_type: rtype as usize,
            }).context(AddrSnafu)?,
        }
    }

    pub fn resolve(&self) -> Result<RecordResolvedType, DomainDecodeError> {
        match self.rtype.clone() {
            RecordDataType::A(mut reader) => Ok(RecordResolvedType::from(reader.get_addr())),
            RecordDataType::AAAA(mut reader) => Ok(RecordResolvedType::from(reader.get_addr())),
            RecordDataType::CNAME(domain) => Ok(RecordResolvedType::Domain(domain.to_string()?)),
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
            .unwrap()
            .resolve()
            .unwrap(),
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
            .unwrap()
            .resolve()
            .unwrap(),
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
            .unwrap()
            .resolve()
            .unwrap(),
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
