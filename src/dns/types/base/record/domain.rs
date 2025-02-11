use crate::dns::types::base::RawDomain;
use crate::dns::utils::SliceReader;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct NS(Rc<RawDomain>);

impl NS {

    #[inline]
    pub fn get_index(&self) -> Rc<RawDomain> {
        self.0.clone()
    }

    #[cfg(feature = "fmt")]
    #[inline]
    pub fn fmt_with_suffix(&self, f: &mut std::fmt::Formatter, _indent: &str) -> std::fmt::Result {
        writeln!(f, "{_indent}NS: {}", self.0)
    }

    #[inline]
    pub fn from_reader_with_size(reader: &mut SliceReader, _raw_len: usize) -> Option<Self> {
        Some(Self(Rc::new(RawDomain::from_reader(reader)?)))
    }
}

#[derive(Clone, Debug)]
pub struct CNAME(Rc<RawDomain>);

impl CNAME {
    #[inline]
    pub fn get_index(&self) -> Rc<RawDomain> {
        self.0.clone()
    }

    #[cfg(feature = "fmt")]
    #[inline]
    pub fn fmt_with_suffix(&self, f: &mut std::fmt::Formatter, _indent: &str) -> std::fmt::Result {
        writeln!(f, "{_indent}CNAME: {}", self.0)
    }

    #[inline]
    pub fn from_reader_with_size(reader: &mut SliceReader, _raw_len: usize) -> Option<Self> {
        Some(Self(Rc::new(RawDomain::from_reader(reader)?)))
    }
}
