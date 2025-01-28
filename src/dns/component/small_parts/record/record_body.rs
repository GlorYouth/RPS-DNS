#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::dns::component::small_parts::record::record::DNSRecord;
use crate::dns::component::*;
use crate::dns::error::Error;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct RecordBody(pub Vec<DNSRecord>);

impl RecordBody {
    #[inline]
    pub fn from_reader_ret_err(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
        count: u16,
    ) -> Result<RecordBody, Error> {
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..count {
            records.push(DNSRecord::from_reader_ret_err(reader, map)?);
        }
        Ok(RecordBody(records))
    }

    #[inline]
    pub fn from_reader(
        reader: &mut SliceReader,
        map: &mut HashMap<u16, Rc<Domain>>,
        count: u16,
    ) -> Option<RecordBody> {
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..count {
            records.push(DNSRecord::from_reader(reader, map)?);
        }
        Option::from(RecordBody(records))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_reader_ret_err() {
        let mut map = HashMap::new();
        map.insert(0xc00c_u16, Rc::new(Domain::from("ocsp.sectigo.com")));
        let reader = &mut SliceReader::from_slice(&[
            0xa8, 0xe1, 0x81, 0x80, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x04, 0x6f,
            0x63, 0x73, 0x70, 0x07, 0x73, 0x65, 0x63, 0x74, 0x69, 0x67, 0x6f, 0x03, 0x63, 0x6f,
            0x6d, 0x00, 0x00, 0x1c, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00,
            0x0e, 0x06, 0x00, 0x26, 0x04, 0x6f, 0x63, 0x73, 0x70, 0x08, 0x63, 0x6f, 0x6d, 0x6f,
            0x64, 0x6f, 0x63, 0x61, 0x03, 0x63, 0x6f, 0x6d, 0x03, 0x63, 0x64, 0x6e, 0x0a, 0x63,
            0x6c, 0x6f, 0x75, 0x64, 0x66, 0x6c, 0x61, 0x72, 0x65, 0x03, 0x6e, 0x65, 0x74, 0x00,
            0xc0, 0x2e, 0x00, 0x1c, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2c, 0x00, 0x10, 0x26, 0x06,
            0x47, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x12, 0x26, 0xe9,
            0xc0, 0x2e, 0x00, 0x1c, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2c, 0x00, 0x10, 0x26, 0x06,
            0x47, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xac, 0x40, 0x95, 0x17,
        ]);
        reader.set_pos(34);
        let records = RecordBody::from_reader_ret_err(reader, &mut map, 3);
        let records = records.unwrap();
        assert_eq!(
            records.0[0].NAME.to_string().unwrap(),
            Domain::from("ocsp.sectigo.com").to_string().unwrap()
        );
        assert_eq!(records.0[0].TYPE, 5);
        assert_eq!(records.0[0].CLASS, 1);
        assert_eq!(records.0[0].TTL, 3590);
        assert_eq!(records.0[0].RDLENGTH, 38);
        assert_eq!(
            Domain::from(records.0[0].RDATA.to_bytes())
                .to_string()
                .unwrap(),
            "ocsp.comodoca.com.cdn.cloudflare.net"
        );

        assert_eq!(
            records.0[1].NAME.to_string().unwrap(),
            Domain::from("ocsp.comodoca.com.cdn.cloudflare.net")
                .to_string()
                .unwrap()
        );
        assert_eq!(records.0[1].TYPE, 28);
        assert_eq!(records.0[1].CLASS, 1);
        assert_eq!(records.0[1].TTL, 300);
        assert_eq!(records.0[1].RDLENGTH, 16);
        assert_eq!(
            records.0[1].RDATA.to_bytes(),
            vec![
                0x26, 0x06, 0x47, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x12,
                0x26, 0xe9
            ]
        );

        assert_eq!(
            records.0[2].NAME.to_string().unwrap(),
            Domain::from("ocsp.comodoca.com.cdn.cloudflare.net")
                .to_string()
                .unwrap()
        );
        assert_eq!(records.0[2].TYPE, 28);
        assert_eq!(records.0[2].CLASS, 1);
        assert_eq!(records.0[2].TTL, 300);
        assert_eq!(records.0[2].RDLENGTH, 16);
        assert_eq!(
            records.0[2].RDATA.to_bytes(),
            vec![
                0x26, 0x06, 0x47, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xac, 0x40,
                0x95, 0x17
            ]
        );
    }
}
