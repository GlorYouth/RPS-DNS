use crate::{DnsType, Request};

pub fn test() {
    let mut buf = [0_u8; 1500];
    for _ in 0..20000 {
        let arr = Request::new("www.google.com".to_string(), DnsType::A.into())
            .encode_into(&mut buf)
            .unwrap();
        assert_eq!(arr.len(), 32);
    }
}