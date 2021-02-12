use structopt::StructOpt;

pub use super::RAMBench;

#[derive(Debug, StructOpt)]
#[structopt(name = "ram")]
/// 内存性能测试
///
/// `mem` 指定内存大小 {n}
/// 注意: 实际使用的内存大小为: 2^mem * 128MB {n}
/// 0 表示使用  128MB {n}
/// 1 表示使用  256MB {n}
/// 2 表示使用  512MB {n}
/// 3 表示使用 1024MB=1GB {n}
/// ... {n}
pub struct RAMCli {
    /// 指定内存的大小
    #[structopt(long, default_value = "0")]
    pub mem: u8,

    /// 多少轮测试
    #[structopt(long, default_value = "1")]
    pub round: usize,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl RAMCli {
    /// 运行 RAM 测试
    pub fn run(&self) {
        let mut ram = RAMBench::new(2usize.pow(self.mem as u32));
        let bench_result: Vec<_> = (0..self.round).map(|_| ram.run_bench()).collect();
        println!("bench result: {:?}", bench_result);
    }
}
