#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::DnsType;
use crate::dns::types::parts::raw::{DnsClass, RawQuestion};
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Question {
    pub qname: Rc<String>,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    #[inline]
    pub fn new(question: &RawQuestion) -> Option<Question> {
        question.into()
    }
}

impl From<&RawQuestion<'_>> for Option<Question> {
    #[inline]
    fn from(question: &RawQuestion) -> Option<Question> {
        Some(Question {
            qname: Rc::from(question.get_name()?),
            qtype: question.get_qtype(),
            qclass: question.get_qclass(),
        })
    }
}

impl Display for Question {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "\t{}: type ", self.qname)?;
        if let Some(qtype) = DnsType::from_u16(self.qtype) {
            Display::fmt(&qtype, fmt)?;
        } else {
            write!(fmt, "Unsupported Type")?;
        }
        let qclass = DnsClass::get_str(self.qclass);
        writeln!(fmt, ", class {}", qclass)?;
        writeln!(fmt, "\t\tName: {}", self.qname)?;
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
