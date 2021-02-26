use structopt::StructOpt;

mod cpu;
mod disk;
#[cfg(target_os = "linux")]
mod disk_raw;
mod ping;
mod quick;
mod ram;
mod report;
mod shared;
mod sqlite_db;

pub use cpu::{CPUBench, CPUCli};
pub use disk::{DiskBench, DiskCli};
#[cfg(target_os = "linux")]
pub use disk_raw::DiskRawCli;
pub use ping::{JobCli, PingCli};
pub use quick::{QuickBench, QuickCli};
pub use ram::{RAMCli, RamBench};
pub use sqlite_db::SQLiteCli;

#[derive(Debug, StructOpt)]
pub enum BenchCli {
    /// CPU测试
    CPU(CPUCli),
    /// 磁盘测试
    Disk(DiskCli),
    #[cfg(target_os = "linux")]
    DiskRaw(DiskRawCli),
    /// 内存测试
    RAM(RAMCli),
    /// SQLite 事务测试
    #[structopt(name = "sqlite")]
    SQLite(SQLiteCli),
    /// VPS 性能 快速测试, 当前会测试: CPU/内存/磁盘
    Quick(QuickCli),
    /// 测试 APP KEY 是否有效
    Ping(PingCli),
    /// 获取新的任务
    Job(JobCli),
}

impl BenchCli {
    pub fn run(self) {
        match self {
            BenchCli::CPU(cpu) => cpu.run(None, None),
            BenchCli::Disk(disk) => disk.run(None, None),
            #[cfg(target_os = "linux")]
            BenchCli::DiskRaw(raw) => raw.run(None, None),
            BenchCli::RAM(ram) => ram.run(None, None),
            BenchCli::SQLite(db) => futures::executor::block_on(async move {
                db.run(None, None).await;
            }),
            BenchCli::Quick(quick) => quick.run(),
            BenchCli::Ping(ping) => ping.run(),
            BenchCli::Job(job) => job.run(),
        }
    }
}
