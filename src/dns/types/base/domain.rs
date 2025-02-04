#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::utils::SliceReader;
use log::{debug, trace};
use smallvec::SmallVec;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub struct RawDomain {
    domain: SmallVec<[u8; 30]>, //不包含最后的0x0
}

impl RawDomain {
    pub fn from_reader(reader: &mut SliceReader) -> Option<RawDomain> {
        let mut domain = SmallVec::new();
        let mut pos = reader.pos();
        loop {
            let first_u8 = reader.read_u8();
            if first_u8 & 0b1100_0000_u8 == 0b1100_0000_u8 {
                #[cfg(debug_assertions)]
                {
                    trace!("发现有Domain Pointer");
                }
                let offset = reader.read_u8() as usize;
                #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
            {
                trace!("发现是普通的域名");
            }
            domain.push(first_u8);
            domain.extend_from_slice(reader.read_slice(first_u8 as usize));
            if reader.pos() > pos {
                pos = reader.pos();
            }
        }
        if domain.is_empty() {
            #[cfg(debug_assertions)]
            {
                debug!("DomainName没有长度");
            }
            return None; //防止无长度的域名
        }
        reader.set_pos(pos);
        Some(RawDomain { domain })
    }

    pub fn from_reader_with_size(reader: &mut SliceReader, size: usize) -> Option<RawDomain> {
        let mut domain = SmallVec::new();
        let mut slice = reader.read_slice(size);
        while slice.len() > 0 {
            let first_u8 = slice[0];
            if first_u8 & 0b1100_0000_u8 == 0b1100_0000_u8 {
                #[cfg(debug_assertions)]
                {
                    trace!("发现有Domain Pointer");
                }
                let offset = slice[1] as usize;
                #[cfg(debug_assertions)]
                {
                    trace!("其指向字节为:{:x}", offset);
                }
                let arr = &reader.as_ref()[offset..];
                if let Some(pos) = arr.iter().position(|b| *b == 0x0) {
                    slice = &arr[..pos];
                    continue;
                } else {
                    #[cfg(debug_assertions)]
                    {
                        debug!("并没有raw_message如下offset后找到b'0' {}", offset);
                    }
                    return None;
                }
                break;
            }
            #[cfg(debug_assertions)]
            {
                trace!("发现是普通的域名");
            }
            domain.extend_from_slice(slice[0..(first_u8 as usize) + 1].as_ref());
            slice = &slice[(first_u8 as usize) + 1..];
        }

        if domain.is_empty() {
            #[cfg(debug_assertions)]
            {
                debug!("DomainName没有长度");
            }
            return None; //防止无长度的域名
        }
        reader.skip(size);
        Some(RawDomain { domain })
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
                        #[cfg(debug_assertions)]
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
                        #[cfg(debug_assertions)]
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
        reader.set_pos(43);
        let domain = RawDomain::from_reader_with_size(reader,15).unwrap();
        assert_eq!(domain.to_string().unwrap(), "www.a.shifen.com".to_string());
        assert_eq!(reader.pos(), 43+15);
    }
}
