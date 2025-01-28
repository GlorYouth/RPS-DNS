use crate::dns::component::small_parts::base::tools::addr_read::AddrReaderError::MismatchVecLen;
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
    pub fn from_vec(v: Vec<u8>, addr_type: u16) -> Result<AddrReader, AddrReaderError> {
        match addr_type {
            1 => {
                if v.len() != 4 {
                    Err(MismatchVecLen {
                        expected: 4,
                        actual: v.len(),
                    })?
                }
                Ok(AddrReader { vec: v })
            }
            28 => {
                if v.len() != 16 {
                    Err(MismatchVecLen {
                        expected: 16,
                        actual: v.len(),
                    })?
                }
                Ok(AddrReader { vec: v })
            }
            _ => Err(AddrReaderError::UnknownAddrType {
                addr_type: addr_type as usize,
            }),
        }
    }

    #[inline]
    pub fn from_reader_ret_err_ipv4(reader: &mut SliceReader) -> AddrReader {
        Self {
            vec: reader.read_slice(4).to_vec(),
        }
    }

    #[inline]
    pub fn from_reader_ret_err_ipv6(reader: &mut SliceReader) -> AddrReader {
        Self {
            vec: reader.read_slice(16).to_vec(),
        }
    }

    pub fn get_addr(&self) -> AddrType {
        //此函数不应有运行时错误
        match self.vec.len() {
            4 => AddrType::Ipv4(std::net::Ipv4Addr::from(
                <[u8; 4]>::try_from(&self.vec[..]).unwrap(),
            )),
            16 => AddrType::Ipv6(std::net::Ipv6Addr::from(
                <[u8; 16]>::try_from(&self.vec[..]).unwrap(),
            )),
            _ => {
                panic!()
            }
        }
    }
}

pub enum AddrReaderError {
    
    UnknownAddrType { addr_type: usize },
    
    MismatchVecLen { expected: usize, actual: usize },
}

impl std::fmt::Display for AddrReaderError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Debug for AddrReaderError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ipv4() {
        let reader =
            AddrReader::from_reader_ret_err_ipv4(&mut SliceReader::from(&[0x3d, 0xf0, 0xdc, 0x06][..]));
        assert_eq!(reader.vec, &[61, 240, 220, 6]);
    }

    #[test]
    fn test_from_ipv6() {
        let reader = AddrReader::from_reader_ret_err_ipv6(&mut SliceReader::from(
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
