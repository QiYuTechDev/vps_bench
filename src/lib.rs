use structopt::StructOpt;

mod cpu;
mod disk;
mod network;
mod ram;
mod shared;

pub use cpu::{CPUBench, CPUCli};
pub use ram::{RAMBench, RAMCli};

#[derive(Debug, StructOpt)]
pub enum BenchCli {
    /// 内存测试
    RAM(RAMCli),
    /// CPU测试
    CPU(CPUCli),
}

impl BenchCli {
    pub fn run(&self) {
        match self {
            BenchCli::RAM(ram) => ram.run(),
            BenchCli::CPU(cpu) => cpu.run(),
        }
    }
}
