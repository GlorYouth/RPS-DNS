extern crate core;

use dns_core::query;
use dns_core::query_result_map;
#[allow(unused_imports)]
use dns_core::query_result_map_err;
#[allow(unused_imports)]
use dns_core::query_type_map;
use dns_core::record_filter;
use dns_core::test_decode_from;

fn main() {
    let result = query! {
        A,
        into_iter,
        @target "www.baidu.com".to_string(),
        @server vec!["9.9.9.9".to_string()]
    };
    println!("{:?}", result);
    test_decode_from();
    return;
}
