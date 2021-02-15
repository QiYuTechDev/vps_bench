use super::QuickCli;

use crate::{CPUCli, DiskCli, RAMCli};

pub struct QuickBench<'a> {
    quick_cli: &'a QuickCli,
}

impl<'a> QuickBench<'a> {
    #[inline]
    pub fn new(quick_cli: &'a QuickCli) -> Self {
        Self { quick_cli }
    }

    /// 运行快速测试
    pub fn run_bench(&'a self) {
        let reporter = self.quick_cli.shared.get_reporter(None);
        let job_id = if let Some(ref r) = reporter {
            Some(r.new_job())
        } else {
            None
        };

        let disk: DiskCli = self.quick_cli.into();
        disk.run(job_id.clone(), reporter.clone());

        let ram: RAMCli = self.quick_cli.into();
        ram.run(job_id.clone(), reporter.clone());

        let cpu: CPUCli = self.quick_cli.into();
        cpu.run(job_id.clone(), reporter.clone());
    }
}
