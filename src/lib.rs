use structopt::StructOpt;

mod cpu;
mod disk;
mod network;
mod ram;
mod shared;

pub use cpu::{CPUBench, CPUCli};
pub use disk::{DiskBench, DiskCli};
pub use ram::{RAMBench, RAMCli};

#[derive(Debug, StructOpt)]
pub enum BenchCli {
    /// CPU测试
    CPU(CPUCli),
    /// 磁盘测试
    Disk(DiskCli),
    /// 内存测试
    RAM(RAMCli),
}

impl BenchCli {
    pub fn run(&self) {
        match self {
            BenchCli::CPU(cpu) => cpu.run(),
            BenchCli::Disk(disk) => disk.run(),
            BenchCli::RAM(ram) => ram.run(),
        }
    }
}
