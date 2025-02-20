#![allow(unused_imports)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rps_dns::bench_func::*;
#[cfg(feature = "logger")]
use rps_dns::error::init_logger;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        #[cfg(feature = "logger")]
        init_logger();
        b.iter(|| test_query())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
