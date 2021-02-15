use structopt::StructOpt;

use super::DiskBench;
use crate::quick::QuickCli;
use crate::report::{BenchReport, DiskForm};

#[derive(Debug, StructOpt)]
#[structopt(name = "ram")]
/// 测试 磁盘 的 读/写 性能
///
/// 注意: 文件使用的大小为 2 ^ n * 1GB{n}
/// 默认是: 1GB
///
/// 在相同的参数下,使用的时间越少越好
pub struct DiskCli {
    #[structopt(long)]
    /// 测试使用的文件名称{n}
    /// 注意: 如果这个文件存在则会被删除
    pub file_name: String,

    /// 实际测试使用的文件最大为 2^n * 1GB{n}
    /// 然后取:{n}
    /// size / 2 ^  0 GB{n}
    /// size / 2 ^  1 GB{n}
    /// size / 2 ^  2 GB{n}
    /// size / 2 ^  3 GB{n}
    /// size / 2 ^  4 GB{n}
    /// size / 2 ^  5 GB{n}
    /// size / 2 ^  6 GB{n}
    /// size / 2 ^  7 GB{n}
    /// size / 2 ^  8 GB{n}
    /// size / 2 ^  9 GB{n}
    /// size / 2 ^ 10 GB{n}
    /// size / 2 ^ 11 GB{n}
    /// 进行测试
    #[structopt(long, default_value = "0")]
    pub n: u8,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl DiskCli {
    /// 运行磁盘性能测试
    pub fn run(&self, job_id: Option<String>, reporter: Option<BenchReport>) {
        let mut result = Vec::new();

        for file_exp in (0..12).rev() {
            let file_size = 2usize.pow(self.n as u32 + 30 - file_exp);

            // 实际测试使用的记录块大小为
            //   2K   4K   8K  16K
            //  32K  64K 128K 256K
            //  512K  1M   2M   4M
            for block_exp in 11..23 {
                let block_size = 2usize.pow(block_exp);

                println!(
                    "磁盘测试开始, 文件大小: {}, 记录快大小: {}",
                    file_size, block_size
                );
                let disk = DiskBench::new(self.file_name.clone(), file_size, block_size);
                let ret = disk.run_bench();
                println!("磁盘测试结束:\n{}\n", ret.to_string());
                result.push(ret);
            }
        }

        // report disk bench result
        if let Some(reporter) = self.shared.get_reporter(reporter) {
            let form = DiskForm::new(job_id, result);
            println!("开始上报磁盘测试结果...");
            reporter.disk_report(&form);
            println!("上报磁盘测试结果成功。");
        }
    }
}

impl From<&QuickCli> for DiskCli {
    fn from(q: &QuickCli) -> Self {
        Self {
            file_name: q.disk_file_name.clone(),
            n: q.disk_n,
            shared: q.shared.clone(),
        }
    }
}
