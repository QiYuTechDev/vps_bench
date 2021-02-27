use std::time::{Duration, Instant};

use rand::{distributions::Alphanumeric, Rng, SeedableRng};
use sqlx::{sqlite::SqliteConnectOptions, sqlite::SqliteConnection, Connection};

use super::{Record, RowCount, SQLiteResult};

/// RAM 读写速度测试
pub struct SQLiteBench {
    /// SQLite 数据库连接
    pub conn: SqliteConnection,
    /// 多少条测试数据
    pub n: usize,
}

impl SQLiteBench {
    /// 新建一个 SQLite 测试示例
    pub async fn new(file: &str, n: usize) -> Self {
        // 保证这个文件是新创建的
        let file = std::path::Path::new(file);
        if file.exists() {
            std::fs::remove_file(file).unwrap();
        }

        let option = SqliteConnectOptions::new()
            .filename(file)
            .create_if_missing(true);
        let conn = SqliteConnection::connect_with(&option).await.unwrap();
        Self {
            conn,
            n: n * 100_000,
        }
    }

    /// 开始执行 SQLite 测试任务
    pub async fn run(mut self) -> SQLiteResult {
        let table = include_str!("sql/table.sql");
        let _ = sqlx::query(table).execute(&mut self.conn).await.unwrap();

        // seq insert
        println!("SQLite3 开始添加数据 ...");
        let seq_insert = self.do_insert().await;
        println!("SQLite3 添加数据已完成 {:?}", seq_insert);
        let row = self.get_row_count().await;
        assert_eq!(row, self.n);

        // seq read
        println!("SQLite3 顺序读 ...");
        let seq_read = self.do_seq_read().await;
        println!("SQLite3 顺序读已完成 {:?}", seq_read);
        // skip read
        println!("SQLite3 跳跃读 ...");
        let skip_read = self.do_skip_read().await;
        println!("SQLite3 跳跃读已完成 {:?}", skip_read);
        // seq update
        println!("SQLite3 顺序更新 ...");
        let seq_update = self.do_seq_update().await;
        println!("SQLite3 顺序更新已完成 {:?}", seq_update);
        // seq delete
        println!("SQLite3 顺序删除...");
        let seq_delete = self.do_seq_delete().await;
        println!("SQLite3 顺序删除已完成 {:?}", seq_delete);
        let row = self.get_row_count().await;
        assert_eq!(row, 0);

        // ==================================================
        // refill the data
        // ==================================================
        println!("SQLite3 重新填充数据...");
        let _ = self.do_insert().await;
        println!("SQLite3 重新填充数据已完成");
        let row = self.get_row_count().await;
        assert_eq!(row, self.n);

        // rand read
        println!("SQLite3 开始随机读取 ...");
        let rand_read = self.do_rand_read().await;
        println!("SQLite3 随机读取已完成 {:?}", rand_read);
        // rand update
        println!("SQLite3 开始随机更新 ...");
        let rand_update = self.do_rand_update().await;
        println!("SQLite3 随机更新已完成 {:?}", rand_update);

        SQLiteResult {
            seq_insert: seq_insert.as_secs_f64(),
            seq_read: seq_read.as_secs_f64(),
            skip_read: skip_read.as_secs_f64(),
            seq_update: seq_update.as_secs_f64(),
            seq_delete: seq_delete.as_secs_f64(),
            rand_read: rand_read.as_secs_f64(),
            rand_update: rand_update.as_secs_f64(),
        }
    }

    pub async fn do_skip_read(&mut self) -> Duration {
        let read = include_str!("sql/read.sql");

        let start_time = Instant::now();

        for offset in 1..1001 {
            for idx in 0..(self.n / 1000) {
                let _: Record = sqlx::query_as(read)
                    .bind((idx * 1000 + offset) as i32)
                    .fetch_one(&mut self.conn)
                    .await
                    .unwrap();
            }
        }

        Instant::now() - start_time
    }

    async fn get_row_count(&mut self) -> usize {
        let read = include_str!("sql/count.sql");

        let d: RowCount = sqlx::query_as(read)
            .fetch_one(&mut self.conn)
            .await
            .unwrap();

        d.total as usize
    }

    async fn get_min_and_max_id(&mut self) -> (i32, i32) {
        let read = include_str!("sql/id_range.sql");

        sqlx::query_as::<_, (i32, i32)>(read)
            .fetch_one(&mut self.conn)
            .await
            .unwrap()
    }

