pub mod mmio;
pub mod queue;
pub mod blk;
pub mod net;
pub mod vsock;

pub const TYPE_NET: u32 = 1;
pub const TYPE_BLOCK: u32 = 2;
#[allow(dead_code)]
pub const TYPE_CONSOLE: u32 = 3;
pub const TYPE_VSOCK: u32 = 19;

pub trait VirtioDevice: Send + Sync {
    fn device_type(&self) -> u32;
    fn queue_max_size(&self) -> u16 {
        256
    }
    fn read_config(&self, offset: u64, data: &mut [u8]);
    #[allow(dead_code)]
    fn write_config(&mut self, offset: u64, data: &[u8]);
    #[allow(dead_code)]
    fn process_queue(&mut self, queue_index: u16) -> Result<(), String>;
}
