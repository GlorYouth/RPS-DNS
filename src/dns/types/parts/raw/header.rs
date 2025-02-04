#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::utils::SliceReader;

#[derive(Debug)]
pub struct RawAnswerHeader<'a> {
    slice: &'a [u8],
}

impl<'a> RawAnswerHeader<'a> {
    pub const SIZE: usize = 12;

    #[inline]
    pub fn new(reader: &mut SliceReader<'a>) -> RawAnswerHeader<'a> {
        RawAnswerHeader::from(reader)
    }

    #[inline]
    pub fn get_id(&self) -> u16 {
        u16::from_be_bytes(self.slice[0..2].try_into().unwrap())
    }

    #[inline]
    pub fn get_qr(&self) -> u8 {
        self.slice[2] >> 7
    }

    #[inline]
    pub fn get_opcode(&self) -> u8 {
        (self.slice[2] << 1) >> 4
    }

    #[inline]
    pub fn get_aa(&self) -> u8 {
        (self.slice[2] & 0b0000_0100) >> 2
    }

    #[inline]
    pub fn get_tc(&self) -> u8 {
        (self.slice[2] & 0b0000_0010) >> 1
    }

    #[inline]
    pub fn get_rd(&self) -> u8 {
        self.slice[2] & 0b0000_0001
    }

    #[inline]
    pub fn get_ra(&self) -> u8 {
        self.slice[3] >> 7
    }

    #[inline]
    pub fn get_z(&self) -> u8 {
        (self.slice[3] << 1) >> 7
    }

    #[inline]
    pub fn get_answer_authenticated(&self) -> u8 {
        (self.slice[3] << 2) >> 7
    }

    #[inline]
    pub fn get_non_authenticated(&self) -> u8 {
        (self.slice[3] << 3) >> 7
    }

    #[inline]
    pub fn get_rcode(&self) -> u8 {
        self.slice[3] & 0b0000_1111
    }

    #[inline]
    pub fn get_qdcount(&self) -> u16 {
        u16::from_be_bytes(self.slice[4..6].try_into().unwrap())
    }

    #[inline]
    pub fn get_ancount(&self) -> u16 {
        u16::from_be_bytes(self.slice[6..8].try_into().unwrap())
    }

    #[inline]
    pub fn get_nscount(&self) -> u16 {
        u16::from_be_bytes(self.slice[8..10].try_into().unwrap())
    }

    #[inline]
    pub fn get_arcount(&self) -> u16 {
        u16::from_be_bytes(self.slice[10..12].try_into().unwrap())
    }
}

impl<'a> From<&mut SliceReader<'a>> for RawAnswerHeader<'a> {
    #[inline]
    fn from(reader: &mut SliceReader<'a>) -> Self {
        RawAnswerHeader::from(reader.read_slice(Self::SIZE))
    }
}

impl<'a> From<&'a [u8]> for RawAnswerHeader<'a> {
    #[inline]
    fn from(slice: &'a [u8]) -> Self {
        RawAnswerHeader { slice }
    }
}

#[derive(Debug)]
pub struct RawRequestHeader<'a> {
    slice: &'a [u8],
}

impl<'a> RawRequestHeader<'a> {
    pub const SIZE: usize = 12;

    #[inline]
    pub fn new(reader: &mut SliceReader<'a>) -> RawRequestHeader<'a> {
        RawRequestHeader::from(reader)
    }

    #[inline]
    pub fn get_id(&self) -> u16 {
        u16::from_be_bytes(self.slice[0..2].try_into().unwrap())
    }

    #[inline]
    pub fn get_qr(&self) -> u8 {
        self.slice[2] >> 7
    }

    #[inline]
    pub fn get_opcode(&self) -> u8 {
        (self.slice[2] << 1) >> 4
    }

    #[inline]
    pub fn get_tc(&self) -> u8 {
        (self.slice[2] & 0b0000_0100) >> 2
    }

    #[inline]
    pub fn get_rd(&self) -> u8 {
        (self.slice[2] & 0b0000_0010) >> 2
    }

    #[inline]
    pub fn get_z(&self) -> u8 {
        (self.slice[3] << 1) >> 7
    }

    #[inline]
    pub fn get_non_authenticated(&self) -> u8 {
        (self.slice[3] << 3) >> 7
    }

    #[inline]
    pub fn get_qdcount(&self) -> u16 {
        u16::from_be_bytes(self.slice[4..6].try_into().unwrap())
    }

    #[inline]
    pub fn get_ancount(&self) -> u16 {
        u16::from_be_bytes(self.slice[6..8].try_into().unwrap())
    }

    #[inline]
    pub fn get_nscount(&self) -> u16 {
        u16::from_be_bytes(self.slice[8..10].try_into().unwrap())
    }

    #[inline]
    pub fn get_arcount(&self) -> u16 {
        u16::from_be_bytes(self.slice[10..12].try_into().unwrap())
    }
}

impl<'a> From<&mut SliceReader<'a>> for RawRequestHeader<'a> {
    #[inline]
    fn from(reader: &mut SliceReader<'a>) -> Self {
        RawRequestHeader::from(reader.read_slice(Self::SIZE))
    }
}

impl<'a> From<&'a [u8]> for RawRequestHeader<'a> {
    #[inline]
    fn from(slice: &'a [u8]) -> Self {
        RawRequestHeader { slice }
    }
}
