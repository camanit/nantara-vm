use super::{VirtioDevice, TYPE_VSOCK};

pub struct VirtioVsock {
    pub guest_cid: u64,
}

impl VirtioVsock {
    pub fn new(guest_cid: u64) -> Self {
        println!("[NantaraVM VirtIO] Initialized virtio-vsock device (Guest CID: {})", guest_cid);
        Self { guest_cid }
    }
}

impl VirtioDevice for VirtioVsock {
    fn device_type(&self) -> u32 {
        TYPE_VSOCK
    }

    fn read_config(&self, offset: u64, data: &mut [u8]) {
        if offset == 0 && data.len() >= 8 {
            data[..8].copy_from_slice(&self.guest_cid.to_le_bytes());
        }
    }

    fn write_config(&mut self, _offset: u64, _data: &[u8]) {}

    fn process_queue(&mut self, queue_index: u16) -> Result<(), String> {
        println!("[NantaraVM VirtIO VSock] Processing Virtqueue {} (Host-Guest Agent IPC)", queue_index);
        Ok(())
    }
}
