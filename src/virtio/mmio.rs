use std::sync::{Arc, Mutex};
use super::VirtioDevice;
use super::queue::VirtQueueWrapper;

pub const VIRTIO_MMIO_MAGIC: u32 = 0x74726976; // "virt"
pub const VIRTIO_MMIO_VERSION: u32 = 2;         // VirtIO 1.0+
pub const VIRTIO_MMIO_VENDOR_ID: u32 = 0x554d564e; // "NVMN" (NantaraVM)

pub struct MmioTransport {
    pub device: Arc<Mutex<dyn VirtioDevice>>,
    pub queues: Vec<VirtQueueWrapper>,
    pub selected_queue: u16,
    pub status: u32,
    pub interrupt_status: u32,
    pub base_addr: u64,
}

impl MmioTransport {
    pub fn new(base_addr: u64, device: Arc<Mutex<dyn VirtioDevice>>) -> Self {
        let max_size = device.lock().unwrap().queue_max_size();
        let queues = vec![VirtQueueWrapper::new(max_size), VirtQueueWrapper::new(max_size)];
        Self {
            device,
            queues,
            selected_queue: 0,
            status: 0,
            interrupt_status: 0,
            base_addr,
        }
    }

    pub fn read_mmio(&self, offset: u64, data: &mut [u8]) {
        if offset >= 0x100 {
            self.device.lock().unwrap().read_config(offset - 0x100, data);
            return;
        }

        if data.len() != 4 {
            return;
        }

        let val = match offset {
            0x00 => VIRTIO_MMIO_MAGIC,
            0x04 => VIRTIO_MMIO_VERSION,
            0x08 => self.device.lock().unwrap().device_type(),
            0x0c => VIRTIO_MMIO_VENDOR_ID,
            0x34 => self.device.lock().unwrap().queue_max_size() as u32,
            0x44 => {
                if (self.selected_queue as usize) < self.queues.len() {
                    self.queues[self.selected_queue as usize].ready as u32
                } else {
                    0
                }
            }
            0x60 => self.interrupt_status,
            0x70 => self.status,
            _ => 0,
        };

        data.copy_from_slice(&val.to_le_bytes());
    }

    #[allow(dead_code)]
    pub fn write_mmio(&mut self, offset: u64, data: &[u8]) {
        if offset >= 0x100 {
            self.device.lock().unwrap().write_config(offset - 0x100, data);
            return;
        }

        if data.len() != 4 {
            return;
        }

        let val = u32::from_le_bytes(data.try_into().unwrap());
        match offset {
            0x30 => self.selected_queue = val as u16,
            0x44 => {
                if (self.selected_queue as usize) < self.queues.len() {
                    self.queues[self.selected_queue as usize].set_ready(val == 1);
                }
            }
            0x50 => {
                let q_idx = val as u16;
                let _ = self.device.lock().unwrap().process_queue(q_idx);
            }
            0x64 => self.interrupt_status &= !val,
            0x70 => self.status = val,
            _ => {}
        }
    }
}