    pub async fn do_rand_read(&mut self) -> Duration {
        let read = include_str!("sql/read.sql");

        let mut rng = rand::rngs::SmallRng::from_entropy();

        let (min_id, max_id) = self.get_min_and_max_id().await;

        let start_time = Instant::now();

        for _ in 0..self.n {
            let id: i32 = rng.gen_range(min_id..max_id);

            let _: Record = sqlx::query_as(read)
                .bind(id)
                .fetch_one(&mut self.conn)
                .await
                .unwrap();
        }

        Instant::now() - start_time
    }

    pub async fn do_seq_read(&mut self) -> Duration {
        let read = include_str!("sql/read.sql");

        let start_time = Instant::now();

        for idx in 1..(self.n + 1) {
            let _ = sqlx::query_as::<_, Record>(read)
                .bind(idx as i32)
                .fetch_one(&mut self.conn)
                .await
                .unwrap();
        }

        Instant::now() - start_time
    }

    pub async fn do_seq_delete(&mut self) -> Duration {
        let delete = include_str!("sql/delete.sql");

        let start_time = Instant::now();

        for idx in 1..(self.n + 1) {
            sqlx::query(delete)
                .bind(idx as i32)
                .execute(&mut self.conn)
                .await
                .unwrap();
        }

        Instant::now() - start_time
    }

    pub async fn do_rand_update(&mut self) -> Duration {
        let update = include_str!("sql/update.sql");

        let mut gen_s = {
            let rng = rand::rngs::SmallRng::from_entropy();

            let mut sample = rng.sample_iter(Alphanumeric);

            move |n: usize| -> String {
                let o = &mut sample;
                let v = o.take(n).collect::<Vec<_>>();
                String::from_utf8(v).unwrap()
            }
        };

        let mut gen_age = {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            move || -> i32 { rng.gen() }
        };

        let mut rng = rand::rngs::SmallRng::from_entropy();

        let (min_id, max_id) = self.get_min_and_max_id().await;

        let start_time = Instant::now();

        for _ in 0..(self.n) {
            let idx: i32 = rng.gen_range(min_id..max_id);
            let name = gen_s(32);
            let value = gen_s(20 * 1024);
            let age = gen_age();

            sqlx::query(update)
                .bind(name)
                .bind(value)
                .bind(age)
                .bind(idx as i32)
                .execute(&mut self.conn)
                .await
                .unwrap();
        }

        Instant::now() - start_time
    }

    pub async fn do_seq_update(&mut self) -> Duration {
        let update = include_str!("sql/update.sql");

        let mut gen_s = {
            let rng = rand::rngs::SmallRng::from_entropy();

            let mut sample = rng.sample_iter(Alphanumeric);

            move |n: usize| -> String {
                let o = &mut sample;
                let v = o.take(n).collect::<Vec<_>>();
                String::from_utf8(v).unwrap()
            }
        };

        let mut gen_age = {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            move || -> i32 { rng.gen() }
        };

        let start_time = Instant::now();

        for idx in 1..(self.n + 1) {
            let name = gen_s(32);
            let value = gen_s(20 * 1024);
            let age = gen_age();

            sqlx::query(update)
                .bind(name)
                .bind(value)
                .bind(age)
                .bind(idx as i32)
                .execute(&mut self.conn)
                .await
                .unwrap();
        }

        Instant::now() - start_time
    }

    /// 插入数据测试
    pub async fn do_insert(&mut self) -> Duration {
        let insert = include_str!("sql/insert.sql");

        let mut gen_s = {
            let rng = rand::rngs::SmallRng::from_entropy();

            let mut sample = rng.sample_iter(Alphanumeric);

            move |n: usize| -> String {
                let o = &mut sample;
                let v = o.take(n).collect::<Vec<_>>();
                String::from_utf8(v).unwrap()
            }
        };

        let mut gen_age = {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            move || -> i32 { rng.gen() }
        };

        let start_time = Instant::now();

        for _ in 0..(self.n) {
            let name = gen_s(32);
            let value = gen_s(20 * 1024);
            let age = gen_age();

            sqlx::query(insert)
                .bind(name)
                .bind(value)
                .bind(age)
                .execute(&mut self.conn)
                .await
                .unwrap();
        }

        Instant::now() - start_time
    }
}
