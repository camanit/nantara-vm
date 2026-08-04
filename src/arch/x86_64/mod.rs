pub mod cpu;

pub const GUEST_RAM_SIZE: usize = 16 * 1024 * 1024; // 16 MB Guest RAM Baseline
pub const BOOT_ADDR: u64 = 0x1000;                  // Entrypoint 4KB

#[allow(dead_code)]
pub const PML4_ADDR: u64 = 0x9000;                  // Page Table Root 36KB
#[allow(dead_code)]
pub const PDP_ADDR: u64 = 0xa000;                   // Page Directory Pointer Table
#[allow(dead_code)]
pub const PD_ADDR: u64 = 0xb000;                    // Page Directory Table
