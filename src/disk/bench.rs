/// 读写测试结果
pub struct IOResult {
    read: f64,
}

/// 磁盘性能测试结果
pub struct DiskResult {
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
    pub fn seq_write(&self) {}

    /// 顺序 读
    pub fn seq_read(&self) {}

    /// 顺序 重写
    pub fn seq_re_write(&self) {}

    /// 顺序 重读
    pub fn seq_re_read(&self) {}

    /// 随机 写
    pub fn rand_write(&self) {}

    /// 随机 读
    pub fn rand_read(&self) {}

    /// 跳 写
    pub fn stride_write(&self) {}

    /// 跳 读
    pub fn stride_read(&self) {}

    /// 倒 写
    pub fn reverse_write(&self) {}

    /// 倒 读
    pub fn reverse_read(&self) {}
}
