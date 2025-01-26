#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{DNSQuestion, Domain};
use std::rc::Rc;

#[allow(non_snake_case)]
pub struct DNSQuestionConstructor {
    pub QNAME: String,
    pub QTYPE: u16,
    pub QCLASS: u16,
}

impl DNSQuestionConstructor {
    #[allow(non_snake_case)]
    pub fn new(QNAME: String, QTYPE: u16, QCLASS: u16) -> Self {
        Self {
            QNAME,
            QTYPE,
            QCLASS,
        }
    }

    pub fn construct(self) -> DNSQuestion {
        DNSQuestion {
            QNAME: Rc::new(Domain::from(&self.QNAME)),
            QTYPE: self.QTYPE,
            QCLASS: self.QCLASS,
        }
    }
}
