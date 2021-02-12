use serde::Deserialize;

/// CPU/内存/磁盘 基准测试结果上报接口返回的数据
#[derive(Debug, Deserialize)]
pub struct ReportResp {
    /// 错误码
    pub errno: isize,
    /// 错误信息
    pub errmsg: String,
}

impl ReportResp {
    #[inline]
    pub fn success(&self) -> bool {
        self.errno == 0
    }
}
