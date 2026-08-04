#[cfg(target_os = "linux")]
use virtio_queue::Queue;

#[allow(dead_code)]
pub struct VirtQueueWrapper {
    pub size: u16,
    pub ready: bool,
    pub desc_table_addr: u64,
    pub avail_ring_addr: u64,
    pub used_ring_addr: u64,
}

impl VirtQueueWrapper {
    pub fn new(max_size: u16) -> Self {
        Self {
            size: max_size,
            ready: false,
            desc_table_addr: 0,
            avail_ring_addr: 0,
            used_ring_addr: 0,
        }
    }

    #[allow(dead_code)]
    pub fn set_ready(&mut self, ready: bool) {
        self.ready = ready;
    }
}
