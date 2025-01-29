#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::types::raw::domain::RawDomain;
use crate::dns::utils::SliceReader;
use std::collections::HashMap;
use std::rc::Rc;

pub enum RawQuestionType<'a> {
    Single(RawQuestion<'a>),
    Multiple(Vec<RawQuestion<'a>>),
    None,
}

#[derive(Debug)]
pub struct RawQuestion<'a> {
    name: Rc<RawDomain<'a>>,
    other: &'a [u8],
}

impl<'a> RawQuestion<'a> {
    pub const FIX_SIZE: usize = 4;
    pub const LEAST_SIZE: usize = Self::FIX_SIZE + 2;

    pub fn new<'b>( // 'b为引用存在的周期，比'a对象存在的周期短或等于
        reader: &'b mut SliceReader<'a>,
        map: &'b mut HashMap<u16, RawDomain<'a>>,
    ) -> Option<RawQuestion<'a>> {
        let name = RawDomain::new(reader, map)?;
        let len = reader.len();
        if reader.pos() + Self::FIX_SIZE > len {
            return None; //检测出界，防止panic
        }
        Some(RawQuestion {
            name: Rc::from(name),
            other: reader.read_slice(Self::FIX_SIZE),
        })
    }
}
