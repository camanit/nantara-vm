#[cfg(target_os = "linux")]
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestMemory};

#[cfg(target_os = "linux")]
use kvm_ioctls::{VcpuFd, VcpuExit};
#[cfg(target_os = "linux")]
use kvm_bindings::{kvm_sregs, kvm_regs, kvm_segment};

#[cfg(target_os = "linux")]
use super::{BOOT_ADDR, PML4_ADDR, PDP_ADDR, PD_ADDR};

pub struct Vcpu {
    pub id: u8,
    #[cfg(target_os = "linux")]
    pub vcpu_fd: VcpuFd,
}

impl Vcpu {
    #[cfg(target_os = "linux")]
    pub fn new(id: u8, vcpu_fd: VcpuFd) -> Self {
        Self { id, vcpu_fd }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    /// Setup 64-bit Long Mode Registers & Identity Paging
    #[cfg(target_os = "linux")]
    pub fn setup_long_mode(&self, mem: &GuestMemoryMmap) -> Result<(), String> {
        // 1. Write Page Tables for Identity Mapping (0 - 2MB)
        mem.write_obj(PDP_ADDR | 0x03, GuestAddress(PML4_ADDR))
            .map_err(|e| format!("Failed to write PML4: {:?}", e))?;

        mem.write_obj(PD_ADDR | 0x03, GuestAddress(PDP_ADDR))
            .map_err(|e| format!("Failed to write PDP: {:?}", e))?;

        mem.write_obj(0x0000_0083u64, GuestAddress(PD_ADDR))
            .map_err(|e| format!("Failed to write PD: {:?}", e))?;

        // 2. Setup Special Registers (sregs) for Long Mode
        let mut sregs: kvm_sregs = self.vcpu_fd.get_sregs().map_err(|e| e.to_string())?;
        sregs.cr3 = PML4_ADDR;
        sregs.cr4 = 1 << 5; // CR4_PAE
        sregs.cr0 = (1 << 0) | (1 << 31); // CR0_PE | CR0_PG
        sregs.efer = (1 << 8) | (1 << 10); // EFER_LME | EFER_LMA

        let seg = kvm_segment {
            base: 0,
            limit: 0xffff_ffff,
            selector: 1 << 3,
            type_: 11, // Execute/Read
            present: 1,
            dpl: 0,
            db: 0,
            s: 1,
            l: 1, // Long mode
            g: 1,
            ..Default::default()
        };
        sregs.cs = seg;
        sregs.ds = seg;
        sregs.es = seg;
        sregs.ss = seg;

        self.vcpu_fd.set_sregs(&sregs).map_err(|e| e.to_string())?;

        // 3. Setup General Purpose Registers (regs)
        let mut regs: kvm_regs = self.vcpu_fd.get_regs().map_err(|e| e.to_string())?;
        regs.rip = BOOT_ADDR;
        regs.rflags = 0x2;

        self.vcpu_fd.set_regs(&regs).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn setup_long_mode(&self, _mem: &[u8]) -> Result<(), String> {
        println!("[NantaraVM] [Stub] Configured 64-bit Long Mode registers (CR0, CR3, CR4, EFER, CS).");
        Ok(())
    }

    /// Load payload and execute the vCPU loop
    #[cfg(target_os = "linux")]
    pub fn run_loop(&self, _mem: &GuestMemoryMmap) -> Result<(), String> {
        println!("[NantaraVM] Starting vCPU {} execution loop...", self.id);
        loop {
            match self.vcpu_fd.run().map_err(|e| e.to_string())? {
                VcpuExit::IoOut(port, data) => {
                    if port == 0x3f8 {
                        for byte in data {
                            print!("{}", *byte as char);
                        }
                    }
                }
                VcpuExit::Hlt => {
                    println!("\n[NantaraVM] vCPU {} halted cleanly.", self.id);
                    break;
                }
                VcpuExit::MmioRead(addr, _) => {
                    println!("[NantaraVM] MMIO Read at 0x{:x}", addr);
                }
                VcpuExit::MmioWrite(addr, data) => {
                    println!("[NantaraVM] MMIO Write at 0x{:x}: {:?}", addr, data);
                }
                exit_reason => {
                    println!("[NantaraVM] Unhandled vCPU exit: {:?}", exit_reason);
                    break;
                }
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn run_loop(&self, _mem: &[u8]) -> Result<(), String> {
        println!("[NantaraVM] Simulating vCPU {} execution on non-Linux platform...", self.id);
        print!("[NantaraVM Serial COM1] NantaraVM NKRI 2026 Booted!\n");
        println!("[NantaraVM] vCPU {} halted cleanly.", self.id);
        Ok(())
    }
}
