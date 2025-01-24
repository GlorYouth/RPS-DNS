use crate::*;

pub type Addr = Vec<u8>;

pub fn from_ipv4(reader: &mut SliceReader) -> Addr {
    reader.read_slice(4).to_vec()
}

pub fn from_ipv6(reader: &mut SliceReader) -> Addr {
    reader.read_slice(16).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ipv4() {
        let slice = from_ipv4(&mut SliceReader::from(&[0x3d, 0xf0, 0xdc, 0x06][..]));
        assert_eq!(slice, &[61, 240, 220, 6]);
    }

    #[test]
    fn test_from_ipv6() {
        let slice = from_ipv6(&mut SliceReader::from(&[
            0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x59,
        ][..]));
        assert_eq!(
            slice,
            &[
                0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x59
            ]
        )
    }
}
