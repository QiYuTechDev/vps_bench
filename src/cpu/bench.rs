use std::time::{Duration, Instant};

use rand::{Rng, SeedableRng};

/// CPU 压力测试
pub struct CPUBench {
    /// compute `n` times
    pub n: u64,
}

impl CPUBench {
    /// `n` 计算多少 十亿 循环
    pub fn new(n: u64) -> Self {
        CPUBench {
            n: n * 1_000_000_000,
        }
    }

    /// 运行 CPU 测试
    /// inline never do the trick for rust *not opt the function(sqrt) away*
    #[inline(never)]
    pub fn run(&self) -> (Duration, f32) {
        let gen_time = crate::shared::gen_n_f32_random_time(self.n);

        let mut rng = rand::rngs::SmallRng::from_entropy();
        let start_time = Instant::now();

        let mut r = 0f32;
        for _ in 0..self.n {
            let v: f32 = rng.gen();
            r += v.sqrt();
        }

        (Instant::now() - start_time - gen_time, r)
    }
}

impl Default for CPUBench {
    fn default() -> Self {
        CPUBench::new(1)
    }
}

#[test]
fn cpu_bench_test() {
    let n = CPUBench::default();
    let (use_time, _) = n.run();
    println!("use time: {:?}", use_time);
}
