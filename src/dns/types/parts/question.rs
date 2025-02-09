#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::base::{DnsType, RawDomain};
use crate::dns::types::parts::DnsClass;
use crate::dns::utils::SliceReader;
use log::trace;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Question {
    pub qname: Rc<RawDomain>,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    pub const FIX_SIZE: usize = 4;
    pub const LEAST_SIZE: usize = Self::FIX_SIZE + 2;
    #[inline]
    pub fn new(reader: &mut SliceReader) -> Option<Question> {
        #[cfg(debug_assertions)]
        {
            trace!("准备解析Question内的name");
        }

        let name = RawDomain::from_reader(reader)?;
        let len = reader.len();
        if reader.pos() + Self::FIX_SIZE > len {
            #[cfg(debug_assertions)]
            {
                trace!("解析完name后，剩余Slice不足以存放Question的其余部分");
            }
            return None; //检测出界，防止panic
        }
        Some(Question {
            qname: Rc::new(name),
            qtype: reader.read_u16(),
            qclass: reader.read_u16(),
        })
    }
}

impl Display for Question {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "\t{}: type ", self.qname.to_string().unwrap_or_else(||"???".to_owned()))?;
        if let Some(qtype) = DnsType::from_u16(self.qtype) {
            Display::fmt(&qtype, fmt)?;
        } else {
            write!(fmt, "Unsupported Type")?;
        }
        let qclass = DnsClass::get_str(self.qclass);
        writeln!(fmt, ", class {}", qclass)?;
        writeln!(fmt, "\t\tName: {}", self.qname.to_string().unwrap_or_else(||"???".to_owned()))?;
        write!(fmt, "\t\tType: ")?;
        if let Some(qtype) = DnsType::from_u16(self.qtype) {
            Display::fmt(&qtype, fmt)?;
        } else {
            write!(fmt, "Unsupported Type")?;
        }
        writeln!(fmt, " ({})", self.qtype)?;
        writeln!(fmt, "\t\tClass: {} ({:#06X})", qclass, self.qclass)
    }
}
