use generic_array::typenum::U2;
use crate::{ArrayU8, FlagsFormat, SliceReader};

#[derive(Debug)]
pub struct Flags(pub ArrayU8<U2>);


impl Flags {
    pub fn from_reader(reader: &mut SliceReader) -> Flags {
        Flags(ArrayU8::<U2>::from_reader(reader))
    }
    
    pub fn resolve(&self) -> FlagsFormat {
        FlagsFormat {
            QR: self.0[0] >> 7,
            Opcode: (self.0[0] << 1) >> 4,
            AA: (self.0[0] & 0b0000_0100) >> 2,
            TC: (self.0[0] & 0b0000_0010) >> 1,
            RD: self.0[0] & 0b0000_0001,
            RA: self.0[1] >> 7,
            Z: (self.0[1] << 1) >> 4,
            RCODE: self.0[1] & 0b0000_0111,
        }
    }
    
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl From<[u8;2]> for Flags {
    fn from(arr: [u8;2]) -> Self {
        Flags(ArrayU8::<U2>::from(arr))
    }
}

impl From<&[u8]> for Flags {
    fn from(arr: &[u8]) -> Self {
        Flags(ArrayU8::<U2>::from(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_flags_format() {
        let flags = Flags(ArrayU8::from(&[0b1000_0001, 0b1000_0000][..]));
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