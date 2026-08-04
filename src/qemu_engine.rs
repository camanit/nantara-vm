/// NantaraVM QEMU Engine — Real VM Manager
/// Menjalankan QEMU secara nyata untuk boot ISO Windows 10, Kali Linux, dll.

use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VmConfig {
    pub name: String,
    pub iso_path: Option<String>,
    pub drive_path: Option<String>,
    pub ram_mb: u32,
    pub vcpu: u32,
}

#[derive(Debug)]
pub struct QemuVm {
    pub config: VmConfig,
    pub process: Child,
    pub pid: u32,
}

pub struct QemuEngine {
    pub running_vms: Arc<Mutex<HashMap<String, QemuVm>>>,
    pub qemu_path: String,
}

impl QemuEngine {
    pub fn new() -> Self {
        // Cari QEMU di lokasi standar Windows
        let qemu_path = Self::find_qemu();
        println!("[NantaraVM QEMU] QEMU binary: {}", qemu_path);
        QemuEngine {
            running_vms: Arc::new(Mutex::new(HashMap::new())),
            qemu_path,
        }
    }

    fn find_qemu() -> String {
        let candidates = vec![
            r"C:\Program Files\QEMU\qemu-system-x86_64.exe",
            r"C:\Program Files (x86)\QEMU\qemu-system-x86_64.exe",
            r"C:\qemu\qemu-system-x86_64.exe",
            "qemu-system-x86_64",
        ];
        for path in candidates {
            if std::path::Path::new(path).exists() || path == "qemu-system-x86_64" {
                return path.to_string();
            }
        }
        "qemu-system-x86_64".to_string()
    }

    /// Cek apakah QEMU sudah terinstall
    pub fn is_qemu_available(&self) -> bool {
        Command::new(&self.qemu_path)
            .arg("--version")
            .output()
            .is_ok()
    }

    /// Start VM baru dengan QEMU
    pub fn start_vm(&self, config: VmConfig) -> Result<String, String> {
        if !self.is_qemu_available() {
            return Err(format!(
                "QEMU tidak ditemukan. Silakan install QEMU dari https://qemu.weilnetz.de/w64/ \
                 lalu tambahkan ke PATH Windows."
            ));
        }

        let vm_name = config.name.clone();

        {
            let vms = self.running_vms.lock().unwrap();
            if vms.contains_key(&vm_name) {
                return Err(format!("VM '{}' sudah berjalan.", vm_name));
            }
        }

        let mut cmd = Command::new(&self.qemu_path);

        // RAM
        cmd.arg("-m").arg(format!("{}M", config.ram_mb));

        // vCPU
        cmd.arg("-smp").arg(format!("{}", config.vcpu));

        // Akselerasi hardware (WHPX di Windows, fallback ke tcg)
        cmd.arg("-accel").arg("whpx,kernel-irqchip=off");

        // VGA Display
        cmd.arg("-vga").arg("std");

        // Display window
        cmd.arg("-display").arg("sdl,grab-on-hover=on");

        // ISO CD-ROM
        if let Some(ref iso) = config.iso_path {
            if std::path::Path::new(iso).exists() {
                cmd.arg("-cdrom").arg(iso);
                cmd.arg("-boot").arg("d"); // Boot dari CD-ROM
                println!("[NantaraVM QEMU] Mounting ISO: {}", iso);
            } else {
                return Err(format!("File ISO tidak ditemukan: {}", iso));
            }
        }

        // Hard Disk (jika ada)
        if let Some(ref drive) = config.drive_path {
            cmd.arg("-drive")
                .arg(format!("file={},format=qcow2,if=virtio", drive));
        }

        // Network
        cmd.arg("-netdev").arg("user,id=net0");
        cmd.arg("-device").arg("virtio-net-pci,netdev=net0");

        // Audio (opsional)
        cmd.arg("-soundhw").arg("hda");

        println!("[NantaraVM QEMU] Menjalankan perintah: {:?}", cmd);

        match cmd.spawn() {
            Ok(child) => {
                let pid = child.id();
                let mut vms = self.running_vms.lock().unwrap();
                vms.insert(
                    vm_name.clone(),
                    QemuVm {
                        config,
                        process: child,
                        pid,
                    },
                );
                println!("[NantaraVM QEMU] VM '{}' berhasil distart! PID: {}", vm_name, pid);
                Ok(format!("VM '{}' berhasil dijalankan! PID: {}", vm_name, pid))
            }
            Err(e) => {
                eprintln!("[NantaraVM QEMU Error] Gagal start VM '{}': {}", vm_name, e);
                // Coba fallback tanpa WHPX (mode software emulation)
                self.start_vm_tcg_fallback(config)
            }
        }
    }

    /// Fallback: jalankan tanpa hardware acceleration (lebih lambat tapi tetap jalan)
    fn start_vm_tcg_fallback(&self, config: VmConfig) -> Result<String, String> {
        let vm_name = config.name.clone();
        println!("[NantaraVM QEMU] Mencoba fallback mode TCG (software emulation)...");

        let mut cmd = Command::new(&self.qemu_path);
        cmd.arg("-m").arg(format!("{}M", config.ram_mb));
        cmd.arg("-smp").arg(format!("{}", config.vcpu));
        cmd.arg("-accel").arg("tcg");
        cmd.arg("-vga").arg("std");

        if let Some(ref iso) = config.iso_path {
            if std::path::Path::new(iso).exists() {
                cmd.arg("-cdrom").arg(iso);
                cmd.arg("-boot").arg("d");
            }
        }

        if let Some(ref drive) = config.drive_path {
            cmd.arg("-drive")
                .arg(format!("file={},format=qcow2", drive));
        }

        match cmd.spawn() {
            Ok(child) => {
                let pid = child.id();
                let mut vms = self.running_vms.lock().unwrap();
                vms.insert(vm_name.clone(), QemuVm { config, process: child, pid });
                Ok(format!("VM '{}' berjalan dalam mode TCG. PID: {}", vm_name, pid))
            }
            Err(e) => Err(format!(
                "Gagal menjalankan QEMU: {}. \
                 Pastikan QEMU sudah terinstall dan ada di PATH.",
                e
            )),
        }
    }

    /// Stop VM
    pub fn stop_vm(&self, vm_name: &str) -> Result<String, String> {
        let mut vms = self.running_vms.lock().unwrap();
        if let Some(mut vm) = vms.remove(vm_name) {
            let _ = vm.process.kill();
            println!("[NantaraVM QEMU] VM '{}' dihentikan.", vm_name);
            Ok(format!("VM '{}' berhasil dihentikan.", vm_name))
        } else {
            Err(format!("VM '{}' tidak ditemukan atau sudah berhenti.", vm_name))
        }
    }

    /// List VM yang sedang berjalan
    pub fn list_running_vms(&self) -> Vec<(String, u32)> {
        let vms = self.running_vms.lock().unwrap();
        vms.iter()
            .map(|(name, vm)| (name.clone(), vm.pid))
            .collect()
    }
}
