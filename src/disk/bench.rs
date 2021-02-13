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

impl IOResult {
    #[inline]
    pub fn new(read: time::Duration, write: time::Duration) -> Self {
        Self {
            read: read.as_secs_f64(),
            write: write.as_secs_f64(),
        }
    }
}

/// 磁盘性能测试结果
#[derive(Debug, Default)]
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

    /// 运行磁盘测试
    pub fn run_bench(&self) -> DiskResult {
        let mut result = DiskResult::default();

        result.file_size = self.file_size;
        result.block_size = self.block_size;

        result.seq = IOResult::new(self.seq_read(), self.seq_write());
        result.seq_re = IOResult::new(self.seq_re_read(), self.seq_re_write());
        result.rand = IOResult::new(self.rand_read(), self.rand_write());
        result.stride = IOResult::new(self.stride_read(), self.stride_write());
        result.reverse = IOResult::new(self.reverse_read(), self.reverse_write());

        result
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

        let mut data = Self::gen_block_size_data(self.block_size);

        let start_time = time::Instant::now();
        for _ in 0..self.blocks() {
            file.read_exact(data.as_mut_slice()).unwrap();
        }
        time::Instant::now() - start_time
    }

    /// 顺序 重写
    #[inline(never)]
    pub fn seq_re_write(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let block_data = Self::gen_block_size_data(self.block_size);

        let start_time = time::Instant::now();
        // from start re-write the file
        for _ in 0..self.blocks() {
            file.write_all(block_data.as_slice()).unwrap();
        }
        time::Instant::now() - start_time
    }

    /// 顺序 重读
    #[inline(never)]
    pub fn seq_re_read(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let mut data = Self::gen_block_size_data(self.block_size);

        // first time read
        for _ in 0..self.blocks() {
            let _ = file.read_exact(data.as_mut_slice()).unwrap();
        }

        // re-read all data
        let start_time = time::Instant::now();
        for _ in 0..self.blocks() {
            let _ = file.read_exact(data.as_mut_slice()).unwrap();
        }
        time::Instant::now() - start_time
    }

    /// 随机 写
    #[inline(never)]
    pub fn rand_write(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let mut rng = rand::rngs::SmallRng::from_entropy();
        let data = Self::gen_block_size_data(self.block_size);

        let blocks = self.blocks();

        let start_time = time::Instant::now();
        // 随机写数据
        for _ in 0..blocks {
            let v = rng.gen_range(0..blocks);
            let _ = file
                .seek(SeekFrom::Start((v * self.block_size) as u64))
                .unwrap();
            let _ = file.write_all(data.as_slice()).unwrap();
        }
        time::Instant::now() - start_time
    }

    /// 随机 读
    #[inline(never)]
    pub fn rand_read(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();
        let mut data = Self::gen_block_size_data(self.block_size);
        let mut rng = rand::rngs::SmallRng::from_entropy();

        let blocks = self.blocks();

        let start_time = time::Instant::now();
        // 随机读取数据
        for _ in 0..blocks {
            let v = rng.gen_range(0..blocks);
            let _ = file
                .seek(SeekFrom::Start((v * self.block_size) as u64))
                .unwrap();
            let _ = file.read_exact(data.as_mut_slice()).unwrap();
        }
        time::Instant::now() - start_time
    }

    /// 跳 写
    #[inline(never)]
    pub fn stride_write(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let data = Self::gen_block_size_data(self.block_size);

        let start_time = time::Instant::now();
        self.do_stride_write(&mut file, 2, data.as_slice());
        self.do_stride_write(&mut file, 4, data.as_slice());
        self.do_stride_write(&mut file, 8, data.as_slice());
        self.do_stride_write(&mut file, 16, data.as_slice());
        self.do_stride_write(&mut file, 32, data.as_slice());
        self.do_stride_write(&mut file, 64, data.as_slice());
        self.do_stride_write(&mut file, 64, data.as_slice()); // 4 * 64 = 256
        time::Instant::now() - start_time
    }

    #[inline(always)]
    pub fn do_stride_write(&self, file: &mut std::fs::File, step: usize, data: &[u8]) {
        let blocks = self.blocks();
        for idx in (0..blocks).step_by(step) {
            let _ = file
                .seek(SeekFrom::Start((idx * step * self.block_size) as u64))
                .unwrap();
            let _ = file.write_all(data).unwrap();
        }
    }

    /// 跳 读
    #[inline(never)]
    pub fn stride_read(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let mut data = Self::gen_block_size_data(self.block_size);
        let start_time = time::Instant::now();
        self.do_stride_read(&mut file, 2, data.as_mut_slice());
        self.do_stride_read(&mut file, 4, data.as_mut_slice());
        self.do_stride_read(&mut file, 8, data.as_mut_slice());
        self.do_stride_read(&mut file, 16, data.as_mut_slice());
        self.do_stride_read(&mut file, 32, data.as_mut_slice());
        self.do_stride_read(&mut file, 64, data.as_mut_slice());
        self.do_stride_read(&mut file, 64, data.as_mut_slice()); // 4 * 64 = 256
        time::Instant::now() - start_time
    }

    #[inline(always)]
    pub fn do_stride_read(&self, file: &mut std::fs::File, step: usize, data: &mut [u8]) {
        let blocks = self.blocks();
        for idx in (0..blocks).step_by(step) {
            let _ = file
                .seek(SeekFrom::Start((idx * step * self.block_size) as u64))
                .unwrap();
            let _ = file.read_exact(data).unwrap();
        }
    }

    /// 倒 写
    #[inline(never)]
    pub fn reverse_write(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let data = Self::gen_block_size_data(self.block_size);

        let start_time = time::Instant::now();
        for idx in self.blocks()..0 {
            file.seek(SeekFrom::Start((idx * self.block_size) as u64))
                .unwrap();
            file.write_all(data.as_slice()).unwrap();
        }
        time::Instant::now() - start_time
    }

    /// 倒 读
    #[inline(never)]
    pub fn reverse_read(&self) -> time::Duration {
        let (_, mut file) = self.do_write_file();

        let mut data = Self::gen_block_size_data(self.block_size);

        let start_time = time::Instant::now();
        for idx in self.blocks()..0 {
            let _ = file
                .seek(SeekFrom::Start((idx * self.block_size) as u64))
                .unwrap();
            file.read_exact(data.as_mut_slice()).unwrap();
        }
        time::Instant::now() - start_time
    }

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
        for _ in 0..self.blocks() {
            file.write_all(block_data.as_slice()).unwrap();
        }
        let use_time = time::Instant::now() - start_time;
        // set offset to start
        file.seek(SeekFrom::Start(0)).unwrap();
        (use_time, file)
    }

    #[inline(always)]
    const fn blocks(&self) -> usize {
        self.file_size / self.block_size
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
