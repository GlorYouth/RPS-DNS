#![allow(non_snake_case, dead_code)]
use crate::{ArrayU8, FlagsFormat, SliceReader};
use generic_array::typenum::U2;

#[derive(Debug)]
pub struct FlagsData(pub ArrayU8<U2>);

impl FlagsData {
    pub fn from_reader(reader: &mut SliceReader) -> FlagsData {
        FlagsData(ArrayU8::<U2>::from_reader(reader))
    }

    pub fn resolve(&self) -> FlagsFormat {
        FlagsFormat {
            QR: self.get_QR(),
            Opcode: self.get_Opcode(),
            AA: self.get_AA(),
            TC: self.get_TC(),
            RD: self.get_RD(),
            RA: self.get_RA(),
            Z: self.get_Z(),
            RCODE: self.get_RCODE(),
        }
    }

    #[inline]
    pub fn get_QR(&self) -> u8 {
        self.0[0] >> 7
    }

    #[inline]
    pub fn get_Opcode(&self) -> u8 {
        (self.0[0] << 1) >> 4
    }

    #[inline]
    pub fn get_AA(&self) -> u8 {
        (self.0[0] & 0b0000_0100) >> 2
    }

    #[inline]
    pub fn get_TC(&self) -> u8 {
        (self.0[0] & 0b0000_0010) >> 1
    }

    #[inline]
    pub fn get_RD(&self) -> u8 {
        self.0[0] & 0b0000_0001
    }

    #[inline]
    pub fn get_RA(&self) -> u8 {
        self.0[1] >> 7
    }

    #[inline]
    pub fn get_Z(&self) -> u8 {
        (self.0[1] << 1) >> 4
    }

    #[inline]
    pub fn get_RCODE(&self) -> u8 {
        self.0[1] & 0b0000_0111
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl From<[u8; 2]> for FlagsData {
    fn from(arr: [u8; 2]) -> Self {
        FlagsData(ArrayU8::<U2>::from(arr))
    }
}

impl From<&[u8]> for FlagsData {
    fn from(arr: &[u8]) -> Self {
        FlagsData(ArrayU8::<U2>::from(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_flags_format() {
        let flags = FlagsData(ArrayU8::from(&[0b1000_0001, 0b1000_0000][..]));
        let flags_format = flags.resolve();
        assert_eq!(flags_format.QR, 0b0000_0001);
        assert_eq!(flags_format.Opcode, 0b0000_0000);
        assert_eq!(flags_format.AA, 0b0000_0000);
        assert_eq!(flags_format.TC, 0b0000_0000);
        assert_eq!(flags_format.RD, 0b0000_0001);
        assert_eq!(flags_format.RA, 0b0000_0001);
        assert_eq!(flags_format.Z, 0b0000_0000);
        assert_eq!(flags_format.RCODE, 0b0000_0000);
    }
}
