use structopt::StructOpt;

pub use super::SQLiteBench;
use crate::quick::QuickCli;
use crate::report::BenchReport;
use crate::sqlite_db::SQLiteForm;

#[derive(Debug, StructOpt)]
#[structopt(name = "sqlite")]
/// SQLite 事务性能测试
pub struct SQLiteCli {
    /// SQLite 测试使用的文件{n}
    /// 注意: 如果这个文件存在则会被删除
    pub file: String,

    /// 测试数据的数量 * 1_000_000
    #[structopt(short, default_value = "1")]
    pub n: usize,

    #[structopt(flatten)]
    pub shared: crate::shared::SharedCli,
}

impl SQLiteCli {
    /// 运行 RAM 测试
    pub async fn run<'a>(&self, job_id: Option<String>, reporter: Option<BenchReport<'a>>) {
        let db = SQLiteBench::new(self.file.as_str(), self.n).await;

        let result = db.run().await;

        if let Some(reporter) = self.shared.get_reporter(reporter) {
            let form = SQLiteForm {
                job_id,
                n: self.n,
                result,
            };

            reporter.sqlite_report(&form)
        }
    }
}

impl From<&QuickCli> for SQLiteCli {
    fn from(q: &QuickCli) -> Self {
        Self {
            file: q.disk_file_name.clone(),
            n: q.sqlite_n,
            shared: q.shared.clone(),
        }
    }
}
