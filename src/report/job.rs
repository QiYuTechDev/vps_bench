use serde::Deserialize;

/// `job` 接口返回的数据
#[derive(Debug, Deserialize)]
pub struct JobResp {
    pub job_id: String,
}
