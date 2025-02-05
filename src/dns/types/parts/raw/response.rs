use crate::dns::types::parts::raw::header::RawResponseHeader;
use crate::dns::types::parts::raw::question::RawQuestion;
use crate::dns::types::parts::raw::record::RawRecord;
use crate::dns::utils::SliceReader;
#[cfg(debug_assertions)]
use log::{debug, trace};
use smallvec::SmallVec;

pub struct RawResponse<'a> {
    reader: SliceReader<'a>,

    raw_header: RawResponseHeader<'a>,
    raw_question: SmallVec<[RawQuestion<'a>; 5]>,
    response: SmallVec<[RawRecord<'a>; 10]>, //预分配，提升性能
    authority: SmallVec<[RawRecord<'a>; 5]>,
    additional: SmallVec<[RawRecord<'a>; 5]>,
}

impl<'a> RawResponse<'a> {
    #[inline]
    pub fn new(slice: &'a [u8]) -> Option<RawResponse<'a>> {
        if slice.len() < RawResponseHeader::SIZE + RawQuestion::LEAST_SIZE {
            #[cfg(debug_assertions)]
            {
                debug!(
                    "传入Slice长度不符合最低标准RawResponse, 输入Slice长度 {}, 需要至少 {}",
                    slice.len(),
                    RawResponseHeader::SIZE + RawQuestion::LEAST_SIZE
                );
            }
            return None;
        }
        #[cfg(debug_assertions)]
        {
            trace!("开始生成SliceReader")
        }
        let mut reader = SliceReader::from_slice(slice);
        #[cfg(debug_assertions)]
        {
            trace!("开始解析Header")
        }
        let raw_header = RawResponseHeader::new(&mut reader);
        #[cfg(debug_assertions)]
        {
            trace!("开始初始化RawResponse");
        }
        Some(RawResponse {
            reader,
            raw_header,
            raw_question: SmallVec::new(),
            response: SmallVec::new(),
            authority: SmallVec::new(),
            additional: SmallVec::new(),
        })
    }

    pub fn init_without_check(&mut self) -> Option<()> {
        let questions = self.raw_header.get_questions();
        let answer_rrs = self.raw_header.get_answer_rrs();
        let authority_rrs = self.raw_header.get_authority_rrs();
        let additional_rrs = self.raw_header.get_additional_rrs();

        for _i in 0..questions {
            #[cfg(debug_assertions)]
            {
                trace!("正在从Slice解析第{}个RawQuestion", _i);
            }
            self.raw_question.push(RawQuestion::new(&mut self.reader)?)
        }

        for _i in 0..answer_rrs {
            #[cfg(debug_assertions)]
            {
                trace!("正在从Slice解析RawRecord=>第{}个response", _i);
            }
            self.response.push(RawRecord::new(&mut self.reader)?);
        }

        for _i in 0..authority_rrs {
            #[cfg(debug_assertions)]
            {
                trace!("正在从Slice解析RawRecord=>第{}个authority", _i);
            }
            self.authority.push(RawRecord::new(&mut self.reader)?);
        }

        for _i in 0..additional_rrs {
            #[cfg(debug_assertions)]
            {
                trace!("正在从Slice解析RawRecord=>第{}个additional", _i);
            }
            self.additional.push(RawRecord::new(&mut self.reader)?);
        }

        Some(())
    }

    #[inline]
    pub fn init<'b, F: FnMut(&RawResponseHeader<'a>) -> Option<()>>(
        &'b mut self,
        mut check: F,
    ) -> Option<()> {
        #[cfg(debug_assertions)]
        {
            trace!("开始检查header部分");
        }
        if check(&self.raw_header).is_none() {
            #[cfg(debug_assertions)]
            {
                debug!("header检验失败");
            }
            return None;
        }
        self.init_without_check()
    }

    #[inline]
    pub fn get_raw_header(&self) -> &RawResponseHeader<'a> {
        &self.raw_header
    }

    #[inline]
    pub fn get_raw_question(&self) -> &SmallVec<[RawQuestion<'a>; 5]> {
        &self.raw_question
    }

    #[inline]
    pub fn get_raw_response(&self) -> &SmallVec<[RawRecord<'a>; 10]> {
        &self.response
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
    use crate::dns::types::parts::raw::response::RawResponse;

    #[test]
    fn test() {
        let mut raw = RawResponse::new(
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
        raw.init(|_h| Some(())).unwrap();
    }
}
