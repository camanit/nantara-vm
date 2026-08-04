use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(target_os = "linux")]
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

pub const UEFI_ROM_BASE: u64 = 0xFFF00000; // 4GB - 1MB ROM Top Offset for OVMF.fd
pub const RESET_VECTOR_ADDR: u64 = 0xFFFFFFF0; // x86_64 Reset Vector RIP

pub struct UefiBootloader {
    pub firmware_path: Option<String>,
    pub rom_size: usize,
}

impl UefiBootloader {
    pub fn new() -> Self {
        Self {
            firmware_path: None,
            rom_size: 0,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn load_ovmf(&mut self, guest_mem: &GuestMemoryMmap, ovmf_path: &Path) -> Result<u64, String> {
        println!("[NantaraVM UEFI Engine] Loading OVMF Firmware (UEFI BIOS) from {:?}...", ovmf_path);

        let mut file = File::open(ovmf_path)
            .map_err(|e| format!("Failed to open OVMF firmware image {:?}: {}", ovmf_path, e))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read OVMF firmware image: {}", e))?;

        let size = buffer.len();
        self.rom_size = size;
        self.firmware_path = Some(ovmf_path.to_string_lossy().to_string());

        println!("[NantaraVM UEFI Engine] OVMF ROM Size: {} KB ({:.2} MB)", size / 1024, size as f64 / (1024.0 * 1024.0));
        
        // Write OVMF ROM to Top of Physical Memory space (0xFFF00000)
        let rom_address = GuestAddress(UEFI_ROM_BASE);
        guest_mem.write_slice(&buffer, rom_address)
            .map_err(|e| format!("Failed to write OVMF firmware into Guest RAM: {:?}", e))?;

        println!("[NantaraVM UEFI Engine] OVMF ROM mapped successfully to Top Mem (0x{:X})", UEFI_ROM_BASE);
        println!("[NantaraVM UEFI Engine] x86_64 Reset Vector set at 0x{:X}", RESET_VECTOR_ADDR);
        println!("[NantaraVM UEFI Engine] Ready to boot Full OS (Windows 10/11 / Linux) via UEFI!");

        Ok(RESET_VECTOR_ADDR)
    }

    #[cfg(not(target_os = "linux"))]
    pub fn load_ovmf(&mut self, _guest_mem: &mut [u8], ovmf_path: &Path) -> Result<u64, String> {
        println!("[NantaraVM UEFI Engine] [Stub] Loading OVMF Firmware from {:?}...", ovmf_path);
        println!("[NantaraVM UEFI Engine] [Stub] OVMF ROM mapped at 0x{:X}, Reset Vector: 0x{:X}", UEFI_ROM_BASE, RESET_VECTOR_ADDR);
        Ok(RESET_VECTOR_ADDR)
    }
}
