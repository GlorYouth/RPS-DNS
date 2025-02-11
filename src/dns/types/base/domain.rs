#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::utils::SliceReader;
#[cfg(feature = "logger")]
use log::{debug, trace};
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct RawDomain {
    domain: Vec<u8>, //不包含最后的0x0
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
                if str.len() == 0 {
                    return Some(v);
                }
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
        Some(RawDomain { domain: vec })
    }

    // 主解析逻辑
    #[inline(always)]
    fn parse_labels<F>(reader: &mut SliceReader, handle_label: F) -> Option<(Vec<u8>, u8)>
    where
        F: Fn(usize) -> bool, // 返回false表示需要终止解析
    {
        let mut domain = Vec::with_capacity(30);
        let mut max_pos = reader.pos();
        let mut visited_num = 0x1_u32; //用移位判断循环次数

        loop {
            let start_pos = reader.pos();
            visited_num = visited_num << 1;
            // 防止指针循环
            if visited_num == 0 {
                #[cfg(feature = "logger")]
                trace!("检测到循环指针");
                return None;
            }

            if !handle_label(reader.pos()) {
                break; //记得后面要检查domain大小
            }

            let label_len = reader.peek_u8();

            // 处理指针
            if (label_len & 0b1100_0000_u8) == 0b1100_0000_u8 {
                let pointer_pos = reader.read_u16() as usize & 0x_3FFF;
                if pointer_pos > reader.len() {
                    #[cfg(feature = "logger")]
                    trace!("parse_labels中处理指针时出界");
                    return None;
                }
                max_pos = max_pos.max(reader.pos());
                reader.set_pos(pointer_pos);
                continue;
            }
            reader.skip(1);

            // 处理结束标记
            if label_len == 0 {
                max_pos = max_pos.max(reader.pos());
                break;
            }

            // 处理普通标签
            let end_pos = reader.pos() + label_len as usize;
            if end_pos > reader.len() {
                #[cfg(feature = "logger")]
                trace!("parse_labels中处理普通标签时出界");
                return None;
            }

            domain.extend_from_slice(&reader.as_ref()[start_pos..end_pos]);
            max_pos = max_pos.max(end_pos);
            reader.set_pos(end_pos);
        }

        Some((domain, max_pos as u8))
    }

    // 优化后的两个公开函数
    pub fn from_reader(reader: &mut SliceReader) -> Option<RawDomain> {
        let len = reader.len();

        let (domain, max_pos) = Self::parse_labels(reader, |current_pos| current_pos < len)?;

        reader.set_pos(max_pos as usize);
        Some(RawDomain { domain })
    }

    pub fn from_reader_with_size(reader: &mut SliceReader, size: usize) -> Option<RawDomain> {
        let start_pos = reader.pos();
        let end_pos = start_pos + size;

        if end_pos > reader.len() {
            #[cfg(feature = "logger")]
            debug!("读取RDATA时出界");
            return None;
        }

        let (domain, _) = Self::parse_labels(reader, |current_pos| current_pos < end_pos)?;

        reader.set_pos(end_pos);
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
        if string.is_empty() {
            string.push('.');
        }
        Some(string)
    }
}

impl Display for RawDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().unwrap_or("???".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "logger")]
    use crate::dns::error::init_logger;

    #[test]
    fn test_from_reader() {
        #[cfg(feature = "logger")]
        init_logger();
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
