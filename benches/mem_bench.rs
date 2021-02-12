use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

fn mem_benchmark(c: &mut Criterion) {
    c.bench_function("memory bench", |b| {
        let n = 128 * 1024 * 1024;
        let mut v = Vec::<u8>::with_capacity(n);
        b.iter(|| {
            v.as_mut_slice().fill(0);
        });
    });
}

criterion_group!(benches, mem_benchmark);
criterion_main!(benches);
