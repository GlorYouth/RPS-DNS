#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::utils::SliceReader;
#[cfg(feature = "logger")]
use log::{debug, trace};
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub struct RawDomain {
    domain: Vec<u8>, //不包含最后的0x0
    raw_len: usize,
}
const SUFFIX: &[u8] = "xn--".as_bytes();
impl RawDomain {
    pub fn as_ref(&self) -> &Vec<u8> {
        &self.domain
    }

    pub fn from_str<T: AsRef<str>>(s: T) -> Option<RawDomain> {
        //不带0x0
        let s = s.as_ref();
        let vec = s
            .split('.')
            .try_fold(Vec::with_capacity(20), |mut v: Vec<u8>, str| {
                if str.is_ascii() {
                    v.push(str.len() as u8);
                    v.extend_from_slice(str.as_bytes());
                } else {
                    match punycode::encode(str) {
                        Ok(s) => {
                            let mut len = SUFFIX.len() as u8;
                            let bytes = s.as_bytes();
                            len += bytes.len() as u8;
                            v.push(len);
                            v.extend_from_slice(SUFFIX);
                            v.extend_from_slice(bytes);
                        }
                        Err(_) => return None,
                    }
                }
                Some(v)
            })?;
        let len = vec.len();
        Some(RawDomain {
            domain: vec,
            raw_len: len,
        })
    }

    pub fn from_reader(reader: &mut SliceReader) -> Option<RawDomain> {
        let mut domain = Vec::with_capacity(30);
        let mut pos = reader.pos();
        loop {
            let first_u8 = reader.read_u8();
            if first_u8 & 0b1100_0000_u8 == 0b1100_0000_u8 {
                #[cfg(feature = "logger")]
                {
                    trace!("发现有Domain Pointer");
                }
                let offset =
                    u16::from_be_bytes([first_u8 & 0b0011_1111_u8, reader.read_u8()]) as usize;
                #[cfg(feature = "logger")]
                {
                    trace!("其指向字节为:{:x}", offset);
                }
                if reader.pos() > pos {
                    pos = reader.pos();
                }
                reader.set_pos(offset);
                continue;
            }
            if first_u8 == 0x0_u8 {
                if reader.pos() > pos {
                    pos = reader.pos();
                }
                break;
            }
            #[cfg(feature = "logger")]
            {
                trace!("普通的Tags,内含{}个ASCII", first_u8);
            }
            if first_u8 as usize + reader.pos() > reader.len() {
                #[cfg(feature = "logger")]
                {
                    trace!(
                        "Tags: {} 超出剩余数组{}",
                        first_u8,
                        reader.len() - reader.pos()
                    );
                }
                return None;
            }
            domain.push(first_u8);
            domain.extend_from_slice(reader.read_slice(first_u8 as usize));
            if reader.pos() > pos {
                pos = reader.pos();
            }
        }
        if domain.is_empty() {
            #[cfg(feature = "logger")]
            {
                debug!("DomainName没有长度");
            }
            return None; //防止无长度的域名
        }
        reader.set_pos(pos);
        let len = domain.len();
        Some(RawDomain {
            domain,
            raw_len: len + 1,
        })
    }

    pub fn from_reader_with_size(reader: &mut SliceReader, size: usize) -> Option<RawDomain> {
        let mut domain = Vec::with_capacity(30);
        let mut slice = reader.read_slice(size);
        while slice.len() > 0 {
            let first_u8 = slice[0];
            if first_u8 & 0b1100_0000_u8 == 0b1100_0000_u8 {
                #[cfg(feature = "logger")]
                {
                    trace!("发现有Domain Pointer");
                }
                let offset = u16::from_be_bytes([first_u8 & 0b0011_1111_u8, slice[1]]) as usize;
                #[cfg(feature = "logger")]
                {
                    trace!("其指向字节为:{:x}", offset);
                }
                let arr = &reader.as_ref()[offset..];
                if let Some(pos) = arr.iter().position(|b| *b == 0x0) {
                    slice = &arr[..pos];
                    continue;
                } else {
                    #[cfg(feature = "logger")]
                    {
                        debug!("并没有raw_message如下offset后找到b'0' {}", offset);
                    }
                    return None;
                }
            }
            if first_u8 == 0x0_u8 {
                //有概率最后一个为0x0,看不同server是如何实现的，这里是为了效率加了判断
                break;
            }
            #[cfg(feature = "logger")]
            {
                trace!("普通的Tags,内含{}个ASCII", slice[0]);
            }
            domain.extend_from_slice(slice[0..(first_u8 as usize) + 1].as_ref());
            slice = &slice[(first_u8 as usize) + 1..];
        }

        if domain.is_empty() {
            #[cfg(feature = "logger")]
            {
                debug!("DomainName没有长度");
            }
            return None; //防止无长度的域名
        }
        Some(RawDomain {
            domain,
            raw_len: size,
        })
    }

