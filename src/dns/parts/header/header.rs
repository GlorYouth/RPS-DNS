use crate::dns::parts::base::*;
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DNSHeader {
    // 请求会随机生成一个ID，对该请求的响应必须返回同样的ID。这是为了在无状态的UDP协议中区分不同的响应。
    pub ID: u16,

    pub FLAGS: Flags,

    // Question的数量
    pub QDCOUNT: u16,

    // Answer的数量
    pub ANCOUNT: u16,

    // Authority Section的数量
    pub NSCOUNT: u16,

    // Additional Section的数量
    pub ARCOUNT: u16,
}

impl DNSHeader {
    pub const SIZE: usize = size_of::<u8>() * 12;
}

impl DNSHeader {
    pub fn from_reader(reader: &mut SliceReader) -> DNSHeader {
        DNSHeader {
            ID: reader.read_u16(),
            FLAGS: Flags::from_reader(reader),
            QDCOUNT: reader.read_u16(),
            ANCOUNT: reader.read_u16(),
            NSCOUNT: reader.read_u16(),
            ARCOUNT: reader.read_u16(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_from_reader() {
        let reader = &mut SliceReader::from(
            &[
                0x00, 0x03, 0x81, 0x80, 0x00, 0x01, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00,
            ][..],
        );
        let header = DNSHeader::from_reader(reader);
        assert_eq!(header.ID, 0x0003);
        assert_eq!(header.QDCOUNT, 0x0001);
        assert_eq!(header.ANCOUNT, 0x000b);
        assert_eq!(header.NSCOUNT, 0x0000);
        assert_eq!(header.ARCOUNT, 0x0000);
    }
}
