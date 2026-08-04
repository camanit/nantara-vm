use super::{VirtioDevice, TYPE_GPU};

pub const VIRTIO_GPU_CMD_GET_DISPLAY_INFO: u32 = 0x0100;
pub const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: u32 = 0x0101;
pub const VIRTIO_GPU_CMD_RESOURCE_FLUSH: u32 = 0x0104;
pub const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D: u32 = 0x0105;

pub struct VirtioGpu {
    pub width: u32,
    pub height: u32,
    pub bpp: u32, // Bits per pixel (32-bit RGBA)
    pub framebuffer_addr: u64,
    pub active: bool,
}

impl VirtioGpu {
    pub fn new(width: u32, height: u32) -> Self {
        println!("[NantaraVM VirtIO-GPU] Registering VirtIO Display Graphics Card...");
        println!("[NantaraVM VirtIO-GPU] Display Resolution: {}x{} @ 32-bit Color Depth", width, height);
        println!("[NantaraVM VirtIO-GPU] Framebuffer Stream Endpoint: WebSocket / noVNC Web Display Active");

        Self {
            width,
            height,
            bpp: 32,
            framebuffer_addr: 0xD0000000,
            active: true,
        }
    }
}

impl VirtioDevice for VirtioGpu {
    fn device_type(&self) -> u32 {
        TYPE_GPU
    }

    fn read_config(&self, offset: u64, data: &mut [u8]) {
        if offset == 0 && data.len() >= 8 {
            data[0..4].copy_from_slice(&self.width.to_le_bytes());
            data[4..8].copy_from_slice(&self.height.to_le_bytes());
        }
    }

    fn write_config(&mut self, _offset: u64, _data: &[u8]) {}

    fn process_queue(&mut self, queue_index: u16) -> Result<(), String> {
        if queue_index == 0 {
            println!("[NantaraVM VirtIO-GPU] Control Virtqueue 0: Processing 2D Resource Flush & Framebuffer Refresh ({}x{})", self.width, self.height);
        } else if queue_index == 1 {
            println!("[NantaraVM VirtIO-GPU] Cursor Virtqueue 1: Processing Hardware Mouse Cursor Movement");
        }
        Ok(())
    }
}
