use serde::Serialize;

use crate::disk::DiskResult;

/// 磁盘 上报接口请求的数据
#[derive(Debug, Serialize)]
pub struct DiskForm {
    /// 任务 ID
    pub job_id: Option<String>,
    /// 文件大小指数
    pub n: u8,
    /// 结果数据
    pub results: Vec<DiskResult>,
}

impl DiskForm {
    #[inline]
    pub fn new(job_id: Option<String>, n: u8, results: Vec<DiskResult>) -> Self {
        Self { job_id, n, results }
    }
}
