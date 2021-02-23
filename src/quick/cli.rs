use structopt::StructOpt;

pub use super::QuickBench;

#[derive(Debug, StructOpt)]
/// VPS 性能快速测试
pub struct QuickCli {
    /// CPU 性能测试{n}
    /// 实际计算的次数是 2^n * 1_000_000
    #[structopt(long, default_value = "0")]
    pub cpu_n: usize,
    /// CPU 进行多少轮测试
    #[structopt(long, default_value = "16")]
    pub cpu_round: usize,

    /// 内存测试指定内存的大小
    #[structopt(long, default_value = "0")]
    pub mem_size: u8,

    /// 内存进行多少轮测试
    #[structopt(long, default_value = "16")]
    pub mem_round: usize,

    /// 磁盘/SQLite 测试使用的文件名称{n}
    /// 注意: 如果这个文件存在则会被删除
    #[structopt(long)]
    pub disk_file_name: String,

    /// 磁盘测试实际测试使用的文件大小为 2^n * 1GB
    #[structopt(long, default_value = "0")]
    pub disk_n: u8,

    #[structopt(long, default_value = "1")]
    /// SQLite 测试多少使用多少 百万行数据
    pub sqlite_n: usize,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl QuickCli {
    /// 运行
    pub fn run(&self) {
        let bench = QuickBench::new(self);
        bench.run_bench();
    }
}
