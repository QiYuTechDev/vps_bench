use std::time;
use std::time::Instant;

use rand::{Rng, SeedableRng};

use super::RamResult;
use crate::shared::IOTime;

/// RAM 读写速度测试
pub struct RamBench {
    /// memory 数据
    pub mem: Vec<u8>,
}

impl RamBench {
    /// 创建一个新的 RAMBench 测试
    /// `mem_size` 单位为: MB
    pub fn new(mem_size: usize) -> Self {
        Self {
            mem: Vec::with_capacity(mem_size * 1024 * 1024),
        }
    }

    /// 运行一次性能测试
    pub fn run_bench(&mut self) -> RamResult {
        let mut result = RamResult::new(self.mem.capacity() as u64);

        result.seq = IOTime::new(self.seq_read_data().0, self.seq_write_data());
        result.rand = IOTime::new(self.rand_read_data().0, self.rand_write_data());

        result
    }

    #[inline(never)]
    /// 随机写内存数据
    pub fn rand_write_data(&mut self) -> time::Duration {
        self.mem.truncate(0);

        let mem_size = self.mem.capacity();

        let gen_position_time = crate::shared::gen_n_usize_range_random_time(mem_size, mem_size);
        let gen_value_time = crate::shared::gen_n_u8_random_time(mem_size);

        let mut rng = rand::rngs::SmallRng::from_entropy();

        let ptr = self.mem.as_mut_ptr();

        let start_time = time::Instant::now();
        for _ in 0..mem_size {
            let position = rng.gen_range(0..mem_size);
            let value: u8 = rng.gen();
            unsafe {
                ptr.add(position).write(value);
            }
        }
        Instant::now() - start_time - gen_position_time - gen_value_time
    }

    #[inline(never)]
    /// 内存随机读取数据
    pub fn rand_read_data(&mut self) -> (time::Duration, u8) {
        self.mem.truncate(0);

        let ptr = self.mem.as_mut_ptr();
        let mem_size = self.mem.capacity();

        let gen_position_time = crate::shared::gen_n_usize_range_random_time(mem_size, mem_size);

        let mut rng = rand::rngs::SmallRng::from_entropy();

        let mut r = 0u8;
        let start_time = time::Instant::now();
        for _ in 0..mem_size {
            let position = rng.gen_range(0..mem_size);
            unsafe { r += ptr.add(position).read() };
        }
        (Instant::now() - start_time - gen_position_time, r)
    }

    /// 顺序写内存数据
    #[inline(never)]
    pub fn seq_write_data(&mut self) -> time::Duration {
        unsafe { self.mem.set_len(self.mem.capacity()) };
        let start_time = time::Instant::now();
        self.mem.fill(0);
        Instant::now() - start_time
    }

    /// 顺序读 数据
    #[inline(never)]
    pub fn seq_read_data(&mut self) -> (time::Duration, u8) {
        let ptr = self.mem.as_mut_ptr();
        let mem_size = self.mem.capacity();

        let mut r = 0u8;
        let start_time = time::Instant::now();
        for i in 0..mem_size {
            let v = unsafe { ptr.add(i).read() };
            r += v;
        }
        (Instant::now() - start_time, r)
    }

    #[inline(never)]
    /// 随机生成 `n` 个 u8 类型的数据，并且计算和使用的时间
    pub fn add_n_random_time(n: usize) -> (time::Duration, u8) {
        let gen_time = crate::shared::gen_n_u8_random_time(n);

        let mut rng = rand::rngs::SmallRng::from_entropy();
        let mut r = 0u8;

        let start_time = time::Instant::now();
        for _ in 0..n {
            let v: u8 = rng.gen();
            r += v;
        }
        (Instant::now() - start_time - gen_time, r)
    }
}

#[test]
pub fn test_seq_write_ram() {
    let mut ram = RamBench::new(1);
    let use_time = ram.seq_write_data();
    println!("use time: {:?}", use_time);
}
