#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use crate::dns::component::small_parts::base::data::domain::DomainDecodeError::InputNotAscii;
use crate::*;
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use std::fmt::{Debug, Error};
use std::ops::Add;
use std::rc::Rc;
use std::str::Utf8Error;

const SIZE_OF_XN: usize = "xn--".len();
#[derive(PartialEq, Debug)]
pub struct Domain(pub Vec<u8>);

impl From<&String> for Domain {
    fn from(str: &String) -> Self {
        let mut encoded: Vec<u8> = Vec::with_capacity(Self::ESTIMATE_DOMAIN_SIZE);

        // 遍历域名部分
        for part in str.split('.') {
            // 检查是否含有非 ASCII 字符，如果是，进行 Punycode 编码
            if part.chars().any(|c| !c.is_ascii()) {
                let temp = punycode::encode(part).unwrap();
                encoded.push((temp.len() + SIZE_OF_XN) as u8); // 添加部分长度
                encoded.extend_from_slice("xn--".as_ref());
                encoded.extend_from_slice(temp.as_ref()); // 添加Punycode编码后的字节
            } else {
                encoded.push(part.len() as u8); // 添加部分长度
                encoded.extend_from_slice(part.as_ref()); // 直接添加 ASCII 字符字节
            }
        }

        encoded.push(0);
        Self(encoded)
    }
}

impl From<&str> for Domain {
    fn from(str: &str) -> Self {
        let mut encoded: Vec<u8> = Vec::with_capacity(Self::ESTIMATE_DOMAIN_SIZE);

        // 遍历域名部分
        for part in str.split('.') {
            // 检查是否含有非 ASCII 字符，如果是，进行 Punycode 编码
            if part.chars().any(|c| !c.is_ascii()) {
                let temp = punycode::encode(part).unwrap();
                encoded.push((temp.len() + SIZE_OF_XN) as u8); // 添加部分长度
                encoded.extend_from_slice("xn--".as_ref());
                encoded.extend_from_slice(temp.as_ref()); // 添加Punycode编码后的字节
            } else {
                encoded.push(part.len() as u8); // 添加部分长度
                encoded.extend_from_slice(part.as_ref()); // 直接添加 ASCII 字符字节
            }
        }

        encoded.push(0);
        Self(encoded)
    }
}

impl From<Vec<u8>> for Domain {
    fn from(vec: Vec<u8>) -> Self {
        Domain(vec)
    }
}

impl Domain {
    fn from_reader(reader: &mut SliceReader) -> Result<Self, DomainReadError> {
        if let Some(offset) = reader.iter_from_current_pos().position(|b| *b == 0x0) {
            return Ok(Domain(Vec::from(reader.read_slice(offset + 1))));
        }
        Err(DomainReadError::ReadFailed {
            findings: "0x0_u8".to_string(),
            container_type: "&mut SliceReader".to_string(),
            other_info: "has "
                .to_string()
                .add(format!("{:?}", reader.as_ref()).as_str()),
        })
    }

    fn from_reader_uncheck(reader: &mut SliceReader) -> Self {
        if let Some(offset) = reader.iter_from_current_pos().position(|b| *b == 0x0) {
            return Domain(Vec::from(reader.read_slice(offset + 1)));
        }
        panic!()
    }

    pub fn from_reader_and_check_map(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
    ) -> Result<Rc<Domain>, Box<DomainReadError>> {
        if reader.peek_u8() & 0b1100_0000 == 0b1100_0000 {
            let key = &reader.read_u16();
            let value = map.get_mut(key);
            return if let Some(v) = value {
                Ok(v.clone())
            } else {
                Err(Box::from(DomainReadError::ReadFailed {
                    findings: key.to_string(),
                    container_type: "&mut HashMap<u16, Rc<Domain>>".to_string(),
                    other_info: "has ".to_string().add(format!("{:?}", map).as_str()),
                }))
            };
        }
        if let Some(offset) = reader.iter_from_current_pos().position(|b| *b == 0x0) {
            let pos = reader.pos() as u16;
            let domain = Rc::new(Domain(Vec::from(reader.read_slice(offset + 1))));
            map.insert(pos | 0b1100_0000_0000_0000, domain.clone());
            return Ok(domain);
        }
        Err(Box::from(DomainReadError::ReadFailed {
            findings: "0x0_u8".to_string(),
            container_type: "&mut SliceReader".to_string(),
            other_info: "has "
                .to_string()
                .add(format!("{:?}", reader.as_ref()).as_str()),
        }))
    }

    pub fn from_reader_and_check_map_uncheck(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
    ) -> Rc<Domain> {
        if reader.peek_u8() & 0b1100_0000 == 0b1100_0000 {
            let key = &reader.read_u16();
            let value = map.get_mut(key);
            return if let Some(v) = value {
                v.clone()
            } else {
                panic!()
            };
        }
        if let Some(offset) = reader.iter_from_current_pos().position(|b| *b == 0x0) {
            let pos = reader.pos() as u16;
            let domain = Rc::new(Domain(Vec::from(reader.read_slice(offset + 1))));
            map.insert(pos | 0b1100_0000_0000_0000, domain.clone());
            return domain;
        }
        panic!()
    }
}

impl Domain {
    pub const ESTIMATE_DOMAIN_SIZE: usize = 40;

