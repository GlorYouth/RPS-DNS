#![cfg_attr(debug_assertions, allow(non_snake_case, dead_code, unused))]

use crate::{DNSHeader, FlagsData, FlagsFormat};

pub struct DNSHeaderConstructor {
    pub ID: u16,

    // 0代表请求，1代表响应
    pub QR: u8,

    // 指定此消息中的查询类型
    pub Opcode: u8,

    // 如果是1，说明返回响应的那个服务器是authoritative的，也就是它“拥有”被查询的域名
    pub AA: u8,

    // 如果是1，则消息超过512字节。通常这意味着DNS是通过TCP协议通信的，此时长度限制不再有效
    pub TC: u8,

    // 若由请求的发送方设置为1，则说明服务器应当在查询不到域名的情况下尝试递归查询
    pub RD: u8,

    // 是否支持递归查询
    pub RA: u8,

    // 一开始是保留字段，现在被用于DNSSEC查询
    pub Z: u8,

    // 响应状态
    pub RCODE: u8,

    // Question的数量
    pub QDCOUNT: u16,

    // Answer的数量
    pub ANCOUNT: u16,

    // Authority Section的数量
    pub NSCOUNT: u16,

    // Additional Section的数量
    pub ARCOUNT: u16,
}

impl DNSHeaderConstructor {
    pub fn construct(&self) -> DNSHeader {
        DNSHeader {
            ID: self.ID,
            FLAGS: FlagsData::new(FlagsFormat {
                QR: self.QR,
                Opcode: self.Opcode,
                AA: self.AA,
                TC: self.TC,
                RD: self.RD,
                RA: self.RA,
                Z: self.Z,
                RCODE: self.RCODE,
            }),
            QDCOUNT: self.QDCOUNT,
            ANCOUNT: self.ANCOUNT,
            NSCOUNT: self.NSCOUNT,
            ARCOUNT: self.ARCOUNT,
        }
    }
}
