mod cpu;
mod disk;
mod job;
mod ping;
mod ram;
mod shared;

use reqwest::blocking::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

pub use cpu::CpuForm;
pub use disk::DiskForm;
pub use job::JobResp;
pub use ping::PingResp;
pub use ram::RamForm;
pub use shared::ReportResp;

use crate::shared::SharedCli;

/// 测试结果上报
#[derive(Clone)]
pub struct BenchReport<'a> {
    cli: &'a SharedCli,
}

impl<'a> BenchReport<'a> {
    #[inline]
    pub fn new(cli: &'a SharedCli) -> Self {
        Self::test_app_key_valid(cli);

        BenchReport { cli }
    }

    /// 创建一个新的任务
    pub fn new_job(&self) -> String {
        let url = self.get_api_url("job");
        let builder = Self::add_header(Client::new().get(url.as_str()), self.cli.app_key.as_str());
        let resp = Self::send_request::<JobResp>(builder, url.as_str());
        resp.job_id
    }

    /// 磁盘测试结果上报
    pub fn disk_report(&self, form: &DiskForm) {
        let url = self.get_api_url("disk");
        let resp = self.do_report(url.as_str(), form);
        if resp.success() {
            return;
        }
        eprintln!(
            "上报 磁盘 基准测试结果失败 错误码 {}, 错误信息: {}",
            resp.errno, resp.errmsg
        );
        std::process::exit(1);
    }

    /// CPU 测试结果上报
    pub fn cpu_report(&self, form: &CpuForm) {
        let url = self.get_api_url("cpu");
        let resp = self.do_report(url.as_str(), form);
        if resp.success() {
            return;
        }
        eprintln!(
            "上报 CPU 基准测试结果失败 错误码 {}, 错误信息: {}",
            resp.errno, resp.errmsg
        );
        std::process::exit(1);
    }

    /// 内存测试结果上报
    pub fn ram_report(&self, form: &RamForm) {
        let url = self.get_api_url("ram");
        let resp = self.do_report(url.as_str(), form);
        if resp.success() {
            return;
        }
        eprintln!(
            "上报 内存 基准测试结果失败 错误码 {}, 错误信息: {}",
            resp.errno, resp.errmsg
        );
        std::process::exit(1);
    }

    /// 上报 `form` 数据到 `url`
    fn do_report<T: serde::Serialize>(&self, url: &str, form: &T) -> ReportResp {
        let builder = self.build_post_request(url, form);
        Self::send_request::<ReportResp>(builder, url)
    }

    /// 测试 app key 在服务器上是否有效
    /// app key 无效则直接退出
    fn test_app_key_valid(cli: &SharedCli) {
        let url = Self::format_url(cli.server_url.as_str(), "ping");
        let builder = Self::add_header(Client::new().get(url.as_str()), cli.app_key.as_str());
        let msg = Self::send_request::<PingResp>(builder, url.as_str());
        println!("当前使用的用户是: {}", msg.username);
    }

    /// 发送请求
    fn send_request<T: DeserializeOwned>(builder: RequestBuilder, url: &str) -> T {
        let resp = match builder.send() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("请求 URL {} 失败: {}", url, e.to_string());
                std::process::exit(1);
            }
        };

        if !resp.status().is_success() {
            eprintln!("请求 URL {} 失败: {:?}", url, resp);
            std::process::exit(1);
        }

        match resp.json() {
            Ok(resp) => return resp,
            Err(e) => {
                eprintln!("解析 JSON 失败 失败: {:?}", e);
                std::process::exit(1);
            }
        }
    }

    /// 构造 POST 请求
    fn build_post_request<F: serde::Serialize>(&self, url: &str, form: &F) -> RequestBuilder {
        Self::add_header(
            Client::new()
                .post(url)
                .body(serde_json::to_string(form).unwrap()),
            self.cli.app_key.as_str(),
        )
    }

    /// 给所有的请求加上 `Authorization` 以及 `Accept` 头信息
    #[inline]
    fn add_header(builder: RequestBuilder, app_key: &str) -> RequestBuilder {
        builder
            .header("Authorization", format!("Bearer {}", app_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
    }

    #[inline]
    fn get_api_url(&self, api_name: &str) -> String {
        Self::format_url(self.cli.server_url.as_str(), api_name)
    }

    /// 获取 要访问的 URL
    #[inline]
    fn format_url(server_uri: &str, api_name: &str) -> String {
        format!("{}{}", server_uri, api_name)
    }
}
