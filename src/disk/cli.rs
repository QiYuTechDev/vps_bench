use structopt::StructOpt;

use super::DiskBench;

#[derive(Debug, StructOpt)]
#[structopt(name = "ram")]
/// 测试 磁盘 的 读/写 性能
///
/// 注意: 文件使用的大小为 1GB 的 2 ^ n 大小 {n}
///      块大小为 512 字节的 2 ^ block 大小
pub struct DiskCli {
    #[structopt(long)]
    /// 测试使用的文件名称{n}
    /// 注意: 如果这个文件存在则会被删除
    pub file_name: String,

    /// 实际测试使用的文件大小为 2^n * 1GB
    #[structopt(long, default_value = "0")]
    pub n: u32,

    #[structopt(long, default_value = "0")]
    /// 实际测试使用的文件大小为 2^n * 512 B
    pub block: u32,

    /// 多少轮测试
    #[structopt(long, default_value = "1")]
    pub round: usize,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl DiskCli {
    /// 运行磁盘性能测试
    pub fn run(&self) {
        let file_size = 2usize.pow(self.n) * 1024 * 1024 * 1024;
        let block_size = 2usize.pow(self.block) * 512;
        println!(
            "disk bench use file size: {}, block size: {}",
            file_size, block_size
        );
        let disk = DiskBench::new(self.file_name.clone(), file_size, block_size);
        let result = disk.run_bench();
        println!("{:?}", result);
    }
}
