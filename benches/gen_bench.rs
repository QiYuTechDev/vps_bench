use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};

#[inline(never)]
fn gen_bench_fn(rng: &mut rand::rngs::SmallRng, n: usize) -> u8 {
    let mut r = 0;
    for _ in 0..n {
        let v: u8 = rng.gen();
        r += v;
    }
    r
}

fn gen_benchmark(c: &mut Criterion) {
    c.bench_function("small rng performance", |b| {
        let mut rng = rand::rngs::SmallRng::from_entropy();
        b.iter(|| {
            gen_bench_fn(&mut rng, black_box(128 * 1024 * 1024));
        });
    });
}

criterion_group!(benches, gen_benchmark);
criterion_main!(benches);
