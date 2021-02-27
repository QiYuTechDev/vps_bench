use serde::Serialize;

/// `cpu` 上报接口请求的数据
#[derive(Debug, Serialize)]
pub struct CpuForm {
    pub job_id: Option<String>,
    pub n: u64,
    pub times: Vec<f64>,
}

impl CpuForm {
    #[inline]
    pub fn new(job_id: Option<String>, n: u64, times: Vec<f64>) -> Self {
        Self { job_id, n, times }
    }
}
