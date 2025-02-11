use crate::dns::utils::SliceReader;

#[derive(Clone, Debug)]
pub struct A(std::net::Ipv4Addr);

impl A {
    #[inline]
    pub fn get_index(&self) -> std::net::Ipv4Addr {
        self.0
    }

    #[cfg(feature = "fmt")]
    #[inline]
    pub fn fmt_with_suffix(&self, f: &mut std::fmt::Formatter, _indent: &str) -> std::fmt::Result {
        writeln!(f, "{_indent}A: {}", self.0)
    }

    #[inline]
    pub fn from_reader_with_size(reader: &mut SliceReader, size: usize) -> Option<Self> {
        Some(Self(std::net::Ipv4Addr::from(
            <[u8; 4]>::try_from(reader.read_slice(size)).ok()?,
        )))
    }
}

#[derive(Clone, Debug)]
pub struct AAAA(std::net::Ipv6Addr);

impl AAAA {
    #[inline]
    pub fn get_index(&self) -> std::net::Ipv6Addr {
        self.0
    }

    #[cfg(feature = "fmt")]
    #[inline]
    pub fn fmt_with_suffix(&self, f: &mut std::fmt::Formatter, _indent: &str) -> std::fmt::Result {
        writeln!(f, "{_indent}AAAA: {}", self.0)
    }

    #[inline]
    pub fn from_reader_with_size(reader: &mut SliceReader, size: usize) -> Option<Self> {
        Some(Self(std::net::Ipv6Addr::from(
            <[u8; 16]>::try_from(reader.read_slice(size)).ok()?,
        )))
    }
}
