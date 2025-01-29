#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::RawHeader;

pub struct Header {
    pub id: u16,

    // 0代表请求，1代表响应
    pub qr: u8,

    // 指定此消息中的查询类型
    pub opcode: u8,

    // 如果是1，说明返回响应的那个服务器是authoritative的，也就是它“拥有”被查询的域名
    pub aa: u8,

    // 如果是1，则消息超过512字节。通常这意味着DNS是通过TCP协议通信的，此时长度限制不再有效
    pub tc: u8,

    // 若由请求的发送方设置为1，则说明服务器应当在查询不到域名的情况下尝试递归查询
    pub rd: u8,

    // 是否支持递归查询
    pub ra: u8,

    // 一开始是保留字段，现在被用于DNSSEC查询
    pub z: u8,

    // 响应状态
    pub rcode: u8,
}

impl From<RawHeader<'_>> for Header {
    fn from(header: RawHeader) -> Self {
        Self {
            id: header.get_id(),
            qr: header.get_qr(),
            opcode: header.get_opcode(),
            aa: header.get_aa(),
            tc: header.get_tc(),
            rd: header.get_rd(),
            ra: header.get_ra(),
            z: header.get_z(),
            rcode: header.get_rcode(),
        }
    }
}
