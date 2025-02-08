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
            DnsType::A => DnsTypeNum::A,
            DnsType::CNAME => DnsTypeNum::CNAME,
            DnsType::AAAA => DnsTypeNum::AAAA,
        }
    }
}

impl DnsType {
    pub(crate) fn from_u16(dns_type: u16) -> Option<DnsType> {
        match dns_type {
            DnsTypeNum::A => Some(DnsType::A),
            DnsTypeNum::CNAME => Some(DnsType::CNAME),
            DnsTypeNum::AAAA => Some(DnsType::AAAA),
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

pub struct DnsTypeNum;

impl DnsTypeNum {
    pub const A: u16 = 1;
    pub const CNAME: u16 = 5;
    pub const AAAA: u16 = 28;
}