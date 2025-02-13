#![allow(unused_imports)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dns_core::bench_func::*;
#[cfg(feature = "logger")]
use dns_core::dns::error::init_logger;
use dns_core::dns::types::parts::Request;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        #[cfg(feature = "logger")]
        init_logger();
        b.iter(|| test_decode_from())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
