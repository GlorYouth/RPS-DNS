#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::raw::{RawRequestHeader, RawResponseHeader};
use rand::{Rng, rng};
use std::fmt::Display;

#[derive(Debug)]
pub struct RequestHeader {
    id: u16,

    // 1bit 0代表请求，1代表响应
    response: u8,

    // 4bit 指定此消息中的查询类型
    opcode: u8,

    // 1bit 如果是1，说明此消息因长度大于传输信道上允许的长度而被截断/tcp传输？
    truncated: u8,

    // 1bit 如果是1，则指定服务器应当在查询不到域名的情况下尝试递归查询
    rec_desired: u8,

    // 1bit 是否为反向dns查询
    z: u8,

    // 1bit 为0不允许未经身份验证的数据
    check_disable: u8,
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
        )
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
        }
    }
}

impl RequestHeader {
    #[inline]
    pub fn get_id(&self) -> u16 {
        self.id
    }

    #[inline]
    pub fn get_flags_first_u8(&self) -> u8 {
        self.response << 7 | self.opcode << 3 | self.truncated << 1 | self.rec_desired
    }

    #[inline]
    pub fn get_flags_second_u8(&self) -> u8 {
        self.z << 6 | self.check_disable << 4
    }

    #[inline]
    pub fn get_flags(&self) -> u16 {
        (self.get_flags_first_u8() as u16) << 8 | (self.get_flags_second_u8() as u16)
    }

    #[inline]
    pub fn get_response(&self) -> u8 {
        self.response
    }

    #[inline]
    pub fn get_z(&self) -> u8 {
        self.z
    }

    #[inline]
    pub fn get_check_disable(&self) -> u8 {
        self.check_disable
    }

    #[inline]
    pub fn get_opcode(&self) -> u8 {
        self.opcode
    }

    #[inline]
    pub fn get_rec_desired(&self) -> u8 {
        self.rec_desired
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

impl From<&RawRequestHeader<'_>> for RequestHeader {
    fn from(header: &RawRequestHeader) -> Self {
        Self {
            id: header.get_id(),
            response: header.get_response(),
            opcode: header.get_opcode(),
            truncated: header.get_truncated(),
            rec_desired: header.get_rec_desired(),
            z: header.get_z(),
            check_disable: header.get_check_disable(),
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
}

impl From<&RawResponseHeader<'_>> for ResponseHeader {
    fn from(header: &RawResponseHeader) -> Self {
        Self {
            id: header.get_id(),
            response: header.get_response(),
            opcode: header.get_opcode(),
            authoritative: header.get_authoritative(),
            truncated: header.get_truncated(),
            rec_desired: header.get_rec_desired(),
            rec_avail: header.get_rec_avail(),
            z: header.get_z(),
            authenticated: header.get_authenticated(),
            check_disable: header.get_check_disable(),
            rcode: header.get_rcode(),
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
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::types::parts::header::{RequestHeader, format_flag};

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
}
