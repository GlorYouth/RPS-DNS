#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::raw::{RawAnswerHeader, RawRequestHeader};

#[derive(Debug)]
pub struct AnswerHeader {
    pub id: u16,

    // 1bit 0代表请求，1代表响应
    pub qr: u8,

    // 4bit 指定此消息中的查询类型
    pub opcode: u8,

    // 1bit 如果是1，说明返回响应的那个服务器是authoritative的，也就是它“拥有”被查询的域名
    pub aa: u8,

    // 1bit 如果是1，说明此消息因长度大于传输信道上允许的长度而被截断。
    pub tc: u8,

    // 1bit 若由请求的发送方设置为1，则说明服务器应当在查询不到域名的情况下尝试递归查询
    pub rd: u8,

    // 1bit 是否支持递归查询
    pub ra: u8,

    // 1bit 是否为反向dns查询
    pub z: u8,

    // 1bit Dns回复是否认证
    pub answer_authenticated: u8,

    // 1bit 为0不允许未经身份验证的数据
    pub non_authenticated: u8,

    // 4bit 响应状态
    pub rcode: u8,
}

impl From<&RawAnswerHeader<'_>> for AnswerHeader {
    fn from(header: &RawAnswerHeader) -> Self {
        Self {
            id: header.get_id(),
            qr: header.get_qr(),
            opcode: header.get_opcode(),
            aa: header.get_aa(),
            tc: header.get_tc(),
            rd: header.get_rd(),
            ra: header.get_ra(),
            z: header.get_z(),
            answer_authenticated: header.get_answer_authenticated(),
            non_authenticated: header.get_non_authenticated(),
            rcode: header.get_rcode(),
        }
    }
}

#[derive(Debug)]
pub struct RequestHeader {
    pub id: u16,

    // 1bit 0代表请求，1代表响应
    pub qr: u8,

    // 4bit 指定此消息中的查询类型
    pub opcode: u8,

    // 1bit 如果是1，说明此消息因长度大于传输信道上允许的长度而被截断/tcp传输？
    pub tc: u8,

    // 1bit 如果是1，则指定服务器应当在查询不到域名的情况下尝试递归查询
    pub rd: u8,

    // 1bit 是否为反向dns查询
    pub z: u8,

    // 1bit 为0不允许未经身份验证的数据
    pub non_authenticated: u8,
}

impl From<&RawRequestHeader<'_>> for RequestHeader {
    fn from(header: &RawRequestHeader) -> Self {
        Self {
            id: header.get_id(),
            qr: header.get_qr(),
            opcode: header.get_opcode(),
            tc: header.get_tc(),
            rd: header.get_rd(),
            z: header.get_z(),
            non_authenticated: header.get_non_authenticated(),
        }
    }
}
