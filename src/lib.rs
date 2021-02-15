use structopt::StructOpt;

mod cpu;
mod disk;
mod network;
mod quick;
mod ram;
mod report;
mod shared;

pub use cpu::{CPUBench, CPUCli};
pub use disk::{DiskBench, DiskCli};
pub use quick::{QuickBench, QuickCli};
pub use ram::{RAMBench, RAMCli};

#[derive(Debug, StructOpt)]
pub enum BenchCli {
    /// CPU测试
    CPU(CPUCli),
    /// 磁盘测试
    Disk(DiskCli),
    /// 内存测试
    RAM(RAMCli),
    /// VPS 性能 快速测试, 当前会测试: CPU/内存/磁盘
    Quick(QuickCli),
}

impl BenchCli {
    pub fn run(&self) {
        match self {
            BenchCli::CPU(cpu) => cpu.run(None, None),
            BenchCli::Disk(disk) => disk.run(None, None),
            BenchCli::RAM(ram) => ram.run(None, None),
            BenchCli::Quick(quick) => quick.run(),
        }
    }
}
