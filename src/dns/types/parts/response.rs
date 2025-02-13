#![cfg_attr(debug_assertions, allow(dead_code))]

#[cfg(feature = "fmt")]
use crate::dns::types::parts::RecordFmtType;
use crate::dns::types::parts::Request;
use crate::dns::types::parts::header::{HEADER_SIZE, ResponseHeader};
use crate::dns::types::parts::question::Question;
use crate::dns::types::parts::record::Record;
use crate::dns::utils::SliceReader;

#[cfg(feature = "result_error")]
use crate::dns::error::error_trait;
#[cfg(feature = "logger")]
use log::{debug, trace};
use smallvec::SmallVec;
#[cfg(feature = "fmt")]
use std::fmt::Display;

#[derive(Debug)]
pub struct Response {
    pub header: ResponseHeader,
    pub question: SmallVec<[Question; 1]>,
    pub answer: Vec<Record>,
}

#[cfg(feature = "result_error")]
impl error_trait::A for Response {}

impl Response {
    #[inline]
    pub fn from_slice_uncheck(slice: &[u8]) -> Option<Response> {
        Self::from_slice_check(slice, |_| Some(()))
    }

    fn from_slice_check<F: Fn(&ResponseHeader) -> Option<()>>(
        slice: &[u8],
        check: F,
    ) -> Option<Response> {
        if slice.len() < HEADER_SIZE + Question::LEAST_SIZE {
            #[cfg(feature = "logger")]
            {
                debug!(
                    "传入Slice长度不符合最低标准RawResponse, 输入Slice长度 {}, 需要至少 {}",
                    slice.len(),
                    HEADER_SIZE + Question::LEAST_SIZE
                );
            }

            return None;
        }
        let mut reader = SliceReader::from_slice(slice);
        #[cfg(feature = "logger")]
        {
            trace!("开始解析Header")
        }
        let header = ResponseHeader::from(&mut reader);
        check(&header)?;

        let mut questions = SmallVec::new();
        let total: usize = header.answer_rrs as usize
            + header.authority_rrs as usize
            + header.additional_rrs as usize;
        let mut rrs = Vec::with_capacity(total);

        for _i in 0..header.questions {
            #[cfg(feature = "logger")]
            {
                trace!("正在从Slice解析第{}个RawQuestion", _i);
            }
            questions.push(Question::new(&mut reader)?)
        }

        #[cfg(not(feature = "logger"))]
        for _ in 0..total {
            rrs.push(Record::new(&mut reader)?);
        }

        #[cfg(feature = "logger")]
        {
            for _i in 0..header.answer_rrs {
                trace!("正在从Slice解析RawRecord=>第{}个response", _i);
                rrs.push(Record::new(&mut reader)?);
            }

            for _i in 0..header.authority_rrs {
                trace!("正在从Slice解析RawRecord=>第{}个authority", _i);
                rrs.push(Record::new(&mut reader)?);
            }

            for _i in 0..header.additional_rrs {
                trace!("正在从Slice解析RawRecord=>第{}个additional", _i);
                rrs.push(Record::new(&mut reader)?);
            }
        }

        Some(Response {
            header,
            question: questions,
            answer: rrs,
        })
    }

    #[inline]
    pub fn from_slice(slice: &[u8], request: &Request) -> Option<Response> {
        Self::from_slice_check(slice, |header| {
            if header.id != request.header.id {
                #[cfg(feature = "logger")]
                {
                    trace!(
                        "请求id和响应id不同,分别为{},{}",
                        header.id, request.header.id
                    );
                }
                return None;
            }
            if header.response != 0x1 {
                #[cfg(feature = "logger")]
                {
                    trace!("响应的response flag非0x1");
                }
                return None;
            }
            if header.opcode != request.header.opcode {
                #[cfg(feature = "logger")]
                {
                    trace!(
                        "请求和响应的opcode不同,分别为{},{}",
                        header.opcode, request.header.opcode
                    );
                }
                return None;
            }
            if header.rec_desired != request.header.rec_desired {
                #[cfg(feature = "logger")]
                {
                    trace!(
                        "请求和响应的rec_desired不同,分别为{},{}",
                        header.rec_desired, request.header.rec_desired
                    );
                }
                return None;
            }
            if header.rcode != 0x0 {
                #[cfg(feature = "logger")]
                {
                    trace!("响应的opcode不为0x0,而是{}", header.rcode);
                }
                return None;
            }
            if header.questions != request.question.len() as u16 {
                #[cfg(feature = "logger")]
                {
                    trace!(
                        "请求与响应的question数不同,分别为{},{}",
                        request.question.len(),
                        header.questions
                    );
                }
            }
            Some(())
        })
    }
}

#[cfg(feature = "fmt")]
impl Display for Response {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(&self.header, fmt)?;
        writeln!(fmt, "Queries:")?;
        for q in &self.question {
            Display::fmt(&q, fmt)?;
        }
        let iter = self.answer.iter();
        let mut iter_new = iter
            .clone()
            .filter(|r| matches!(r.get_fmt_type(), RecordFmtType::Answers))
            .peekable();

        if iter_new.peek().is_some() {
            writeln!(fmt, "Answers:")?;
        }
        iter_new.try_for_each(|x| Display::fmt(&x, fmt))?;

        let mut iter_new = iter
            .filter(|r| matches!(r.get_fmt_type(), RecordFmtType::Authoritative))
            .peekable();

        if iter_new.peek().is_some() {
            writeln!(fmt, "Authoritative nameservers:")?;
        }

        iter_new.try_for_each(|x| Display::fmt(&x, fmt))?;
        Ok(())
    }
}
