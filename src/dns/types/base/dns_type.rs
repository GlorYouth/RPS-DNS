#![cfg_attr(debug_assertions, allow(dead_code))]

pub enum DnsType {
    A,
    CNAME,
    AAAA,
}

impl Into<u16> for DnsType {
    fn into(self) -> u16 {
        match self {
            DnsType::A => 1,
            DnsType::CNAME => 5,
            DnsType::AAAA => 28,
        }
    }
}
