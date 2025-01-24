#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use crate::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::Utf8Error;

const SIZE_OF_XN: usize = "xn--".len();
#[derive(PartialEq, Debug)]
pub struct Domain(pub Vec<u8>);

impl Domain {
    
    pub fn from_reader(reader: &mut SliceReader) -> Domain {
        if let Some(offset) = reader.as_ref().iter().position(|b| *b == 0x0) {
            return Domain(Vec::from(reader.read_slice(offset + 1)));
        }
        panic!()
    }

    pub fn from_reader_for_question(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
    ) -> Rc<Domain> {
        if let Some(offset) = reader.iter_from_current_pos().position(|b| *b == 0x0) {
            let pos = reader.pos() as u16;
            let domain = Rc::new(Domain(Vec::from(reader.read_slice(offset + 1))));
            map.insert(pos | 0b1100_0000_0000_0000, domain.clone());
            return domain;
        }
        panic!()
    }

    pub fn from_reader_for_record(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
    ) -> Rc<Domain> {
        if reader.peek_u8() & 0b1100_0000 == 0b1100_0000 {
            return map[&reader.read_u16()].clone();
        }
        Self::from_reader_for_question(reader, map)
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

    pub fn from_str(str: &String) -> Domain {
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

    pub fn to_string(&self) -> Result<String, Utf8Error> {
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
                let encoded_part = &part_bytes[4..]; // 去掉 'xn--' 前缀
                match punycode::decode(std::str::from_utf8(encoded_part)?) {
                    Ok(decoded_part) => {
                        decoded.push_str(&decoded_part);
                    }
                    Err(_) => {
                        panic!("Punycode 解码失败");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_from_str() {
        assert_eq!(
            Domain::from_str(&"小米.中国".to_string()).0,
            [
                0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a, 0x78,
                0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
            ]
        );
        assert_eq!(
            Domain::from_str(&"www.google.com".to_string()).0,
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
            &Domain::from_reader(&mut SliceReader::from(&[
                3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0
            ][..]))
            .to_string()
            .unwrap(),
            "www.google.com"
        );

        assert_eq!(
            &Domain::from_reader(&mut SliceReader::from(&[
                3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0
            ][..]))
            .0,
            &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        )
    }

    #[test]
    fn test_domain_from_reader_for_answer() {
        let mut map: HashMap<u16, Rc<Domain>> = HashMap::new();
        let reader = &mut SliceReader::from(&[
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
        ][..]);
        assert_eq!(
            &Domain::from_reader_for_question(reader, &mut map).0,
            &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        );
        assert_eq!(
            map[&0b1100_0000_0000_0000].0,
            &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        );
        assert_eq!(
            &Domain::from_reader_for_record(reader, &mut map).0,
            &[3, 119, 119, 119, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        );
        assert_eq!(
            &Domain::from_reader_for_record(reader, &mut map).0,
            &[
                0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a, 0x78,
                0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00
            ]
        )
    }
}
