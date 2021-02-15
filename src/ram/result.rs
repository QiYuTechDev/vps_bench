use serde::Serialize;

use crate::shared::IOTime;

#[derive(Debug, Default, Serialize)]
pub struct RamResult {
    /// 内存大小
    pub mem_size: u64,
    /// 顺序 读写耗时
    pub seq: IOTime,
    /// 随机 读写耗时
    pub rand: IOTime,
}

impl RamResult {
    #[inline]
    pub fn new(mem_size: u64) -> Self {
        Self {
            mem_size,
            seq: IOTime::default(),
            rand: IOTime::default(),
        }
    }
}

impl ToString for RamResult {
    fn to_string(&self) -> String {
        format!(
            r#"内存大小: {}
顺序读写: {}
随机读写: {}
"#,
            self.mem_size,
            self.seq.to_string(),
            self.rand.to_string()
        )
    }
}
