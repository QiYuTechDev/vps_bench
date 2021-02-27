use serde::Serialize;

use crate::shared::IOTime;

/// 磁盘性能测试结果
#[derive(Debug, Default, Serialize)]
pub struct DiskResult {
    /// 文件大小
    pub file_size: u64,
    /// 记录块大小
    pub block_size: usize,
    /// 顺序 读/写 测试结果
    pub seq: IOTime,
    /// 顺序 重 读/写 测试结果
    pub seq_re: IOTime,
    /// 随机 读/写 测试结果
    pub rand: IOTime,
    /// 跳 读/写 测试结果
    pub stride: IOTime,
    /// 倒 读/写 测试结果
    pub reverse: IOTime,
}

impl DiskResult {
    #[inline]
    pub fn new(file_size: u64, block_size: usize) -> Self {
        Self {
            file_size,
            block_size,
            seq: IOTime::default(),
            seq_re: IOTime::default(),
            rand: IOTime::default(),
            stride: IOTime::default(),
            reverse: IOTime::default(),
        }
    }
}

impl ToString for DiskResult {
    fn to_string(&self) -> String {
        format!(
            r#"文件大小: {} 记录块大小: {}
顺序读写: {}
顺序重读写: {}
随机读写: {}
跳跃读写: {}
倒序读写: {}"#,
            self.file_size,
            self.block_size,
            self.seq.to_string(),
            self.seq_re.to_string(),
            self.rand.to_string(),
            self.stride.to_string(),
            self.reverse.to_string()
        )
    }
}
