#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::base::RawDomain;
use crate::dns::utils::SliceReader;
#[cfg(feature = "logger")]
use log::trace;

#[derive(Debug)]
pub struct RawQuestion<'a> {
    name: RawDomain,
    other: &'a [u8],
}

impl<'a> RawQuestion<'a> {
    pub const FIX_SIZE: usize = 4;
    pub const LEAST_SIZE: usize = Self::FIX_SIZE + 2;

    pub fn new(reader: &mut SliceReader<'a>) -> Option<RawQuestion<'a>> {
        #[cfg(feature = "logger")]
        {
            trace!("准备解析Question内的name");
        }
        let name = RawDomain::from_reader(reader)?;
        let len = reader.len();
        if reader.pos() + Self::FIX_SIZE > len {
            #[cfg(feature = "logger")]
            {
                trace!("解析完name后，剩余Slice不足以存放Question的其余部分");
            }
            return None; //检测出界，防止panic
        }
        Some(RawQuestion {
            name,
            other: reader.read_slice(Self::FIX_SIZE),
        })
    }

    #[inline]
    pub fn get_name(&self) -> Option<String> {
        self.name.to_string()
    }

    #[inline]
    pub fn get_qtype(&self) -> u16 {
        u16::from_be_bytes(self.other[0..2].try_into().unwrap())
    }

    #[inline]
    pub fn get_qclass(&self) -> u16 {
        u16::from_be_bytes(self.other[2..4].try_into().unwrap())
    }
}
