use structopt::StructOpt;

use crate::report::BenchReport;
use crate::shared::SharedCli;

#[derive(Debug, StructOpt)]
/// 连接服务器测试 APP_KEY 是否有效
pub struct PingCli {
    #[structopt(flatten)]
    pub shared: SharedCli,
}

impl PingCli {
    /// 运行 Ping 命令
    pub fn run(&self) {
        println!("测试服务器地址: {}", self.shared.server_url);
        if self.shared.app_key.is_empty() {
            eprintln!("没有指定 APP KEY");
            std::process::exit(1)
        }
        let _ = BenchReport::new(&self.shared, true);
    }
}

#[derive(Debug, StructOpt)]
/// 使用 APP_KEY 获取一个新的任务
pub struct JobCli {
    #[structopt(flatten)]
    pub shared: SharedCli,
}

impl JobCli {
    /// 获取新的任务
    pub fn run(&self) {
        let reporter = self.shared.get_reporter(None).expect("--app-key 没有设置");
        let job_id = reporter.new_job();
        println!("{}", job_id);
    }
}
