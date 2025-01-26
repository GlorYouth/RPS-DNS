use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dns_core::bench_func::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| black_box(bench_decode())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
