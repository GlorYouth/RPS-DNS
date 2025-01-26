#![allow(non_snake_case, dead_code)]

use crate::FlagsData;

#[derive(Debug)]
pub struct FlagsFormat {
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
}

impl FlagsFormat {
    pub fn into_u16(self) -> u16 {
        ((self.QR << 7 | self.Opcode << 3 | self.AA << 2 | self.TC << 1 | self.RD) as u16) << 8
            | (self.RA << 7 | self.Z << 6 | self.RCODE) as u16
    }

    pub fn into_vec(self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(2);
        vec.push(self.QR << 7 | self.Opcode << 3 | self.AA << 2 | self.TC << 1 | self.RD);
        vec.push(self.RA << 7 | self.Z << 6 | self.RCODE);
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use generic_array::typenum::U2;

    #[test]
    fn test_flags_to_vec() {
        let flags = FlagsData(ArrayU8::from(&[0b1000_0001, 0b1000_0000][..])).resolve();
        assert_eq!(
            flags.into_vec(),
            ArrayU8::<U2>::from(&[0b1000_0001, 0b1000_0000][..]).to_vec()
        );
    }

    #[test]
    fn test_flags_into_u16() {
        let flags = FlagsData(ArrayU8::from(&[0b1000_0001, 0b1000_0000][..])).resolve();
        assert_eq!(
            flags.into_u16(),
            u16::from_be_bytes([0b1000_0001, 0b1000_0000])
        );
    }
}
