extern crate core;

use rps_dns::*;

fn main() {
    let result = query! {
        a,
        all,
        @target "www.baidu.com".to_string(),
        @server vec!["9.9.9.9".to_string()]
    };
    println!("{:?}", result);
    return;
}
