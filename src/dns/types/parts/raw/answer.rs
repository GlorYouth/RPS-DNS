use log::{debug, trace};
use crate::dns::types::base::RawDomain;
use crate::dns::types::parts::raw::header::RawAnswerHeader;
use crate::dns::types::parts::raw::question::RawQuestion;
use crate::dns::types::parts::raw::record::RawRecord;
use crate::dns::utils::SliceReader;
use small_map::SmallMap;
use smallvec::SmallVec;

pub struct RawAnswer<'a> {
    reader: SliceReader<'a>,

    raw_header: RawAnswerHeader<'a>,
    raw_question: SmallVec<[RawQuestion<'a>; 5]>,
    answer: SmallVec<[RawRecord<'a>; 10]>, //预分配，提升性能
    authority: SmallVec<[RawRecord<'a>; 5]>,
    additional: SmallVec<[RawRecord<'a>; 5]>,
}

impl<'a> RawAnswer<'a> {
    #[inline]
    pub fn new(slice: &'a [u8]) -> Option<RawAnswer<'a>> {
        if slice.len() < RawAnswerHeader::SIZE + RawQuestion::LEAST_SIZE {
            #[cfg(debug_assertions)] {
                debug!("传入Slice长度不符合最低标准RawAnswer, 输入Slice长度 {}, 需要至少 {}", slice.len(), RawAnswerHeader::SIZE + RawQuestion::LEAST_SIZE);
            }
            return None;
        }
        #[cfg(debug_assertions)] {
            trace!("开始生成SliceReader")
        }
        let mut reader = SliceReader::from_slice(slice);
        #[cfg(debug_assertions)] {
            trace!("开始解析Header")
        }
        let raw_header = RawAnswerHeader::new(&mut reader);
        #[cfg(debug_assertions)] {
            trace!("开始初始化RawAnswer");
        }
        Some(RawAnswer {
            reader,
            raw_header,
            raw_question: SmallVec::new(),
            answer: SmallVec::new(),
            authority: SmallVec::new(),
            additional: SmallVec::new(),
        })
    }

    pub fn init<'b, F: FnMut(&RawAnswerHeader<'a>) -> Option<()>>(
        &'b mut self,
        mut map: &mut SmallMap<32, u16, RawDomain<'a>>,
        mut check: F,
    ) -> Option<()> {
        if check(&self.raw_header).is_none() {
            #[cfg(debug_assertions)] {
                debug!("外置header检验失败(check函数)");
            }
            return None;
        }
        let qdcount = self.raw_header.get_qdcount();
        let ancount = self.raw_header.get_ancount();
        let nscount = self.raw_header.get_nscount();
        let arcount = self.raw_header.get_arcount();

        for i in 0..qdcount {
            #[cfg(debug_assertions)] {
                trace!("正在从Slice解析第{}个RawQuestion",i);
            }
            self.raw_question
                .push(RawQuestion::new(&mut self.reader, map)?)
        }

        for i in 0..ancount {
            #[cfg(debug_assertions)] {
                trace!("正在从Slice解析RawRecord=>第{}个answer",i);
            }
            self.answer
                .push(RawRecord::new(&mut self.reader, &mut map)?);
        }

        for i in 0..nscount {
            #[cfg(debug_assertions)] {
                trace!("正在从Slice解析RawRecord=>第{}个authority",i);
            }
            self.authority
                .push(RawRecord::new(&mut self.reader, &mut map)?);
        }

        for i in 0..arcount {
            #[cfg(debug_assertions)] {
                trace!("正在从Slice解析RawRecord=>第{}个additional",i);
            }
            self.additional
                .push(RawRecord::new(&mut self.reader, &mut map)?);
        }

        Some(())
    }

    #[inline]
    pub fn get_raw_header(&self) -> &RawAnswerHeader<'a> {
        &self.raw_header
    }

    #[inline]
    pub fn get_raw_question(&self) -> &SmallVec<[RawQuestion<'a>; 5]> {
        &self.raw_question
    }

    #[inline]
    pub fn get_raw_answer(&self) -> &SmallVec<[RawRecord<'a>; 10]> {
        &self.answer
    }

    #[inline]
    pub fn get_raw_authority(&self) -> &SmallVec<[RawRecord<'a>; 5]> {
        &self.authority
    }

    #[inline]
    pub fn get_raw_additional(&self) -> &SmallVec<[RawRecord<'a>; 5]> {
        &self.additional
    }
}

#[cfg(test)]
mod test {
    use crate::dns::types::parts::raw::answer::RawAnswer;
    use small_map::SmallMap;

    #[test]
    fn test() {
        let mut raw = RawAnswer::new(
            &[
                0xb4_u8, 0xdb, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x03,
                0x64, 0x6e, 0x73, 0x06, 0x77, 0x65, 0x69, 0x78, 0x69, 0x6e, 0x02, 0x71, 0x71, 0x03,
                0x63, 0x6f, 0x6d, 0x02, 0x63, 0x6e, 0x00, 0x00, 0x1c, 0x00, 0x01, 0xc0, 0x0c, 0x00,
                0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00,
                0x02, 0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0xc0, 0x0c, 0x00,
                0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00,
                0x10, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0xc0, 0x0c, 0x00,
                0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00,
                0x02, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0xc0, 0x0c, 0x00,
                0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00,
                0x10, 0x10, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25,
            ][..],
        )
        .unwrap();
        let mut map = SmallMap::new();
        raw.init(&mut map, |_h| Some(())).unwrap();
    }
}
