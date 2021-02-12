use structopt::StructOpt;

use super::CPUBench;

#[derive(Debug, StructOpt)]
#[structopt(name = "ram")]
/// 测试 CPU 的性能
/// 当前仅仅支持 sqrt 性能测试
pub struct CPUCli {
    /// 实际计算的次数是 2^n * 1_000_000
    #[structopt(long, default_value = "0")]
    pub n: usize,

    /// 多少轮测试
    #[structopt(long, default_value = "1")]
    pub round: usize,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl CPUCli {
    pub fn run(&self) {
        let cpu = CPUBench::new(2usize.pow(self.n as u32));
        for _ in 0..self.round {
            let (use_time, _) = cpu.run();
            println!("use time: {:?}", use_time);
        }
    }
}
