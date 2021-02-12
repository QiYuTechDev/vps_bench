use serde::Serialize;

/// 磁盘 上报接口请求的数据
#[derive(Debug, Serialize)]
pub struct DiskForm {
    pub job_id: Option<String>,
    pub n: usize,
    pub times: Vec<f64>,
}
