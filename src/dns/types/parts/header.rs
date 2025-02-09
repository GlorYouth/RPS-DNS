#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::utils::SliceReader;
use rand::{Rng, rng};
use std::fmt::Display;

#[derive(Debug)]
pub struct RequestHeader {
    pub id: u16,

    // 1bit 0代表请求，1代表响应
    pub response: u8,

    // 4bit 指定此消息中的查询类型
    pub opcode: u8,

    // 1bit 如果是1，说明此消息因长度大于传输信道上允许的长度而被截断/tcp传输？
    pub truncated: u8,

    // 1bit 如果是1，则指定服务器应当在查询不到域名的情况下尝试递归查询
    pub rec_desired: u8,

    // 1bit 是否为反向dns查询
    pub z: u8,

    // 1bit 为0不允许未经身份验证的数据
    pub check_disable: u8,

    pub questions: u16,

    pub answer_rrs: u16,

    pub authority_rrs: u16,

    pub additional_rrs: u16,
}

impl Display for RequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Header: ")?;
        writeln!(f, "\tTransaction ID: {:#06X}", self.id)?;
        let opcode = match self.opcode {
            0 => "Standard query",
            1 => "Inverse query",
            2 => "server status request",
            _ => "reserved for future use",
        };
        writeln!(f, "\tFlags: {:#06X} {}", self.get_flags(), opcode)?;

        let response = match self.response {
            0 => "query",
            1 => "response",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Response: Message is a {}",
            format_flag(self.response, 0, 1),
            response
        )?;

        writeln!(
            f,
            "\t\t{} => Opcode: {} ({})",
            format_flag(self.opcode, 1, 4),
            opcode,
            self.opcode
        )?;

        let truncated = match self.truncated {
            0 => "not truncated",
            1 => "truncated",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Truncated: Message is {}",
            format_flag(self.truncated, 6, 1),
            truncated
        )?;

        let rec_desired = match self.rec_desired {
            0 => "Do",
            1 => "Don't",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Recursion Desired: {} query recursively",
            format_flag(self.rec_desired, 7, 1),
            rec_desired
        )?;

        writeln!(
            f,
            "\t\t{} => Z: reserved ({})",
            format_flag(self.z, 9, 1),
            self.z
        )?;

        let check_disable = match self.check_disable {
            0 => "Unacceptable",
            1 => "Acceptable",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Non-authenticated data: {}",
            format_flag(self.check_disable, 11, 1),
            check_disable
        )?;
        writeln!(f, "\tQuestions: {}", self.questions)?;
        writeln!(f, "\tAnswer RRs: {}", self.answer_rrs)?;
        writeln!(f, "\tAuthority RRs: {}", self.authority_rrs)?;
        writeln!(f, "\tAdditional RRs: {}", self.additional_rrs)
    }
}

pub const HEADER_SIZE: usize = 12;
impl RequestHeader {
    #[inline]
    fn get_flags_first_u8(&self) -> u8 {
        self.response << 7 | self.opcode << 3 | self.truncated << 1 | self.rec_desired
    }

    #[inline]
    fn get_flags_second_u8(&self) -> u8 {
        self.z << 6 | self.check_disable << 4 | self.opcode
    }

    #[inline]
    pub(crate) fn get_flags(&self) -> u16 {
        (self.get_flags_first_u8() as u16) << 8 | (self.get_flags_second_u8() as u16)
    }
}

impl Default for RequestHeader {
    fn default() -> Self {
        RequestHeader {
            id: rng().random(),
            response: 0,
            opcode: 0,
            truncated: 0,
            rec_desired: 1,
            z: 0,
            check_disable: 0,
            questions: 1,
            answer_rrs: 0,
            authority_rrs: 0,
            additional_rrs: 0,
        }
    }
}

#[inline]
fn format_flag(value: u8, position: u8, length: u8) -> String {
    assert!(position + length <= 16, "Position and length exceed range.");

    let mut s = String::from(".... .... .... ....");
    let bits_str = format!("{:0width$b}", value, width = length as usize);
    let chars = bits_str.chars();

    for (k, bit) in chars.enumerate() {
        let char_index = (position as usize + k) + (position as usize + k) / 4;
        s.replace_range(char_index..=char_index, &bit.to_string());
    }

    s
}

