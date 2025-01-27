use dns_core::bench_func::bench_decode;

fn main() {
    for _ in 0..20 {
        bench_decode();
    }
}