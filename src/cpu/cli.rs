use structopt::StructOpt;

use super::CPUBench;
use crate::quick::QuickCli;
use crate::report::{BenchReport, CpuForm};

#[derive(Debug, StructOpt)]
#[structopt(name = "ram")]
/// 测试 CPU 的性能{n}
///
/// 测试结果使用相对值进行比较,绝对值并没有什么实际意义。
///
/// 当前仅仅支持 sqrt 性能测试{n}
/// 在相同的参数下,使用的时间越少越好
pub struct CPUCli {
    /// 实际计算的次数是 n * 1_000_000_000
    #[structopt(short, default_value = "1")]
    pub n: u64,

    /// 多少轮测试
    #[structopt(long, default_value = "16")]
    pub round: usize,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl CPUCli {
    pub fn run(&self, job_id: Option<String>, reporter: Option<BenchReport>) {
        let cpu = CPUBench::new(self.n);

        let result: Vec<_> = (0..self.round)
            .map(|idx| {
                println!("第 {} 轮的 CPU 测试开始...", idx + 1);
                let (use_time, _) = cpu.run();
                println!("第 {} 轮的 CPU 测试结束, 使用时间: {:?}", idx + 1, use_time);
                use_time
            })
            .collect();

        if let Some(reporter) = self.shared.get_reporter(reporter) {
            let times: Vec<_> = result.iter().map(|d| d.as_secs_f64()).collect();
            let form = CpuForm::new(job_id, self.n, times);
            println!("开始上报 CPU 基准测试结果 ...");
            reporter.cpu_report(self.shared.out_dir.as_deref(), &form);
            println!("上报 CPU 基准测试结果 已完成")
        }
    }
}

impl From<&QuickCli> for CPUCli {
    fn from(q: &QuickCli) -> Self {
        CPUCli {
            n: q.cpu_n,
            round: q.cpu_round,
            shared: q.shared.clone(),
        }
    }
}
