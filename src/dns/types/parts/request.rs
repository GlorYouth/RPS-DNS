#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::types::parts::header::RequestHeader;
use crate::dns::types::parts::question::Question;
use crate::dns::utils::SliceOperator;
use smallvec::SmallVec;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::dns::types::base::RawDomain;

const SUFFIX: &[u8] = "xn--".as_bytes();

pub struct Request {
    pub header: RequestHeader,
    pub question: SmallVec<[Question; 1]>,
}

impl Request {
    #[inline]
    pub fn new(domain: Rc<RawDomain>, qtype: u16) -> Request {
        let mut question = SmallVec::new();
        question.push(Question {
            qname: domain.clone(),
            qtype,
            qclass: 1,
        });

        Request {
            header: Default::default(),
            question,
        }
    }

    pub fn encode_to_udp<'b>(&self, buffer: &'b mut [u8]) -> &'b [u8] {
        let mut operator = SliceOperator::from_slice(buffer);

        // 前两个Bytes
        operator.set_pos(2);
        operator.write_u16(self.header.id);

        operator.write_u16(self.header.get_flags());
        operator.write_u16(self.question.len() as u16);
        operator.write_u32(0);
        operator.write_u16(0);
        self.encode_question(&mut operator);
        let pos = operator.pos();
        if pos - 2 > 512 {
            //自动返回tcp的slice
            buffer[0..2].copy_from_slice(((pos - 2) as u16).to_be_bytes().as_ref());
            return buffer[..pos].as_ref();
        }
        buffer[2..pos].as_ref()
    }

    pub fn encode_to_tcp<'b>(&self, buffer: &'b mut [u8]) -> &'b [u8] {
        let mut operator = SliceOperator::from_slice(buffer);
        operator.set_pos(2);
        operator.write_u16(self.header.id);
        operator.write_u16(self.header.get_flags());
        operator.write_u16(self.question.len() as u16);
        operator.write_u32(0);
        operator.write_u16(0);
        self.encode_question(&mut operator);
        let pos = operator.pos();
        buffer[0..2].copy_from_slice(((pos - 2) as u16).to_be_bytes().as_ref());
        buffer[..pos].as_ref()
    }

    fn encode_question(&self, operator: &mut SliceOperator) -> Option<()> {
        for q in &self.question {
            operator.write_slice(q.qname.as_ref().as_ref());
            operator.write_u8(0x0);
            operator.write_u16(q.qtype);
            operator.write_u16(q.qclass);
        }
        Some(())
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.header, f)?;
        writeln!(f, "\tQuestions: {}", self.question.len())?;
        writeln!(f, "\tAnswer RRs: 0")?;
        writeln!(f, "\tAuthority RRs: 0")?;
        writeln!(f, "\tAdditional RRs: 0")?;
        writeln!(f, "Queries:")?;
        for q in &self.question {
            Display::fmt(&q, f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::dns::Request;
    use crate::dns::{DnsTypeNum, RawDomain};

    #[test]
    fn test_fmt() {
        let request = Request::new(Rc::new(RawDomain::from_str("www.baidu.com").unwrap()),DnsTypeNum::A);
        println!("{}", request);
    }
}
