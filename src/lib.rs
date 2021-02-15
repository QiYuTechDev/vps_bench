use structopt::StructOpt;

mod cpu;
mod disk;
mod network;
mod ping;
mod quick;
mod ram;
mod report;
mod shared;

pub use cpu::{CPUBench, CPUCli};
pub use disk::{DiskBench, DiskCli};
pub use ping::PingCli;
pub use quick::{QuickBench, QuickCli};
pub use ram::{RAMCli, RamBench};

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
    /// 测试 APP KEY 是否有效
    Ping(PingCli),
}

impl BenchCli {
    pub fn run(&self) {
        match self {
            BenchCli::CPU(cpu) => cpu.run(None, None),
            BenchCli::Disk(disk) => disk.run(None, None),
            BenchCli::RAM(ram) => ram.run(None, None),
            BenchCli::Quick(quick) => quick.run(),
            BenchCli::Ping(ping) => ping.run(),
        }
    }
}
