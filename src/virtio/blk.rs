use std::path::PathBuf;
use super::{VirtioDevice, TYPE_BLOCK};

pub struct VirtioBlock {
    #[allow(dead_code)]
    pub disk_path: PathBuf,
    pub capacity_sectors: u64,
}

impl VirtioBlock {
    pub fn new(disk_path: PathBuf, capacity_bytes: u64) -> Self {
        let capacity_sectors = capacity_bytes / 512;
        println!("[NantaraVM VirtIO] Initialized virtio-blk device (Path: {:?}, Size: {} MB, Sectors: {})",
            disk_path, capacity_bytes / (1024 * 1024), capacity_sectors);
        Self {
            disk_path,
            capacity_sectors,
        }
    }
}

impl VirtioDevice for VirtioBlock {
    fn device_type(&self) -> u32 {
        TYPE_BLOCK
    }

    fn read_config(&self, offset: u64, data: &mut [u8]) {
        if offset == 0 && data.len() >= 8 {
            data[..8].copy_from_slice(&self.capacity_sectors.to_le_bytes());
        }
    }

    fn write_config(&mut self, _offset: u64, _data: &[u8]) {}

    fn process_queue(&mut self, queue_index: u16) -> Result<(), String> {
        println!("[NantaraVM VirtIO Block] Processing Virtqueue {} (Read/Write Disk I/O)", queue_index);
        Ok(())
    }
}
