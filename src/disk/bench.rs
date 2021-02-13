use std::io::{Read, Seek, SeekFrom, Write};
use std::time;

use rand::{Rng, SeedableRng};

/// 读写测试结果
#[derive(Default)]
pub struct IOResult {
    /// 读 耗时 单位: 秒
    read: f64,
    /// 写 耗时 单位: 秒
    write: f64,
}

/// 磁盘性能测试结果
pub struct DiskResult {
    /// 文件大小
    file_size: usize,
    /// 记录块大小
    block_size: usize,
    /// 顺序 读/写 测试结果
    seq: IOResult,
    /// 顺序 重 读/写 测试结果
    seq_re: IOResult,
    /// 随机 读/写 测试结果
    rand: IOResult,
    /// 跳 读/写 测试结果
    stride: IOResult,
    /// 倒 读/写 测试结果
    reverse: IOResult,
}

/// 磁盘性能测试
pub struct DiskBench {
    /// 测试使用的文件名称
    pub file_name: String,
    /// 测试文件大小
    pub file_size: usize,
    /// 一条记录的大小
    /// 每次 读取/写入 的固定大小
    pub block_size: usize,
}

impl DiskBench {
    /// 创建一个 新的 磁盘测试
    pub fn new(file_name: String, file_size: usize, block_size: usize) -> DiskBench {
        DiskBench {
            file_name,
            file_size,
            block_size,
        }
    }

    /// 顺序 写
    #[inline(never)]
    pub fn seq_write(&self) -> time::Duration {
        let (use_time, file) = self.do_write_file();
        drop(file);
        use_time
    }

    /// 顺序 读
    #[inline(never)]
    pub fn seq_read(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let mut data = Vec::<u8>::with_capacity(self.block_size);
        unsafe { data.set_len(self.block_size) };

        let start_time = time::Instant::now();
        for _ in 0..(self.file_size / self.block_size) {
            file.read_exact(data.as_mut_slice())
        }
        time::Instant::now() - start_time
    }

    /// 顺序 重写
    #[inline(never)]
    pub fn seq_re_write(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();
        let start_time = time::Instant::now();
        // from start re-write the file
        for _ in 0..(self.file_size / self.block_size) {
            file.write_all(block_data.as_slice());
        }
        time::Instant::now() - start_time
    }

    /// 顺序 重读
    #[inline(never)]
    pub fn seq_re_read(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let mut data = Vec::<u8>::with_capacity(self.block_size);
        unsafe { data.set_len(self.block_size) };

        // first time read
        for _ in 0..(self.file_size / self.block_size) {
            file.read_exact(data.as_mut_slice())
        }

        // re-read all data
        let start_time = time::Instant::now();
        for _ in 0..(self.file_size / self.block_size) {
            file.read_exact(data.as_mut_slice())
        }
        time::Instant::now() - start_time
    }

    /// 随机 写
    #[inline(never)]
    pub fn rand_write(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();
        // 随机写数据
    }

    /// 随机 读
    #[inline(never)]
    pub fn rand_read(&self) {}

    /// 跳 写
    #[inline(never)]
    pub fn stride_write(&self) {}

    /// 跳 读
    #[inline(never)]
    pub fn stride_read(&self) {}

    /// 倒 写
    #[inline(never)]
    pub fn reverse_write(&self) {}

    /// 倒 读
    #[inline(never)]
    pub fn reverse_read(&self) {}

    /// fill file with random data
    #[inline(never)]
    fn do_write_file(&self) -> (time::Duration, std::fs::File) {
        let _ = std::fs::remove_file(self.file_name.as_str());

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(self.file_name.as_str())
            .expect("打开文件失败");

        let block_data = Self::gen_block_size_data(self.block_size);

        let start_time = time::Instant::now();
        for _ in 0..(self.file_size / self.block_size) {
            file.write_all(block_data.as_slice());
        }
        let use_time = time::Instant::now() - start_time;
        // set offset to start
        file.seek(SeekFrom::Start(0));
        (use_time, file)
    }

    /// 生成 `block_size` 大小的随机数据
    fn gen_block_size_data(block_size: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(block_size);
        for _ in 0..block_size {
            v.push(rand::random());
        }
        assert_eq!(v.len(), block_size);
        v
    }
}
