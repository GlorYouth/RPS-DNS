#![allow(unused_imports)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dns_core::bench_func::*;
use dns_core::{DnsType, Request};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| {
            test()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
