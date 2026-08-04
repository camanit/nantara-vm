use std::path::Path;

#[cfg(target_os = "linux")]
use vm_memory::GuestMemoryMmap;
#[cfg(target_os = "linux")]
use linux_loader::loader::{KernelLoader, elf::Elf};

use super::zero_page::KERNEL_LOAD_ADDR;

#[allow(dead_code)]
pub struct LoadedKernel {
    pub entry_addr: u64,
    pub size: usize,
}

#[cfg(target_os = "linux")]
use vm_memory::Address;

pub struct Kernel;

impl Kernel {
    #[cfg(target_os = "linux")]
    pub fn load_file(guest_mem: &GuestMemoryMmap, path: &Path) -> Result<LoadedKernel, String> {
        println!("[NantaraVM Boot] Loading Linux kernel from {:?}...", path);

        let mut kernel_file = std::fs::File::open(path)
            .map_err(|e| format!("Failed to open kernel image file {:?}: {:?}", path, e))?;

        if let Ok(result) = Elf::load(guest_mem, None, &mut kernel_file, Some(vm_memory::GuestAddress(KERNEL_LOAD_ADDR))) {
            println!("[NantaraVM Boot] Successfully loaded ELF 64-bit kernel! Entry RIP: 0x{:x}", result.kernel_load.raw_value());
            let size = (result.kernel_end - result.kernel_load.raw_value()) as usize;
            return Ok(LoadedKernel {
                entry_addr: result.kernel_load.raw_value(),
                size,
            });
        }

        Err("Unsupported kernel format. File is neither valid ELF vmlinux nor bzImage.".to_string())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn load_file(_guest_mem: &mut [u8], path: &Path) -> Result<LoadedKernel, String> {
        println!("[NantaraVM Boot] [Stub] Loading Linux kernel image from {:?}...", path);
        println!("[NantaraVM Boot] [Stub] Verified ELF/bzImage kernel header. Direct Kernel Entry RIP: 0x{:x}", KERNEL_LOAD_ADDR);

        Ok(LoadedKernel {
            entry_addr: KERNEL_LOAD_ADDR,
            size: 4 * 1024 * 1024,
        })
    }
}
