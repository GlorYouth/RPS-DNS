extern crate core;

use dns_core::query;
use dns_core::query_type_map;
use dns_core::query_result_map_err;
use dns_core::test_decode_from;


fn main() {
    let server = vec!["9.9.9.9".to_string()];
    let result = query! {
            A,
            all,
            @target "www.baidu.com".to_string(),
            @server server,
            -feature error
        };
    println!("{:?}", result.get_error().unwrap());
    test_decode_from();
    return;
}
