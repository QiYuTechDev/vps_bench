use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub age: i32,
}

#[derive(Debug, FromRow)]
pub struct RowCount {
    pub total: i32,
}

#[derive(Debug, Serialize)]
pub struct SQLiteResult {
    /// 顺序插入
    pub seq_insert: f64,
    /// 顺序读取
    pub seq_read: f64,
    /// 跳读
    pub skip_read: f64,
    /// 顺序更新
    pub seq_update: f64,
    /// 顺序删除
    pub seq_delete: f64,
    /// 随机读取
    pub rand_read: f64,
    /// 随机更新
    pub rand_update: f64,
}

#[derive(Debug, Serialize)]
pub struct SQLiteForm {
    pub job_id: Option<String>,
    pub n: usize,
    pub result: SQLiteResult,
}

impl SQLiteForm {
    #[inline]
    pub fn new(job_id: Option<String>, n: usize, result: SQLiteResult) -> Self {
        Self { job_id, n, result }
    }
}
