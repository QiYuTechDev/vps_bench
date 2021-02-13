use std::time::Duration;

#[derive(Debug, Default)]
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
