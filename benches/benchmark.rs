#![allow(unused_imports)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dns_core::bench_func::*;
use dns_core::{Request};
#[cfg(feature = "logger")]
use dns_core::error::init_logger;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        #[cfg(feature = "logger")]
        init_logger();
        b.iter(|| {
            black_box(test_decode_from());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
