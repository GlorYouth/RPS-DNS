use crate::dns::parts::base::*;

pub type RecordData = Vec<u8>;

pub fn from_reader(reader: &mut SliceReader, rtype: u16) -> RecordData {
    match rtype {
        1 => addr_read::from_ipv4(reader),
        5 => {
            let result = Domain::from(reader);
            result.0
        }
        28 => addr_read::from_ipv6(reader),
        _ => {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_ipv4() {
        assert_eq!(
            from_reader(
                &mut SliceReader::from(&[61, 240, 220, 6][..]),
                DNSType::to_u16(&DNSType::A)
            ),
            &[61, 240, 220, 6]
        )
    }

    #[test]
    fn test_read_ipv6() {
        assert_eq!(
            from_reader(
                &mut SliceReader::from(&[
                    0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x59
                ][..]),
                DNSType::to_u16(&DNSType::AAAA)
            ),
            &[
                0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x59
            ]
        )
    }

    #[test]
    fn test_read_cname() {
        assert_eq!(
            from_reader(
                &mut SliceReader::from(&[
                    0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a,
                    0x78, 0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
                ][..]),
                DNSType::to_u16(&DNSType::CNAME)
            ),
            &[
                0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a, 0x78,
                0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
            ]
        );
    }
}
