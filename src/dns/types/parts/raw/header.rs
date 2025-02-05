#![cfg_attr(debug_assertions, allow(dead_code))]
use crate::dns::utils::SliceReader;

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
    pub fn get_response(&self) -> u8 {
        self.slice[2] >> 7
    }

    #[inline]
    pub fn get_opcode(&self) -> u8 {
        (self.slice[2] << 1) >> 4
    }

    #[inline]
    pub fn get_truncated(&self) -> u8 {
        (self.slice[2] & 0b0000_0010) >> 2
    }

    #[inline]
    pub fn get_rec_desired(&self) -> u8 {
        self.slice[2] & 0b0000_0001
    }

    #[inline]
    pub fn get_z(&self) -> u8 {
        (self.slice[3] << 1) >> 7
    }

    #[inline]
    pub fn get_check_disable(&self) -> u8 {
        (self.slice[3] << 3) >> 7
    }

    #[inline]
    pub fn get_questions(&self) -> u16 {
        u16::from_be_bytes(self.slice[4..6].try_into().unwrap())
    }

    #[inline]
    pub fn get_answer_rrs(&self) -> u16 {
        u16::from_be_bytes(self.slice[6..8].try_into().unwrap())
    }

    #[inline]
    pub fn get_authority_rrs(&self) -> u16 {
        u16::from_be_bytes(self.slice[8..10].try_into().unwrap())
    }

    #[inline]
    pub fn get_additional_rrs(&self) -> u16 {
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

#[derive(Debug)]
pub struct RawResponseHeader<'a> {
    slice: &'a [u8],
}

impl<'a> RawResponseHeader<'a> {
    pub const SIZE: usize = 12;

    #[inline]
    pub fn new(reader: &mut SliceReader<'a>) -> RawResponseHeader<'a> {
        RawResponseHeader::from(reader)
    }

    #[inline]
    pub fn get_id(&self) -> u16 {
        u16::from_be_bytes(self.slice[0..2].try_into().unwrap())
    }

    #[inline]
    pub fn get_response(&self) -> u8 {
        self.slice[2] >> 7
    }

    #[inline]
    pub fn get_opcode(&self) -> u8 {
        (self.slice[2] << 1) >> 4
    }

    #[inline]
    pub fn get_authoritative(&self) -> u8 {
        (self.slice[2] & 0b0000_0100) >> 2
    }

    #[inline]
    pub fn get_truncated(&self) -> u8 {
        (self.slice[2] & 0b0000_0010) >> 1
    }

    #[inline]
    pub fn get_rec_desired(&self) -> u8 {
        self.slice[2] & 0b0000_0001
    }

    #[inline]
    pub fn get_rec_avail(&self) -> u8 {
        self.slice[3] >> 7
    }

    #[inline]
    pub fn get_z(&self) -> u8 {
        (self.slice[3] << 1) >> 7
    }

    #[inline]
    pub fn get_authenticated(&self) -> u8 {
        (self.slice[3] << 2) >> 7
    }

    #[inline]
    pub fn get_check_disable(&self) -> u8 {
        (self.slice[3] << 3) >> 7
    }

    #[inline]
    pub fn get_rcode(&self) -> u8 {
        (self.slice[3] << 4) >> 4
    }

    #[inline]
    pub fn get_questions(&self) -> u16 {
        u16::from_be_bytes(self.slice[4..6].try_into().unwrap())
    }

    #[inline]
    pub fn get_answer_rrs(&self) -> u16 {
        u16::from_be_bytes(self.slice[6..8].try_into().unwrap())
    }

    #[inline]
    pub fn get_authority_rrs(&self) -> u16 {
        u16::from_be_bytes(self.slice[8..10].try_into().unwrap())
    }

    #[inline]
    pub fn get_additional_rrs(&self) -> u16 {
        u16::from_be_bytes(self.slice[10..12].try_into().unwrap())
    }
}

impl<'a> From<&mut SliceReader<'a>> for RawResponseHeader<'a> {
    #[inline]
    fn from(reader: &mut SliceReader<'a>) -> Self {
        RawResponseHeader::from(reader.read_slice(Self::SIZE))
    }
}

