use crate::{Answer, RawAnswer};
use small_map::SmallMap;

pub fn test() {
    let mut raw = RawAnswer::new(
        &[
            0xb4_u8, 0xdb, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x03,
            0x64, 0x6e, 0x73, 0x06, 0x77, 0x65, 0x69, 0x78, 0x69, 0x6e, 0x02, 0x71, 0x71, 0x03,
            0x63, 0x6f, 0x6d, 0x02, 0x63, 0x6e, 0x00, 0x00, 0x1c, 0x00, 0x01, 0xc0, 0x0c, 0x00,
            0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00,
            0x02, 0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0xc0, 0x0c, 0x00,
            0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00,
            0x10, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0xc0, 0x0c, 0x00,
            0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x06, 0x00,
            0x02, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0xc0, 0x0c, 0x00,
            0x1c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x10, 0x24, 0x08, 0x87, 0x11, 0x00,
            0x10, 0x10, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25,
        ][..],
    )
        .unwrap();
    let mut map = SmallMap::new();
    raw.init(&mut map, |_h| Some(())).unwrap();
    let answer = Answer::new(&raw).unwrap();
    println!("{:?}", answer);
}
