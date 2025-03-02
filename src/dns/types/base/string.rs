use crate::dns::utils::SliceReader;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct RawString {
    str: Vec<u8>,
}

impl RawString {
    pub fn from_reader_with_maximum(reader: &mut SliceReader, maximum: usize) -> Option<RawString> {
        let len = reader.peek_u8();
        if reader.pos() + len as usize > maximum {
            return None;
        }
        reader.skip(1);
        Some(RawString {
            str: Vec::from(reader.read_slice(len as usize)),
        })
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(self.str.as_slice()).to_string()
    }
}

impl<'a> Display for RawString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::types::base::RawString;
    use crate::dns::utils::SliceReader;

    #[test]
    fn test_raw_string() {
        let slice = [
            0x94_u8, 0x20, 0x81, 0x80, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x02, 0x66,
            0x73, 0x09, 0x67, 0x6c, 0x6f, 0x72, 0x79, 0x6f, 0x75, 0x74, 0x68, 0x03, 0x63, 0x6f,
            0x6d, 0x00, 0x00, 0x10, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x10, 0x00, 0x01, 0x00, 0x00,
            0x02, 0x30, 0x00, 0x37, 0x36, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74,
            0x69, 0x6f, 0x6e, 0x2d, 0x63, 0x6f, 0x64, 0x65, 0x2d, 0x73, 0x69, 0x74, 0x65, 0x2d,
            0x41, 0x70, 0x70, 0x5f, 0x66, 0x65, 0x69, 0x73, 0x68, 0x75, 0x3d, 0x34, 0x7a, 0x43,
            0x44, 0x59, 0x74, 0x73, 0x77, 0x51, 0x46, 0x48, 0x43, 0x71, 0x69, 0x6e, 0x79, 0x78,
            0x64, 0x61, 0x61, 0xc0, 0x0c, 0x00, 0x10, 0x00, 0x01, 0x00, 0x00, 0x02, 0x30, 0x00,
            0x2c, 0x2b, 0x76, 0x3d, 0x73, 0x70, 0x66, 0x31, 0x20, 0x2b, 0x69, 0x6e, 0x63, 0x6c,
            0x75, 0x64, 0x65, 0x3a, 0x5f, 0x6e, 0x65, 0x74, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73,
            0x2e, 0x6d, 0x2e, 0x66, 0x65, 0x69, 0x73, 0x68, 0x75, 0x2e, 0x63, 0x6e, 0x20, 0x2d,
            0x61, 0x6c, 0x6c,
        ];
        let mut reader = SliceReader::from_slice(&slice);
        reader.set_pos(46);
        let string = RawString::from_reader_with_maximum(&mut reader, 46 + 55).unwrap();
        assert_eq!(
            string.str,
            &[
                118, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 45, 99, 111, 100, 101,
                45, 115, 105, 116, 101, 45, 65, 112, 112, 95, 102, 101, 105, 115, 104, 117, 61, 52,
                122, 67, 68, 89, 116, 115, 119, 81, 70, 72, 67, 113, 105, 110, 121, 120, 100, 97,
                97
            ]
        );
    }
}
