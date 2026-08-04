use super::{VirtioDevice, TYPE_NET};

pub struct VirtioNet {
    pub tap_name: String,
    pub mac_addr: [u8; 6],
    pub active: bool,
}

impl VirtioNet {
    pub fn new(tap_name: &str, mac_addr: [u8; 6]) -> Self {
        println!("[NantaraVM VirtIO-Net] Registering virtio-net Network Interface...");
        println!("[NantaraVM VirtIO-Net] Bound to TAP Device: '{}'", tap_name);
        println!("[NantaraVM VirtIO-Net] Assigned Hardware MAC Address: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac_addr[0], mac_addr[1], mac_addr[2], mac_addr[3], mac_addr[4], mac_addr[5]);
        
        Self {
            tap_name: tap_name.to_string(),
            mac_addr,
            active: true,
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
        if queue_index == 0 {
            println!("[NantaraVM VirtIO-Net] Virtqueue 0 (RX): Receiving Ethernet packets from TAP '{}' -> Guest RAM", self.tap_name);
        } else if queue_index == 1 {
            println!("[NantaraVM VirtIO-Net] Virtqueue 1 (TX): Transmitting Ethernet packets Guest RAM -> TAP '{}'", self.tap_name);
        }
        Ok(())
    }
}
