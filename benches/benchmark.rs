#![allow(unused_imports)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dns_core::bench_func::*;
use dns_core::{DnsType, Request};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| {
            let mut buf = [0_u8; 1500];
            for _ in 0..20000 {
                let arr = Request::new("www.google.com".to_string(), DnsType::A.into())
                    .encode_into(&mut buf)
                    .unwrap();
                assert_eq!(arr.len(), 32);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
