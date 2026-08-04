pub mod cpu;

pub use self::cpu::ArmVcpu;

pub const GUEST_RAM_SIZE: usize = 512 * 1024 * 1024; // 512 MB ARM64 RAM
pub const BOOT_ADDR_ARM64: u64 = 0x4000_0000; // ARM64 RAM Base Address
