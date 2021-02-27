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

    /// 实际测试使用的文件最大为 n * 1GB
    #[structopt(short, default_value = "1")]
    pub n: u64,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl DiskCli {
    /// 运行磁盘性能测试
    pub fn run(&self, job_id: Option<String>, reporter: Option<BenchReport>) {
        let mut result = Vec::new();

        assert!(self.n > 0);

        let disk = DiskBench::new(self.file_name.clone(), self.n, 4096);
        let ret = disk.run_bench();

        result.push(ret);

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