impl From<&mut SliceReader<'_>> for RequestHeader {
    fn from(reader: &mut SliceReader) -> Self {
        let id = reader.read_u16();
        let first_u8 = reader.read_u8();
        let second_u8 = reader.read_u8();
        let response = first_u8 >> 7;
        let opcode = (first_u8 << 1) >> 4;
        let truncated = (first_u8 & 0b0000_0010) >> 1;
        let rec_desired = first_u8 & 0b0000_0001;
        let z = (second_u8 << 1) >> 7;
        let check_disable = (second_u8 << 3) >> 7;
        let questions = reader.read_u16();
        let answer_rrs = reader.read_u16();
        let authority_rrs = reader.read_u16();
        let additional_rrs = reader.read_u16();

        Self {
            id,
            response,
            opcode,
            truncated,
            rec_desired,
            z,
            check_disable,
            questions,
            answer_rrs,
            authority_rrs,
            additional_rrs,
        }
    }
}

#[derive(Debug)]
pub struct ResponseHeader {
    pub id: u16,

    // 1bit 0代表请求，1代表响应
    pub response: u8,

    // 4bit 指定此消息中的查询类型
    pub opcode: u8,

    // 1bit 如果是1，说明返回响应的那个服务器是authoritative的，也就是它“拥有”被查询的域名
    pub authoritative: u8,

    // 1bit 如果是1，说明此消息因长度大于传输信道上允许的长度而被截断。
    pub truncated: u8,

    // 1bit 若由请求的发送方设置为1，则说明服务器应当在查询不到域名的情况下尝试递归查询
    pub rec_desired: u8,

    // 1bit 是否支持递归查询
    pub rec_avail: u8,

    // 1bit 是否为反向dns查询
    pub z: u8,

    // 1bit Dns回复是否认证
    pub authenticated: u8,

    // 1bit 为0不允许未经身份验证的数据
    pub check_disable: u8,

    // 4bit 响应状态
    pub rcode: u8,

    pub questions: u16,

    pub answer_rrs: u16,

    pub authority_rrs: u16,

    pub additional_rrs: u16,
}

impl From<&mut SliceReader<'_>> for ResponseHeader {
    fn from(reader: &mut SliceReader) -> Self {
        let id = reader.read_u16();
        let first_u8 = reader.read_u8();
        let second_u8 = reader.read_u8();

        let response = first_u8 >> 7;
        let opcode = (first_u8 << 1) >> 4;
        let authoritative = (first_u8 & 0b0000_0100) >> 2;
        let truncated = (first_u8 & 0b0000_0010) >> 1;
        let rec_desired = first_u8 & 0b0000_0001;
        let rec_avail = second_u8 >> 7;
        let z = (second_u8 << 1) >> 7;
        let authenticated = (second_u8 << 2) >> 7;
        let check_disable = (second_u8 << 3) >> 7;
        let rcode = (second_u8 << 4) >> 4;
        let questions = reader.read_u16();
        let answer_rrs = reader.read_u16();
        let authority_rrs = reader.read_u16();
        let additional_rrs = reader.read_u16();

        Self {
            id,
            response,
            opcode,
            authoritative,
            truncated,
            rec_desired,
            rec_avail,
            z,
            authenticated,
            check_disable,
            rcode,
            questions,
            answer_rrs,
            authority_rrs,
            additional_rrs,
        }
    }
}

impl ResponseHeader {
    #[inline]
    fn get_flags_first_u8(&self) -> u8 {
        self.response << 7
            | self.opcode << 3
            | self.authoritative << 2
            | self.truncated << 1
            | self.rec_desired
    }

    #[inline]
    fn get_flags_second_u8(&self) -> u8 {
        self.rec_avail << 7
            | self.z << 6
            | self.authenticated << 5
            | self.check_disable << 4
            | self.opcode
    }

