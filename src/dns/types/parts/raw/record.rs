#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::types::base::DnsType;
use crate::dns::types::base::RawDomain;
use crate::dns::utils::SliceReader;
#[cfg(debug_assertions)]
use log::{debug, trace};
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::DnsTypeNum;

pub struct RawRecord<'a> {
    name: RawDomain,
    rtype: u16,
    other: &'a [u8],
    // no data length, but you can use data.len() instead
    data: RawRecordDataType<'a>,
}

impl<'a> RawRecord<'a> {
    pub const FIX_SIZE: usize = 10;
    pub const LEAST_SIZE: usize = 12;

    pub fn new(reader: &mut SliceReader<'a>) -> Option<RawRecord<'a>> {
        #[cfg(debug_assertions)]
        {
            trace!("准备解析Record内的name");
        }
        let name = RawDomain::from_reader(reader)?;
        let len = reader.len();

        if reader.pos() + Self::FIX_SIZE > len {
            #[cfg(debug_assertions)]
            {
                trace!("解析完name后，剩余Slice不足以存放Record的其余部分");
            }
            return None;
        }
        let rtype = reader.read_u16();
        let other = reader.read_slice(6);
        let data_length = reader.read_u16() as usize;

        if reader.pos() + data_length > len {
            #[cfg(debug_assertions)]
            {
                debug!(
                    "读取到Record中Data可变部分长度为{:x},需要总Slice长度为{:x},实际Slice长度{:x}",
                    data_length,
                    reader.pos() + data_length,
                    len
                );
            }
            return None;
        }

        let data = match rtype {
            5 => RawRecordDataType::Domain(RawDomain::from_reader_with_size(reader, data_length)?),
            _ => RawRecordDataType::Other(reader.read_slice(data_length)),
        };

        Some(RawRecord {
            name,
            rtype,
            other,
            data,
        })
    }

    #[inline]
    pub fn get_name(&self) -> Option<String> {
        self.name.to_string()
    }

    #[inline]
    pub fn get_rtype(&self) -> u16 {
        self.rtype
    }

    #[inline]
    pub fn get_class(&self) -> u16 {
        u16::from_be_bytes(self.other[0..2].try_into().unwrap())
    }

    #[inline]
    pub fn get_ttl(&self) -> u32 {
        u32::from_be_bytes(self.other[2..6].try_into().unwrap())
    }

    #[inline]
    pub fn get_data(&self) -> Option<RecordDataType> {
        RecordDataType::new(self.get_rtype(), &self.data)
    }
}

enum RawRecordDataType<'a> {
    Domain(RawDomain),
    Other(&'a [u8]),
}

#[derive(Debug, Clone)]
pub enum RecordDataType {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME((String, usize)),
}

impl RecordDataType {
    fn new(rtype: u16, data: &RawRecordDataType) -> Option<RecordDataType> {
        match (rtype, data) {
            (DnsTypeNum::A, RawRecordDataType::Other(d)) if d.len() >= 4 => {
                Some(RecordDataType::A(Ipv4Addr::new(d[0], d[1], d[2], d[3])))
            }

            (DnsTypeNum::CNAME, RawRecordDataType::Domain(d)) => {
                let len = d.raw_len();
                Some(RecordDataType::CNAME((d.to_string()?, len)))
            }

            (DnsTypeNum::AAAA, RawRecordDataType::Other(d)) if d.len() >= 16 => Some(RecordDataType::AAAA(
                Ipv6Addr::from(<&[u8] as TryInto<[u8; 16]>>::try_into(d).ok()?),
            )),

            _ => {
                #[cfg(debug_assertions)]
                debug!("RecordDataType未实现类型或数据格式错误: {}", rtype);
                None
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            RecordDataType::A(_) => 4,
            RecordDataType::AAAA(_) => 16,
            RecordDataType::CNAME(str) => str.1,
        }
    }

    pub fn get_rtype(&self) -> u16 {
        match self {
            RecordDataType::A(_) => DnsTypeNum::A,
            RecordDataType::AAAA(_) => DnsTypeNum::AAAA,
            RecordDataType::CNAME(_) => DnsTypeNum::CNAME,
        }
    }

    pub fn get_dns_type(&self) -> DnsType {
        match self {
            RecordDataType::A(_) => DnsType::A,
            RecordDataType::AAAA(_) => DnsType::AAAA,
            RecordDataType::CNAME(_) => DnsType::CNAME,
        }
    }
}
