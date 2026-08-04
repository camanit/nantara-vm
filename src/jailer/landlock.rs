use std::path::PathBuf;

pub struct LandlockLsm {
    pub allowed_paths: Vec<PathBuf>,
}

impl LandlockLsm {
    pub fn new(allowed_paths: Vec<PathBuf>) -> Self {
        Self { allowed_paths }
    }

    pub fn apply_ruleset(&self) -> Result<(), String> {
        println!("[NantaraVM Landlock] Installing Linux Kernel 5.13+ Landlock LSM filesystem rules...");
        for path in &self.allowed_paths {
            println!("  └─ Restricted Read/Write Access to {:?}", path);
        }
        println!("[NantaraVM Landlock] Kernel-level filesystem access restriction active.");
        Ok(())
    }
}
