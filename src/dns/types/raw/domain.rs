#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use crate::dns::types::raw::question::RawQuestion;
use crate::dns::utils::SliceReader;
use crate::*;
use small_map::SmallMap;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, write};
use std::ops::Add;
use std::rc::Rc;
use std::str::Utf8Error;

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
        map: &mut SmallMap<32, u16, RawDomain<'a>>,
    ) -> Option<RawDomain<'a>> {
        if reader.peek_u8() & 0b1100_0000_u8 == 0b1100_0000_u8 {
            let key = reader.read_u16();
            return Some(map.get(&key)?.clone());
        }
        let position = reader.pos();
        let len = reader.len();
        let mut read = reader.read_u8();
        while read != 0x0_u8 {
            if position + read as usize > len {
                return None; //检测出界，防止panic
            }
            reader.skip(read as usize);
            read = reader.read_u8();
        }

        let name = RawDomain::from(&reader.as_mut()[position..reader.pos()-1]);
        map.insert((position as u16) | 0b1100_0000_0000_0000_u16, name.clone());
        Some(name)
    }
    
    pub fn to_string(&self) -> Option<String> {
        let mut decoded = String::with_capacity(40);
        let mut i = 0;

        while i < self.0.len() {
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
                for byte in part_bytes {
                    if byte.is_ascii() {
                        decoded.push(*byte as char);
                    } else {
                        return None;
                    }
                }
            }

            // 添加分隔符 "."
            if i < self.0.len() {
                decoded.push('.');
            }
        }
        Some(decoded)
    }
}