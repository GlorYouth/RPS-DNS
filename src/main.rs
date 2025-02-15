extern crate core;

use paste::paste;
use dns_core::query;
use dns_core::dns_type_num;
use dns_core::query_result_map;
use dns_core::query_type_map;
use dns_core::test_decode_from;

fn main() {
    let result = query! {
        a,
        all,
        @target "www.baidu.com".to_string(),
        @server vec!["9.9.9.9".to_string()]
    };
    println!("{:?}", result);
    test_decode_from();
    return;
}
