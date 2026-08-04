#[cfg(target_os = "linux")]
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestMemory};

pub const ZERO_PAGE_ADDR: u64 = 0x7000;
#[allow(dead_code)]
pub const CMDLINE_ADDR: u64 = 0x20000;
pub const KERNEL_LOAD_ADDR: u64 = 0x100000; // 1MB Offset

#[allow(dead_code)]
pub const E820_RAM: u32 = 1;
#[allow(dead_code)]
pub const E820_RESERVED: u32 = 2;

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct E820Entry {
    pub addr: u64,
    pub size: u64,
    pub entry_type: u32,
}

#[cfg(target_os = "linux")]
unsafe impl vm_memory::ByteValued for E820Entry {}

pub struct ZeroPage;

impl ZeroPage {
    #[cfg(target_os = "linux")]
    pub fn setup(
        guest_mem: &GuestMemoryMmap,
        ram_size: u64,
        cmdline: &str,
    ) -> Result<(), String> {
        println!("[NantaraVM Boot] Setting up Zero-Page (boot_params) at 0x{:x}...", ZERO_PAGE_ADDR);

        let cmdline_bytes = cmdline.as_bytes();
        guest_mem.write_slice(cmdline_bytes, GuestAddress(CMDLINE_ADDR))
            .map_err(|e| format!("Failed to write cmdline: {:?}", e))?;
        guest_mem.write_obj(0u8, GuestAddress(CMDLINE_ADDR + cmdline_bytes.len() as u64))
            .map_err(|e| format!("Failed to write cmdline null terminator: {:?}", e))?;

        let e820_map = vec![
            E820Entry { addr: 0, size: 0x9f000, entry_type: E820_RAM },
            E820Entry { addr: 0x9f000, size: 0x1000, entry_type: E820_RESERVED },
            E820Entry { addr: KERNEL_LOAD_ADDR, size: ram_size - KERNEL_LOAD_ADDR, entry_type: E820_RAM },
        ];

        let header_magic: u32 = 0x53726448;
        guest_mem.write_obj(header_magic, GuestAddress(ZERO_PAGE_ADDR + 0x202))
            .map_err(|e| format!("Failed to write HdrS magic: {:?}", e))?;

        guest_mem.write_obj(CMDLINE_ADDR as u32, GuestAddress(ZERO_PAGE_ADDR + 0x228))
            .map_err(|e| format!("Failed to write cmdline_ptr: {:?}", e))?;

        guest_mem.write_obj(0xffu8, GuestAddress(ZERO_PAGE_ADDR + 0x210))
            .map_err(|e| format!("Failed to write type_of_loader: {:?}", e))?;

        guest_mem.write_obj(e820_map.len() as u8, GuestAddress(ZERO_PAGE_ADDR + 0x1e8))
            .map_err(|e| format!("Failed to write e820 entries count: {:?}", e))?;

        for (i, entry) in e820_map.iter().enumerate() {
            let offset = ZERO_PAGE_ADDR + 0x2d0 + (i * std::mem::size_of::<E820Entry>()) as u64;
            guest_mem.write_obj(*entry, GuestAddress(offset))
                .map_err(|e| format!("Failed to write E820 entry {}: {:?}", i, e))?;
        }

        println!("[NantaraVM Boot] Zero-Page initialized successfully with {} E820 entries.", e820_map.len());
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn setup(
        _guest_mem: &mut [u8],
        ram_size: u64,
        cmdline: &str,
    ) -> Result<(), String> {
        println!("[NantaraVM Boot] [Stub] Setting up Zero-Page (boot_params) at 0x{:x}...", ZERO_PAGE_ADDR);
        println!("[NantaraVM Boot] [Stub] Command Line: \"{}\"", cmdline);
        println!("[NantaraVM Boot] [Stub] E820 RAM Region: 0x{:x} - 0x{:x} ({} MB)", KERNEL_LOAD_ADDR, ram_size, ram_size / (1024 * 1024));
        Ok(())
    }
}
