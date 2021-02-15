use serde::Serialize;

/// `cpu` 上报接口请求的数据
#[derive(Debug, Serialize)]
pub struct CpuForm {
    pub job_id: Option<String>,
    pub n: usize,
    pub times: Vec<f64>,
}

impl CpuForm {
    #[inline]
    pub fn new(job_id: Option<String>, n: usize, times: Vec<f64>) -> Self {
        Self { job_id, n, times }
    }
}
