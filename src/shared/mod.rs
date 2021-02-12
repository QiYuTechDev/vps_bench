use std::time;

use rand::{Rng, SeedableRng};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SharedCli {
    /// 运行结果上报
    #[structopt(long, env = "VPS_BENCH_REPORT")]
    pub report: bool,
    /// 遥测信息不上报
    #[structopt(long, env = "VPS_BENCH_NO_TELEMETRY", hidden = true)]
    pub no_telemetry: bool,
}

#[inline(never)]
/// 生成 `n` 个 `f32` 类型随机数的时间
pub fn gen_n_f32_random_time(n: usize) -> time::Duration {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let start_time = time::Instant::now();
    for _ in 0..n {
        let _: f32 = rng.gen();
    }
    time::Instant::now() - start_time
}

#[inline(never)]
/// 生成 `n` 个 `u8` 类型随机数的时间
pub fn gen_n_u8_random_time(n: usize) -> time::Duration {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let start_time = time::Instant::now();
    for _ in 0..n {
        let _: u8 = rng.gen();
    }
    time::Instant::now() - start_time
}

#[inline(never)]
/// 生成 `n` 个 `usize` 类型随机数的时间
pub fn gen_n_usize_range_random_time(n: usize, max: usize) -> time::Duration {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let start_time = time::Instant::now();
    for _ in 0..n {
        let _: usize = rng.gen_range(0..max);
    }
    time::Instant::now() - start_time
}