    #[inline]
    fn get_flags(&self) -> u16 {
        (self.get_flags_first_u8() as u16) << 8 | (self.get_flags_second_u8() as u16)
    }
}

impl Display for ResponseHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Header: ")?;
        writeln!(f, "\tTransaction ID: {:#06X}", self.id)?;
        let opcode = match self.opcode {
            0 => "Standard query",
            1 => "Inverse query",
            2 => "server status request",
            _ => "reserved for future use",
        };
        writeln!(f, "\tFlags: {:#06X} {}", self.get_flags(), opcode)?;

        let response = match self.response {
            0 => "query",
            1 => "response",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Response: Message is a {}",
            format_flag(self.response, 0, 1),
            response
        )?;

        writeln!(
            f,
            "\t\t{} => Opcode: {} ({})",
            format_flag(self.opcode, 1, 4),
            opcode,
            self.opcode
        )?;

        let authoritative = match self.authoritative {
            0 => "Server is not an authoritative server for domain",
            1 => "Server is an authoritative server for domain",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Authoritative: {}",
            format_flag(self.authenticated, 5, 1),
            authoritative
        )?;

        let truncated = match self.truncated {
            0 => "not truncated",
            1 => "truncated",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Truncated: Message is {}",
            format_flag(self.truncated, 6, 1),
            truncated
        )?;

        let rec_desired = match self.rec_desired {
            0 => "Do",
            1 => "Don't",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Recursion Desired: {} query recursively",
            format_flag(self.rec_desired, 7, 1),
            rec_desired
        )?;

        let rec_avail = match self.rec_avail {
            0 => "Server can do recursive queries",
            1 => "Server cannot do recursive queries",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Recursion Available: {}",
            format_flag(self.rec_avail, 8, 1),
            rec_avail
        )?;

        writeln!(
            f,
            "\t\t{} => Z: reserved ({})",
            format_flag(self.z, 9, 1),
            self.z
        )?;

        let authenticated = match self.authenticated {
            0 => "Answer/Authority portion was not authenticated by the server",
            1 => "Answer/Authority portion was authenticated by the server",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Answer authenticated: {}",
            format_flag(self.authenticated, 10, 1),
            authenticated
        )?;

        let check_disable = match self.check_disable {
            0 => "Unacceptable",
            1 => "Acceptable",
            _ => "code error",
        };
        writeln!(
            f,
            "\t\t{} => Non-authenticated data: {}",
            format_flag(self.check_disable, 11, 1),
            check_disable
        )?;

        let rcode = match self.rcode {
            0 => "No error",
            1 => "Format error",
            2 => "Server failure",
            3 => "Name error",
            4 => "Not implemented",
            5 => "Refused",
            _ => "Reserved",
        };
        writeln!(
            f,
            "\t\t{} => Reply code: {} ({})",
            format_flag(self.rcode, 12, 4),
            rcode,
            self.rcode
        )?;
        writeln!(f, "\tQuestions: {}", self.questions)?;
        writeln!(f, "\tAnswer RRs: {}", self.answer_rrs)?;
        writeln!(f, "\tAuthority RRs: {}", self.authority_rrs)?;
        writeln!(f, "\tAdditional RRs: {}", self.additional_rrs)
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::types::parts::header::{RequestHeader, ResponseHeader, format_flag};
    use crate::dns::utils::SliceReader;

    #[test]
    fn test_fmt() {
        let header = RequestHeader {
            id: 0xad,
            response: 0,
            opcode: 0,
            truncated: 0,
            rec_desired: 1,
            z: 0,
            check_disable: 0,
            questions: 1,
            answer_rrs: 0,
            authority_rrs: 0,
            additional_rrs: 0,
        };
        println!("{:}", header);
    }

    #[test]
    fn test_format_flag() {
        assert_eq!(format_flag(0x5, 3, 4), "...0 101. .... ....");
        assert_eq!(format_flag(0x1, 0, 1), "1... .... .... ....");
        assert_eq!(format_flag(0b0110, 2, 3), "..11 0... .... ....");
        assert_eq!(format_flag(0b1111, 0, 16), "0000 0000 0000 1111");
    }

    #[test]
    fn test_request_header() {
        let mut reader = SliceReader::from(
            &[
                0x75, 0xb3, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x64,
                0x6e, 0x73, 0x06, 0x77, 0x65, 0x69, 0x78, 0x69, 0x6e, 0x02, 0x71, 0x71, 0x03, 0x63,
                0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01,
            ][..],
        );
        let header = RequestHeader::from(&mut reader);
        assert_eq!(header.id, 0x75b3);
        assert_eq!(header.response, 0x00);
        assert_eq!(header.opcode, 0x00);
        assert_eq!(header.truncated, 0x00);
        assert_eq!(header.rec_desired, 0x01);
        assert_eq!(header.z, 0x00);
        assert_eq!(header.check_disable, 0x00);
        assert_eq!(header.questions, 0x01);
        assert_eq!(header.answer_rrs, 0x00);
        assert_eq!(header.authority_rrs, 0x00);
        assert_eq!(header.additional_rrs, 0x00);
    }

    #[test]
    fn test_response_header() {
        let mut reader = SliceReader::from(
            &[
                0x0f, 0x04, 0x81, 0x80, 0x00, 0x01, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x06, 0x63,
                0x6f, 0x6e, 0x66, 0x69, 0x67, 0x04, 0x65, 0x64, 0x67, 0x65, 0x05, 0x73, 0x6b, 0x79,
                0x70, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00,
                0x05, 0x00, 0x01, 0x00, 0x00, 0x22, 0x30, 0x00, 0x2a, 0x06, 0x63, 0x6f, 0x6e, 0x66,
                0x69, 0x67, 0x04, 0x65, 0x64, 0x67, 0x65, 0x05, 0x73, 0x6b, 0x79, 0x70, 0x65, 0x03,
                0x63, 0x6f, 0x6d, 0x0e, 0x74, 0x72, 0x61, 0x66, 0x66, 0x69, 0x63, 0x6d, 0x61, 0x6e,
                0x61, 0x67, 0x65, 0x72, 0x03, 0x6e, 0x65, 0x74, 0x00, 0xc0, 0x33, 0x00, 0x05, 0x00,
                0x01, 0x00, 0x00, 0x00, 0x3a, 0x00, 0x10, 0x06, 0x6c, 0x2d, 0x30, 0x30, 0x30, 0x37,
                0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0xc0, 0x18, 0xc0, 0x69, 0x00, 0x05, 0x00,
                0x01, 0x00, 0x00, 0x21, 0x3e, 0x00, 0x24, 0x11, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
                0x2d, 0x65, 0x64, 0x67, 0x65, 0x2d, 0x73, 0x6b, 0x79, 0x70, 0x65, 0x06, 0x6c, 0x2d,
                0x30, 0x30, 0x30, 0x37, 0x08, 0x6c, 0x2d, 0x6d, 0x73, 0x65, 0x64, 0x67, 0x65, 0xc0,
                0x58, 0xc0, 0x85, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x02, 0xc0,
                0x97, 0xc0, 0x97, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x92, 0x00, 0x04, 0x0d,
                0x6b, 0x2a, 0x10,
            ][..],
        );
        let header = ResponseHeader::from(&mut reader);
        assert_eq!(header.id, 0x0f04);
        assert_eq!(header.response, 0x01);
        assert_eq!(header.opcode, 0x00);
        assert_eq!(header.authoritative, 0x00);
        assert_eq!(header.truncated, 0x00);
        assert_eq!(header.rec_desired, 0x01);
        assert_eq!(header.rec_avail, 0x01);
        assert_eq!(header.z, 0x00);
        assert_eq!(header.authenticated, 0x00);
        assert_eq!(header.check_disable, 0x00);
        assert_eq!(header.rcode, 0x00);
        assert_eq!(header.questions, 0x01);
        assert_eq!(header.answer_rrs, 0x05);
        assert_eq!(header.authority_rrs, 0x00);
        assert_eq!(header.additional_rrs, 0x00);
    }
}
