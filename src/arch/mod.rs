pub mod x86_64;
pub mod aarch64;

pub use self::x86_64::cpu::Vcpu;
pub use self::aarch64::cpu::ArmVcpu;
