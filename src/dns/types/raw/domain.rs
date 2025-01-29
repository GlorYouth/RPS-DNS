#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use crate::dns::utils::SliceReader;
use crate::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, write};
use std::ops::Add;
use std::rc::Rc;
use std::str::Utf8Error;
use small_map::SmallMap;
use crate::dns::types::raw::question::RawQuestion;

const SIZE_OF_XN: usize = "xn--".len();
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RawDomain<'a>(&'a [u8]);

impl<'a> From<&'a [u8]> for RawDomain<'a> {
    #[inline]
    fn from(slice: &'a [u8]) -> RawDomain<'a> {
        Self(slice)
    }
}

impl<'a> RawDomain<'a> {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn clone(&self) -> RawDomain<'a> {
        RawDomain(self.0)
    }

    #[inline]
    pub fn new<'b>(
        reader: &'b mut SliceReader<'a>,
        map: &mut SmallMap<32,u16,RawDomain<'a>>,
    ) -> Option<RawDomain<'a>> {
        if reader.peek_u8() & 0b1100_0000_u8 == 0b1100_0000_u8 {
            let key = reader.read_u16();
            return Some(map.get(&key)?.clone());
        }
        let position = reader.pos();
        let len = reader.len();
        let mut read = reader.read_u8();
        while read != 0x0_u8 {
            if position + read as usize + 1 > len {
                return None; //检测出界，防止panic
            }
            reader.skip(read as usize);
            read = reader.read_u8();
        }

        let name = RawDomain::from(&reader.as_mut()[position..reader.pos()]);
        map.insert((position as u16) | 0b1100_0000_0000_0000_u16, name.clone());
        Some(name)
    }

    pub fn to_string_ret_err(&self) -> Result<String, Box<DomainDecodeError>> {
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
                let input = std::str::from_utf8(&part_bytes[4..])?; // 去掉 'xn--' 前缀
                match punycode::decode(input) {
                    Ok(decoded_part) => {
                        decoded.push_str(&decoded_part);
                    }
                    Err(_) => {
                        return Err(Box::from(DomainDecodeError::PunycodeDecode {
                            string: input.to_string(),
                        }));
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

    pub fn to_string(&self) -> Option<String> {
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
                        return None;
                    }
                }
            } else {
                // 直接是 ASCII 字符部分
                match std::str::from_utf8(part_bytes) {
                    Ok(decoded_part) => {
                        decoded.push_str(&decoded_part);
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }

            // 添加分隔符 "."
            if i < self.0.len() - 1 {
                decoded.push('.');
            }
        }
        Some(decoded)
    }
}

pub enum DomainError {
    ReadFailed {
        findings: String,
        container_type: Cow<'static, str>,
        other_info: String,
    },
    DecodeFailed,
}

pub enum DomainDecodeError {
    InputNotAscii { source: Utf8Error },
    PunycodeDecode { string: String },
}

impl Display for DomainDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Debug for DomainDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<Utf8Error> for DomainDecodeError {
    fn from(source: Utf8Error) -> Self {
        DomainDecodeError::InputNotAscii { source }
    }
}

impl From<Utf8Error> for Box<DomainDecodeError> {
    fn from(source: Utf8Error) -> Self {
        Box::from(DomainDecodeError::InputNotAscii { source })
    }
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
