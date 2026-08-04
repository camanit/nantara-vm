use super::{VirtioDevice, TYPE_NET};

pub struct VirtioNet {
    #[allow(dead_code)]
    pub tap_name: String,
    pub mac_addr: [u8; 6],
}

impl VirtioNet {
    pub fn new(tap_name: &str, mac_addr: [u8; 6]) -> Self {
        println!("[NantaraVM VirtIO] Initialized virtio-net device (TAP: {}, MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x})",
            tap_name, mac_addr[0], mac_addr[1], mac_addr[2], mac_addr[3], mac_addr[4], mac_addr[5]);
        Self {
            tap_name: tap_name.to_string(),
            mac_addr,
        }
    }
}

impl VirtioDevice for VirtioNet {
    fn device_type(&self) -> u32 {
        TYPE_NET
    }

    fn read_config(&self, offset: u64, data: &mut [u8]) {
        if offset == 0 && data.len() >= 6 {
            data[..6].copy_from_slice(&self.mac_addr);
        }
    }

    fn write_config(&mut self, _offset: u64, _data: &[u8]) {}

    fn process_queue(&mut self, queue_index: u16) -> Result<(), String> {
        println!("[NantaraVM VirtIO Net] Processing Virtqueue {} (TAP Frame Forwarding)", queue_index);
        Ok(())
    }
}