    pub fn to_string(&self) -> Option<String> {
        let mut string = String::with_capacity(40);
        let mut remaining = self.domain.as_slice();

        while !remaining.is_empty() {
            let part_length = remaining[0] as usize;
            remaining = &remaining[1..];

            let part_bytes = &remaining[..part_length];

            // 处理内容
            if part_bytes.starts_with(b"xn--") {
                // Punycode 编码的部分，解码
                let input = std::str::from_utf8(&part_bytes[4..]).unwrap(); // 去掉 'xn--' 前缀
                match punycode::decode(input) {
                    Ok(decoded_part) => {
                        string.push_str(&decoded_part);
                    }
                    Err(_) => {
                        #[cfg(feature = "logger")]
                        {
                            debug!("punycode解码失败, 解码输入的为 {}", input);
                        }
                        return None;
                    }
                }
            } else {
                for byte in part_bytes {
                    if byte.is_ascii() {
                        string.push(*byte as char);
                    } else {
                        #[cfg(feature = "logger")]
                        {
                            debug!("domain内有非ASCII字符:{},{}", *byte as char, *byte);
                        }
                        return None;
                    }
                }
            }

            if part_length != remaining.len() {
                string.push('.');
            }
            remaining = &remaining[part_length..];
        }
        Some(string)
    }

    pub fn raw_len(&self) -> usize {
        self.domain.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_reader() {
        let reader = &mut SliceReader::from_slice(&[
            3, 119, 119, 119, 5, 98, 97, 105, 100, 117, 3, 99, 111, 109, 0,
        ]);
        let domain = RawDomain::from_reader(reader);
        assert_eq!(
            domain.unwrap().to_string().unwrap(),
            "www.baidu.com".to_string()
        );
        assert_eq!(reader.pos(), reader.as_ref().len());
        let reader = &mut SliceReader::from_slice(&[
            0xb9, 0xde, 0x80, 0x80, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x03, 0x77,
            0x77, 0x77, 0x05, 0x62, 0x61, 0x69, 0x64, 0x75, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00,
            0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1d, 0x00,
            0x0f, 0x03, 0x77, 0x77, 0x77, 0x01, 0x61, 0x06, 0x73, 0x68, 0x69, 0x66, 0x65, 0x6e,
            0xc0, 0x16, 0xc0, 0x2b, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x04,
            0xb7, 0x02, 0xac, 0xb9, 0xc0, 0x2b, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1d,
            0x00, 0x04, 0xb7, 0x02, 0xac, 0x2a,
        ]);

        reader.set_pos(31);
        let domain = RawDomain::from_reader(reader);
        assert_eq!(
            domain.unwrap().to_string().unwrap(),
            "www.baidu.com".to_string()
        );
        assert_eq!(reader.pos(), 33);
        reader.set_pos(43);
        let domain = RawDomain::from_reader_with_size(reader, 15).unwrap();
        assert_eq!(domain.to_string().unwrap(), "www.a.shifen.com".to_string());
        assert_eq!(reader.pos(), 43 + 15);
    }

    #[test]
    fn test_from_str() {
        let domain = RawDomain::from_str("www.baidu.com").unwrap();
        assert_eq!(domain.to_string().unwrap(), "www.baidu.com".to_string());
    }
}
