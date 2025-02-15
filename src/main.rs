extern crate core;

use rps_dns::paste;
use rps_dns::query;
use rps_dns::dns_type_num;
use rps_dns::query_result_map;
use rps_dns::query_type_map;
use rps_dns::test_decode_from;

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