    pub fn with_capacity(capacity: usize) -> Domain {
        Domain(Vec::with_capacity(capacity))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn new(vec: Vec<u8>) -> Domain {
        Domain(vec)
    }

    pub fn clone(&self) -> Domain {
        Domain(self.0.clone())
    }

    pub fn to_string(&self) -> Result<String, DomainDecodeError> {
        let mut decoded = String::with_capacity(40);
        let mut i = 0;

        while i < self.0.len() - 1 {
            //排除最后的'\0'
            let part_length = self.0[i] as usize;
            i += 1; // 移动到部分内容

            let part_bytes = self.0[i..(i + part_length)].as_ref();
            i += part_length; // 移动到下一部分

            if part_bytes.starts_with(b"xn--") {
                // Punycode 编码的部分，解码
                let input = std::str::from_utf8(&part_bytes[4..]).context(InputNotAsciiSnafu)?; // 去掉 'xn--' 前缀
                match punycode::decode(input) {
                    Ok(decoded_part) => {
                        decoded.push_str(&decoded_part);
                    }
                    Err(_) => {
                        return Err(DomainDecodeError::PunycodeDecode {
                            string: input.to_string(),
                        })
                    }
                }
            } else {
                // 直接是 ASCII 字符部分
                decoded.push_str(&String::from_utf8_lossy(part_bytes));
            }

            // 添加分隔符 "."
            if i < self.0.len() - 1 {
                decoded.push('.');
            }
        }
        Ok(decoded)
    }

    pub fn to_string_uncheck(&self) -> String {
        let mut decoded = String::with_capacity(40);
        let mut i = 0;

        while i < self.0.len() - 1 {
            //排除最后的'\0'
            let part_length = self.0[i] as usize;
            i += 1; // 移动到部分内容

            let part_bytes = self.0[i..(i + part_length)].as_ref();
            i += part_length; // 移动到下一部分

            if part_bytes.starts_with(b"xn--") {
                // Punycode 编码的部分，解码
                let input = std::str::from_utf8(&part_bytes[4..]).unwrap(); // 去掉 'xn--' 前缀
                match punycode::decode(input) {
                    Ok(decoded_part) => {
                        decoded.push_str(&decoded_part);
                    }
                    Err(_) => {
                        panic!()
                    }
                }
            } else {
                // 直接是 ASCII 字符部分
                decoded.push_str(&String::from_utf8_lossy(part_bytes));
            }

            // 添加分隔符 "."
            if i < self.0.len() - 1 {
                decoded.push('.');
            }
        }
        decoded
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DomainError {
    #[snafu(display("Domain error:\n {}", source))]
    ReadError { source: Box<DomainReadError> },

    #[snafu(display("Domain error:\n {}", source))]
    DecodeError { source: Box<DomainDecodeError> },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DomainReadError {
    #[snafu(display(
        "Failed to find {} in {}, which {}",
        findings,
        container_type,
        other_info
    ))]
    ReadFailed {
        findings: String,
        container_type: String,
        other_info: String,
    },
}

#[derive(Debug, Snafu)]
pub enum DomainDecodeError {
    #[snafu(display("Has character that are not ASCII, source: {}", source))]
    InputNotAscii { source: Utf8Error },

    #[snafu(display("Punycode decode error, string: {}", string))]
    PunycodeDecode { string: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_from_str() {
        assert_eq!(
            Domain::from("小米.中国").0,
            [
                0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a, 0x78,
                0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
            ]
        );
        assert_eq!(
            Domain::from("www.google.com").0,
            [3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        )
    }

    #[test]
    fn test_domain_to_string() {
        assert_eq!(
            Domain::new(
                [
                    0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a,
                    0x78, 0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
                ]
                .to_vec()
            )
            .to_string()
            .unwrap(),
            "小米.中国"
        );

        assert_eq!(
            Domain::new(
                [3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0].to_vec()
            )
            .to_string()
            .unwrap(),
            "www.google.com"
        );
    }
    #[test]
    fn test_domain_from_reader() {
        assert_eq!(
            &Domain::from_reader(&mut SliceReader::from(
                &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0][..]
            ))
            .unwrap()
            .to_string()
            .unwrap(),
            "www.google.com"
        );

        assert_eq!(
            &Domain::from_reader(&mut SliceReader::from(
                &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0][..]
            ))
            .unwrap()
            .0,
            &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        )
    }

    #[test]
    fn test_domain_from_reader_for_answer() {
        let mut map: HashMap<u16, Rc<Domain>> = HashMap::new();
        let reader = &mut SliceReader::from(
            &[
                3,
                119,
                119,
                119,
                6,
                103,
                111,
                111,
                103,
                108,
                101,
                3,
                99,
                111,
                109,
                0,
                0b1100_0000,
                0x0,
                0x0b,
                0x78,
                0x6e,
                0x2d,
                0x2d,
                0x79,
                0x65,
                0x74,
                0x73,
                0x37,
                0x36,
                0x65,
                0x0a,
                0x78,
                0x6e,
                0x2d,
                0x2d,
                0x66,
                0x69,
                0x71,
                0x73,
                0x38,
                0x73,
                0x00,
            ][..],
        );
        assert_eq!(
            &Domain::from_reader_and_check_map(reader, &mut map)
                .unwrap()
                .0,
            &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        );
        assert_eq!(
            map[&0b1100_0000_0000_0000].0,
            &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        );
        assert_eq!(
            &Domain::from_reader_and_check_map(reader, &mut map)
                .unwrap()
                .0,
            &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        );
        assert_eq!(
            &Domain::from_reader_and_check_map(reader, &mut map)
                .unwrap()
                .0,
            &[
                0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a, 0x78,
                0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
            ]
        )
    }
}
