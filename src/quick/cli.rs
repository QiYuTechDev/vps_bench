use structopt::StructOpt;

pub use super::QuickBench;

#[derive(Debug, StructOpt)]
/// VPS 性能快速测试
pub struct QuickCli {
    /// CPU 性能测试{n}
    /// 实际计算的次数是 cpu_n * 1_000_000_000
    #[structopt(long, default_value = "1")]
    pub cpu_n: u64,

    /// CPU 进行多少轮测试
    #[structopt(long, default_value = "16")]
    pub cpu_round: usize,

    /// 内存测试指定内存的大小 单位: MB
    #[structopt(long, default_value = "1")]
    pub mem_size: usize,

    /// 内存进行多少轮测试
    #[structopt(long, default_value = "16")]
    pub mem_round: usize,

    /// 磁盘/SQLite 测试使用的文件名称{n}
    /// 注意: 如果这个文件存在则会被删除
    #[structopt(long)]
    pub disk_file_name: String,

    /// 磁盘测试实际测试使用的文件大小为 n * 1GB
    #[structopt(long, default_value = "1")]
    pub disk_n: u64,

    #[structopt(long, default_value = "1")]
    /// SQLite 测试的数据量 n * 100_000
    pub sqlite_n: usize,

    #[structopt(long)]
    /// 指定任务 ID
    pub job_id: Option<String>,

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
