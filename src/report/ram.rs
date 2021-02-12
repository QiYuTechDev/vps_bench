use serde::Serialize;

use crate::ram::RAMResult;

/// 内存 上报接口请求的数据
#[derive(Debug, Serialize)]
pub struct RamForm {
    pub job_id: Option<String>,
    pub mem: u8,
    pub results: Vec<RAMResult>,
}
