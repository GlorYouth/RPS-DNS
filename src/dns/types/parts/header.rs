#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::raw::{RawResponseHeader, RawRequestHeader};


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

