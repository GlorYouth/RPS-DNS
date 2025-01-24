#![allow(non_snake_case, dead_code)]

use crate::*;
use generic_array::typenum::U2;

#[derive(Debug)]
pub struct Flags {
    // 0代表请求，1代表响应
    QR: u8,

    // 指定此消息中的查询类型
    Opcode: u8,

    // 如果是1，说明返回响应的那个服务器是authoritative的，也就是它“拥有”被查询的域名
    AA: u8,

    // 如果是1，则消息超过512字节。通常这意味着DNS是通过TCP协议通信的，此时长度限制不再有效
    TC: u8,

    // 若由请求的发送方设置为1，则说明服务器应当在查询不到域名的情况下尝试递归查询
    RD: u8,

    // 是否支持递归查询
    RA: u8,

    // 一开始是保留字段，现在被用于DNSSEC查询
    Z: u8,

    // 响应状态
    RCODE: u8,
}

impl Flags {
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

    pub fn from_array(arr: ArrayU8<U2>) -> Flags {
        Flags {
            QR: arr[0] >> 7,
            Opcode: (arr[0] << 1) >> 4,
            AA: (arr[0] & 0b0000_0100) >> 2,
            TC: (arr[0] & 0b0000_0010) >> 1,
            RD: arr[0] & 0b0000_0001,
            RA: arr[1] >> 7,
            Z: (arr[1] << 1) >> 4,
            RCODE: arr[1] & 0b0000_0111,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use generic_array::typenum::U2;

    #[test]
    fn test_flags_from_array() {
        let arr = ArrayU8::<U2>::from_bytes(&[0b1000_0001, 0b1000_0000]);
        let flags = Flags::from_array(arr);
        assert_eq!(flags.QR, 0b0000_0001);
        assert_eq!(flags.Opcode, 0b0000_0000);
        assert_eq!(flags.AA, 0b0000_0000);
        assert_eq!(flags.TC, 0b0000_0000);
        assert_eq!(flags.RD, 0b0000_0001);
        assert_eq!(flags.RA, 0b0000_0001);
        assert_eq!(flags.Z, 0b0000_0000);
        assert_eq!(flags.RCODE, 0b0000_0000);
    }

    #[test]
    fn test_flags_to_vec() {
        let flags = Flags::from_array(ArrayU8::from_bytes(&[0b1000_0001, 0b1000_0000]));
        assert_eq!(
            flags.into_vec(),
            ArrayU8::<U2>::from_bytes(&[0b1000_0001, 0b1000_0000]).to_vec()
        );
    }

    #[test]
    fn test_flags_into_u16() {
        let flags = Flags::from_array(ArrayU8::from_bytes(&[0b1000_0001, 0b1000_0000]));
        assert_eq!(
            flags.into_u16(),
            u16::from_be_bytes([0b1000_0001, 0b1000_0000])
        );
    }
}
