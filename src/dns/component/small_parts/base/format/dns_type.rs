#![cfg_attr(debug_assertions, allow(dead_code))]

pub enum DNSType {
    A,     //1
    CNAME, //5
    AAAA,  //28
    ANY,   //255
    Other(u16),
}

impl DNSType {
    pub fn from_u16(value: u16) -> DNSType {
        match value {
            1 => DNSType::A,
            5 => DNSType::CNAME,
            28 => DNSType::AAAA,
            255 => DNSType::ANY,
            _ => DNSType::Other(value),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            DNSType::A => 1,
            DNSType::CNAME => 5,
            DNSType::AAAA => 28,
            DNSType::ANY => 255,
            DNSType::Other(n) => *n,
        }
    }
}
