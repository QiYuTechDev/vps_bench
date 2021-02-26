use structopt::StructOpt;

use crate::quick::QuickCli;
use crate::report::{BenchReport, DiskForm};

use super::DiskBench;

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

    /// 实际测试使用的文件最大为 2^n * 4GB
    #[structopt(long, default_value = "0")]
    pub n: u8,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl DiskCli {
    /// 运行磁盘性能测试
    pub fn run(&self, job_id: Option<String>, reporter: Option<BenchReport>) {
        let mut result = Vec::new();

        for file_exp in (0..1).rev() {
            let file_size = 2usize.pow(self.n as u32 + 32 - file_exp);
            // 对于 128MB 以下的文件不进行测试
            if file_size < 128 * 1024 * 1024 {
                continue;
            }

            let block_size = 4096; // 4KB

            println!(
                "磁盘测试开始, 文件大小: {}, 记录块大小: {}",
                file_size, block_size
            );
            let disk = DiskBench::new(self.file_name.clone(), file_size, block_size);
            let ret = disk.run_bench();
            println!("磁盘测试结束:\n{}\n", ret.to_string());
            result.push(ret);
        }

        // report disk bench result
        if let Some(reporter) = self.shared.get_reporter(reporter) {
            let form = DiskForm::new(job_id, result);
            println!("开始上报磁盘测试结果...");
            reporter.disk_report(self.shared.out_dir.as_deref(), &form);
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
