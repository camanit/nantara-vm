use std::path::PathBuf;
use super::{VirtioDevice, TYPE_BLOCK};

pub struct VirtioBlock {
    pub disk_path: PathBuf,
    pub capacity_sectors: u64,
    pub readonly: bool,
}

impl VirtioBlock {
    pub fn new(disk_path: PathBuf, capacity_bytes: u64) -> Self {
        let capacity_sectors = capacity_bytes / 512;
        let capacity_mb = capacity_bytes as f64 / (1024.0 * 1024.0);
        let capacity_gb = capacity_mb / 1024.0;

        println!("[NantaraVM VirtIO-Block] Registering VirtIO Block Drive Device...");
        println!("[NantaraVM VirtIO-Block] Image File: {:?}", disk_path);
        println!("[NantaraVM VirtIO-Block] Capacity: {:.2} MB ({:.2} GB), Sectors: {}", capacity_mb, capacity_gb, capacity_sectors);

        Self {
            disk_path,
            capacity_sectors,
            readonly: false,
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
        println!("[NantaraVM VirtIO-Block] Processing Request Descriptor on Virtqueue {} (Async Sector Read/Write)", queue_index);
        Ok(())
    }
}
