use crate::report::BenchReport;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct SharedCli {
    /// 遥测信息不上报
    #[structopt(long, env = "BENCH_NO_TELEMETRY", hidden = true)]
    pub no_telemetry: bool,
    #[structopt(long, env = "BENCH_APP_KEY", default_value = "")]
    /// 设置 app-key 则会上报数据到服务器
    pub app_key: String,
    #[structopt(
        long,
        env = "BENCH_SERVER_URL",
        default_value = "https://vps.qiyutech.tech/api/bench/v1/",
        hidden = true
    )]
    pub server_url: String,
}

impl SharedCli {
    /// 获取基准测试报告工具
    #[inline]
    pub fn get_reporter<'a>(&'a self, other: Option<BenchReport<'a>>) -> Option<BenchReport<'a>> {
        if self.app_key.is_empty() {
            return None;
        }

        match other {
            Some(reporter) => Some(reporter),
            None => Some(BenchReport::new(self)),
        }
    }
}
