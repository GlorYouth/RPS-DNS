#![allow(unused_imports)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dns_core::bench_func::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        for _ in 0..2 {
            test()
        };
        b.iter(|| black_box(test()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
