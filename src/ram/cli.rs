use structopt::StructOpt;

pub use super::RamBench;
use crate::quick::QuickCli;
use crate::report::BenchReport;
use crate::report::RamForm;

#[derive(Debug, StructOpt)]
#[structopt(name = "ram")]
/// 内存性能测试
///
/// 测试结果使用相对值进行比较,绝对值并没有什么实际意义。
///
/// `mem` 指定内存大小 {n}
/// 注意: 实际使用的内存大小为: 2^mem * 128MB {n}
/// 0 表示使用  128MB {n}
/// 1 表示使用  256MB {n}
/// 2 表示使用  512MB {n}
/// 3 表示使用 1024MB=1GB {n}
/// ... {n}
///
/// 在相同的参数下,使用的时间越少越好
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
    pub fn run(&self, job_id: Option<String>, reporter: Option<BenchReport>) {
        let mut ram = RamBench::new(2usize.pow(self.mem as u32));
        let results: Vec<_> = (0..self.round)
            .map(|idx| {
                println!("第 {} 轮的 内存 测试开始...", idx);
                let result = ram.run_bench();
                println!("第 {} 轮的 内存 测试结束\n{}\n", idx, result.to_string());
                result
            })
            .collect();

        if let Some(reporter) = self.shared.get_reporter(reporter) {
            let form = RamForm::new(job_id, self.mem, results);
            println!("开始上报 内存 基准测试结果 ...");
            reporter.ram_report(&form);
            println!("上报 内存 基准测试结果 已完成")
        }
    }
}

impl From<&QuickCli> for RAMCli {
    fn from(q: &QuickCli) -> Self {
        Self {
            mem: q.mem_size,
            round: q.mem_round,
            shared: q.shared.clone(),
        }
    }
}
