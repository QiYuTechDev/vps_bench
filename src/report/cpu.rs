use serde::Serialize;

/// `cpu` 上报接口请求的数据
#[derive(Debug, Serialize)]
pub struct CpuForm {
    pub job_id: Option<String>,
    pub n: usize,
    pub times: Vec<f64>,
}
