use crate::*;
use std::any::Any;

#[allow(dead_code)]
pub fn bench_decode() {
    for _ in 0..20000 {
        let reader = &mut SliceReader::from_slice(&[
            0xb4_u8, 0xdb, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x03, 0x64,
            0x6e, 0x73, 0x06, 0x77, 0x65, 0x69, 0x78, 0x69, 0x6e, 0x02, 0x71, 0x71, 0x03, 0x63,
            0x6f, 0x6d, 0x02, 0x63, 0x6e, 0x00, 0x00, 0x1c, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x1c,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00, 0x02,
            0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0xc0, 0x0c, 0x00, 0x1c,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00, 0x10,
            0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0xc0, 0x0c, 0x00, 0x1c,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00, 0x02,
            0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0xc0, 0x0c, 0x00, 0x1c,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00, 0x10,
            0x10, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25,
        ]);
        let answer = DNSAnswer::from_reader(reader).unwrap();
        answer.type_id();
    }
}
#[allow(dead_code)]
fn bench_encode_domain() {
    for _ in 0..20 {
        let _ = Domain::from("小米.中国");
    }
}
#[allow(dead_code)]
fn bench_decode_domain() {
    for _ in 0..20 {
        Domain::new(
            [
                0x0b, 0x78, 0x6e, 0x2d, 0x2d, 0x79, 0x65, 0x74, 0x73, 0x37, 0x36, 0x65, 0x0a, 0x78,
                0x6e, 0x2d, 0x2d, 0x66, 0x69, 0x71, 0x73, 0x38, 0x73, 0x00,
            ]
            .to_vec(),
        )
        .to_string()
        .unwrap();
    }
}