impl<'a> From<&'a [u8]> for RawResponseHeader<'a> {
    #[inline]
    fn from(slice: &'a [u8]) -> Self {
        RawResponseHeader { slice }
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::types::parts::raw::{RawRequestHeader, RawResponseHeader};
    use crate::dns::utils::SliceReader;

    #[test]
    fn test_request_header() {
        let mut reader = SliceReader::from(
            &[
                0x75, 0xb3, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x64,
                0x6e, 0x73, 0x06, 0x77, 0x65, 0x69, 0x78, 0x69, 0x6e, 0x02, 0x71, 0x71, 0x03, 0x63,
                0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01,
            ][..],
        );
        let header = RawRequestHeader::new(&mut reader);
        assert_eq!(header.get_id(), 0x75b3);
        assert_eq!(header.get_response(), 0x00);
        assert_eq!(header.get_opcode(), 0x00);
        assert_eq!(header.get_truncated(), 0x00);
        assert_eq!(header.get_rec_desired(), 0x01);
        assert_eq!(header.get_z(), 0x00);
        assert_eq!(header.get_check_disable(), 0x00);
        assert_eq!(header.get_questions(), 0x01);
        assert_eq!(header.get_answer_rrs(), 0x00);
        assert_eq!(header.get_authority_rrs(), 0x00);
        assert_eq!(header.get_additional_rrs(), 0x00);
    }

    #[test]
    fn test_response_header() {
        let mut reader = SliceReader::from(
            &[
                0x0f, 0x04, 0x81, 0x80, 0x00, 0x01, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x06, 0x63,
                0x6f, 0x6e, 0x66, 0x69, 0x67, 0x04, 0x65, 0x64, 0x67, 0x65, 0x05, 0x73, 0x6b, 0x79,
                0x70, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00,
                0x05, 0x00, 0x01, 0x00, 0x00, 0x22, 0x30, 0x00, 0x2a, 0x06, 0x63, 0x6f, 0x6e, 0x66,
                0x69, 0x67, 0x04, 0x65, 0x64, 0x67, 0x65, 0x05, 0x73, 0x6b, 0x79, 0x70, 0x65, 0x03,
                0x63, 0x6f, 0x6d, 0x0e, 0x74, 0x72, 0x61, 0x66, 0x66, 0x69, 0x63, 0x6d, 0x61, 0x6e,
                0x61, 0x67, 0x65, 0x72, 0x03, 0x6e, 0x65, 0x74, 0x00, 0xc0, 0x33, 0x00, 0x05, 0x00,
                0x01, 0x00, 0x00, 0x00, 0x3a, 0x00, 0x10, 0x06, 0x6c, 0x2d, 0x30, 0x30, 0x30, 0x37,
                0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0xc0, 0x18, 0xc0, 0x69, 0x00, 0x05, 0x00,
                0x01, 0x00, 0x00, 0x21, 0x3e, 0x00, 0x24, 0x11, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
                0x2d, 0x65, 0x64, 0x67, 0x65, 0x2d, 0x73, 0x6b, 0x79, 0x70, 0x65, 0x06, 0x6c, 0x2d,
                0x30, 0x30, 0x30, 0x37, 0x08, 0x6c, 0x2d, 0x6d, 0x73, 0x65, 0x64, 0x67, 0x65, 0xc0,
                0x58, 0xc0, 0x85, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x02, 0xc0,
                0x97, 0xc0, 0x97, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x92, 0x00, 0x04, 0x0d,
                0x6b, 0x2a, 0x10,
            ][..],
        );
        let header = RawResponseHeader::new(&mut reader);
        assert_eq!(header.get_id(), 0x0f04);
        assert_eq!(header.get_response(), 0x01);
        assert_eq!(header.get_opcode(), 0x00);
        assert_eq!(header.get_authoritative(), 0x00);
        assert_eq!(header.get_truncated(), 0x00);
        assert_eq!(header.get_rec_desired(), 0x01);
        assert_eq!(header.get_rec_avail(), 0x01);
        assert_eq!(header.get_z(), 0x00);
        assert_eq!(header.get_authenticated(), 0x00);
        assert_eq!(header.get_check_disable(), 0x00);
        assert_eq!(header.get_rcode(), 0x00);
        assert_eq!(header.get_questions(), 0x01);
        assert_eq!(header.get_answer_rrs(), 0x05);
        assert_eq!(header.get_authority_rrs(), 0x00);
        assert_eq!(header.get_additional_rrs(), 0x00);
    }
}
