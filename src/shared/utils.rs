use rand::{RngCore, SeedableRng};
use std::io::Write;

/// 快速创建一个指定大小的文件
pub fn create_large_file_fast(file: &str, mut size: u64) {
    // 删除文件 - 如果存在
    if std::path::Path::new(file).exists() {
        std::fs::remove_file(file).unwrap();
    }

    let mut rng = rand::rngs::SmallRng::from_entropy();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .open(file)
        .expect("打开文件失败");

    let mut block = [0u8; 4096];

    while size > 0 {
        size -= block.len() as u64;
        rng.fill_bytes(block.as_mut());
        file.write_all(&block[..]).unwrap();
    }
    file.flush().unwrap();
    file.sync_all().unwrap();
}
