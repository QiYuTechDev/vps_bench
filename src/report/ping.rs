use serde::Deserialize;

/// `ping` 接口返回的数据
#[derive(Debug, Deserialize)]
pub struct PingResp {
    pub username: String,
}
