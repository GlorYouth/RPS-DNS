#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::types::base::RawDomain;
use crate::dns::utils::SliceReader;
use log::{debug, trace};
use std::net::{Ipv4Addr, Ipv6Addr};

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

    pub fn new(
        // 'b为引用存在的周期，比'a对象存在的周期短或等于
        reader: &mut SliceReader<'a>,
    ) -> Option<RawRecord<'a>> {
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

#[derive(Debug)]
pub enum RecordDataType {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
}

impl RecordDataType {
    fn new(rtype: u16, data: &RawRecordDataType) -> Option<RecordDataType> {
        match rtype {
            1 => {
                let data = match data {
                    RawRecordDataType::Other(d) => d,
                    _ => {
                        #[cfg(debug_assertions)]
                        {
                            debug!("RecordDataType::new代码异常");
                        }
                        return None;
                    }
                };
                Some(RecordDataType::A(Ipv4Addr::new(
                    data[0], data[1], data[2], data[3],
                )))
            }
            5 => {
                let data = match data {
                    RawRecordDataType::Domain(d) => d,
                    _ => {
                        #[cfg(debug_assertions)]
                        {
                            debug!("RecordDataType::new代码异常");
                        }
                        return None;
                    }
                };
                Some(RecordDataType::CNAME(data.to_string()?))
            }
            28 => {
                let data = match data {
                    RawRecordDataType::Other(d) => d,
                    _ => {
                        #[cfg(debug_assertions)]
                        {
                            debug!("RecordDataType::new代码异常");
                        }
                        return None;
                    }
                };
                Some(RecordDataType::AAAA(Ipv6Addr::from(
                    <&[u8] as TryInto<[u8; 16]>>::try_into(data).unwrap(),
                )))
            }
            _ => {
                #[cfg(debug_assertions)]
                {
                    debug!("RecordDataType未实现类型: {}", rtype);
                }
                None
            }
        }
    }
}
