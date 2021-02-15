use std::time::Duration;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
/// IO 时间
pub struct IOTime {
    /// 写时间, 单位: s
    pub write: f64,
    /// 读时间, 单位: s
    pub read: f64,
}

impl IOTime {
    #[inline]
    pub fn new(read: Duration, write: Duration) -> Self {
        Self {
            read: read.as_secs_f64(),
            write: write.as_secs_f64(),
        }
    }
}

impl ToString for IOTime {
    fn to_string(&self) -> String {
        format!("读取: {} 写入: {}", self.read, self.write)
    }
}
