use crate::dns::parts::question::question::DNSQuestion;
use crate::dns::parts::*;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(unused)]
#[derive(Debug)]
pub enum QuestionBody {
    Single(DNSQuestion),
    Multi(Vec<DNSQuestion>),
}

impl QuestionBody {
    pub const ESTIMATE_SIZE_FOR_ONE : usize = DNSQuestion::ESTIMATE_SIZE;
    
    pub fn from_reader(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
        qdcount: u16,
    ) -> Self {
        if qdcount == 1 {
            return QuestionBody::Single(DNSQuestion::from_reader(reader, map));
        }
        let mut vec = Vec::with_capacity(qdcount as usize);
        for _ in 0..qdcount {
            vec.push(DNSQuestion::from_reader(reader, map));
        }
        QuestionBody::Multi(vec)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_from_reader() {
        todo!()
    }
}
