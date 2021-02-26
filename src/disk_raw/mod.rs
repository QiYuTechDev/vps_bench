//! 磁盘 性能测试 (RAW 模式)
//!
//! 启用了 O_DIRECT & O_SYNC 模式
//! 尽量避免文件系统的影响。
//!

mod bench;
mod cli;

use crate::disk::DiskResult;

pub use bench::DiskRawBench;
pub use cli::DiskRawCli;
