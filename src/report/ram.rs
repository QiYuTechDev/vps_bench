use serde::Serialize;

use crate::ram::RamResult;

/// 内存 上报接口请求的数据
#[derive(Debug, Serialize)]
pub struct RamForm {
    pub job_id: Option<String>,
    pub mem: usize,
    pub results: Vec<RamResult>,
}

impl RamForm {
    #[inline]
    pub fn new(job_id: Option<String>, mem: usize, results: Vec<RamResult>) -> Self {
        Self {
            job_id,
            mem,
            results,
        }
    }
}
