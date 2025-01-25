use crate::*;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug)]
pub enum AddrType {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
}

#[derive(Clone, Debug)]
pub struct AddrReader {
    pub vec: Vec<u8>,
}

impl AddrReader {
    #[inline]
    pub fn from_reader_ipv4(reader: &mut SliceReader) -> AddrReader {
        Self {
            vec: reader.read_slice(4).to_vec(),
        }
    }

    #[inline]
    pub fn from_reader_ipv6(reader: &mut SliceReader) -> AddrReader {
        Self {
            vec: reader.read_slice(16).to_vec(),
        }
    }

    pub fn get_addr(&mut self) -> AddrType {
        match self.vec.len() {
            4 => AddrType::Ipv4(std::net::Ipv4Addr::from(
                <[u8; 4]>::try_from(&self.vec[..4]).unwrap(),
            )),
            16 => AddrType::Ipv6(std::net::Ipv6Addr::from(
                <[u8; 16]>::try_from(&self.vec[..16]).unwrap(),
            )),
            _ => {
                panic!("Unsupported address type");
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ipv4() {
        let reader =
            AddrReader::from_reader_ipv4(&mut SliceReader::from(&[0x3d, 0xf0, 0xdc, 0x06][..]));
        assert_eq!(reader.vec, &[61, 240, 220, 6]);
    }

    #[test]
    fn test_from_ipv6() {
        let reader = AddrReader::from_reader_ipv6(&mut SliceReader::from(
            &[
                0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x59,
            ][..],
        ));
        assert_eq!(
            reader.vec,
            &[
                0x24, 0x08, 0x87, 0x52, 0x0e, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x59
            ]
        )
    }
}
