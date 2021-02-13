use structopt::StructOpt;

use super::DiskBench;

#[derive(Debug, StructOpt)]
#[structopt(name = "ram")]
/// 测试 磁盘 的 读/写 性能
///
pub struct DiskCli {
    /// 实际测试使用的文件大小为 2^n * 1GB
    #[structopt(long, default_value = "0")]
    pub n: usize,

    /// 多少轮测试
    #[structopt(long, default_value = "1")]
    pub round: usize,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl DiskCli {
    /// 运行磁盘性能测试
    pub fn run(&self) {}
}
