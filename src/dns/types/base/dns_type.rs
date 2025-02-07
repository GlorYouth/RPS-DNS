#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fmt::{Display, Formatter};

#[allow(unused)]
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

impl DnsType {
    pub(crate) fn from_u16(dns_type: u16) -> Option<DnsType> {
        match dns_type {
            1 => Some(DnsType::A),
            5 => Some(DnsType::CNAME),
            28 => Some(DnsType::AAAA),
            _ => None,
        }
    }
}

impl Display for DnsType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsType::A => {
                write!(f, "A")?;
            }
            DnsType::CNAME => {
                write!(f, "CNAME")?;
            }
            DnsType::AAAA => {
                write!(f, "AAAA")?;
            }
        }
        Ok(())
    }
}
