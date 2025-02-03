#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::utils::SliceReader;
use small_map::SmallMap;
use std::fmt::Debug;
use log::{debug, trace};

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
            #[cfg(debug_assertions)] {
                trace!("发现是Domain Pointer");
            }
            let key = reader.read_u16();
            #[cfg(debug_assertions)] {
                trace!("其指向字节为:{:x}",key);
            }
            
            if let Some(value) = map.get(&key) {
                return Some(value.clone());
            }
            #[cfg(debug_assertions)] {
                debug!("在Map中并未找到key");
            }
            return None;
        }
        #[cfg(debug_assertions)] {
            trace!("发现是普通的域名");
        }
        let position = reader.pos();
        let len = reader.len();
        let mut read = reader.read_u8();
        if read == 0x0_u8 {
            #[cfg(debug_assertions)] {
                debug!("DomainName没有长度");
            }
            return None; //防止无长度的域名
        }
        while read != 0x0_u8 {
            if position + read as usize > len {
                #[cfg(debug_assertions)] {
                    debug!("在依次读取域名的片段时出界");
                }
                return None; //检测出界，防止panic
            }
            reader.skip(read as usize);
            read = reader.read_u8();
        }

        let name = RawDomain::from(&reader.as_mut()[position..reader.pos()-1]);
        map.insert((position as u16) | 0b1100_0000_0000_0000_u16, name.clone());
        #[cfg(debug_assertions)] {
            trace!("DomainName读取完成，将信息{:x}插入Map",(position as u16) | 0b1100_0000_0000_0000_u16);
        }
        Some(name)
    }
    
    pub fn to_string(&self) -> Option<String> {
        let mut string = String::with_capacity(40);
        let mut remaining = self.0;

        while !remaining.is_empty() {
            let part_length = remaining[0] as usize;
            remaining = &remaining[1..];

            // 检查长度有效性
            if remaining.len() < part_length {
                return None;
            }
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
                        return None;
                    }
                }
            } else {
                for byte in part_bytes {
                    if byte.is_ascii() {
                        string.push(*byte as char);
                    } else {
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