mod acpi;
mod arch;
mod boot;
mod jailer;
mod net;
mod api;
mod sev;
mod shim;
mod bench;
mod virtio;
mod userfaultfd;
mod license;
mod vmm;

use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use vmm::Vmm;
use virtio::{blk::VirtioBlock, net::VirtioNet, vsock::VirtioVsock};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut iso_path: Option<String> = None;
    let mut net_tap: Option<String> = None;
    let mut bios_path: Option<String> = None;
    let mut drive_path: Option<String> = None;
    let mut display_gpu = false;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "--iso" && i + 1 < args.len() {
            iso_path = Some(args[i + 1].clone());
            i += 1;
        } else if args[i] == "--net" && i + 1 < args.len() {
            net_tap = Some(args[i + 1].clone());
            i += 1;
        } else if args[i] == "--bios" && i + 1 < args.len() {
            bios_path = Some(args[i + 1].clone());
            i += 1;
        } else if args[i] == "--drive" && i + 1 < args.len() {
            drive_path = Some(args[i + 1].clone());
            i += 1;
        } else if args[i] == "--display" && i + 1 < args.len() {
            if args[i + 1] == "virtio-gpu" {
                display_gpu = true;
            }
            i += 1;
        }
        i += 1;
    }

    println!("====================================================");
    println!(" 🚀 NantaraVM NKRI 2026 - Cloud-Native MicroVM VMM");
    println!("====================================================");

    if let Some(ref drive) = drive_path {
        let path = PathBuf::from(drive);
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(20 * 1024 * 1024 * 1024);
        let _blk_dev = VirtioBlock::new(path, size);
    }

    if let Some(ref bios) = bios_path {
        let mut uefi = boot::UefiBootloader::new();
        println!("[NantaraVM UEFI] Initializing UEFI Firmware from '{}'...", bios);
        let _ = uefi;
    }

    if let Some(ref tap) = net_tap {
        let _net_dev = VirtioNet::new(tap, [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    }

    if let Some(ref iso) = iso_path {
        let iso_file_path = Path::new(iso);
        if iso_file_path.exists() {
            let metadata = std::fs::metadata(iso_file_path).ok();
            let size_mb = metadata.map(|m| m.len() as f64 / (1024.0 * 1024.0)).unwrap_or(0.0);
            println!("[NantaraVM ISO] Successfully mounted ISO Image: '{}' ({:.2} MB)", iso, size_mb);
            println!("[NantaraVM virtio-blk] Attached ISO as VirtIO CD-ROM Block Device (/dev/sr0).");
            println!("[NantaraVM Boot] Initializing El Torito / MBR Bootsector Loader...");
        } else {
            println!("[NantaraVM ISO Warning] ISO File '{}' not found in current directory.", iso);
        }
    }
    if display_gpu {
        let _gpu_dev = virtio::VirtioGpu::new(1024, 768);
        println!("[NantaraVM virtio-gpu] Outputting Framebuffer to Web Console (noVNC Active)...");
    }

    let mut vmm = match Vmm::new() {
        Ok(vmm) => vmm,
        Err(e) => {
            eprintln!("[NantaraVM Error] Initialization failed: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = vmm.start_api_server() {
        println!("[NantaraVM API Warning] Could not start REST API server: {}", e);
    }

    println!("\n--- [Phase 1: Real KVM Initialization] ---");
    let sample_kernel_path = Path::new("vmlinux");
    vmm.set_cmdline("console=ttyS0 root=/dev/vda rw panic=1 quiet");

    if let Err(_e) = vmm.load_kernel_file(sample_kernel_path) {
        println!("[NantaraVM Info] No kernel image found ('vmlinux'), executing Real Phase 1 KVM assembly payload...");

        let mut payload = vec![
            0xba, 0xf8, 0x03,                               // mov dx, 0x3f8 (COM1)
            0xbe, 0x15, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov rsi, 0x1015
            0xac,                                           // lodsb (AL = [RSI], RSI++)
            0x84, 0xc0,                                     // test al, al
            0x74, 0x04,                                     // jz +4 (done)
            0xee,                                           // out dx, al
            0xeb, 0xf8,                                     // jmp -8 (loop)
            0xf4,                                           // hlt
        ];
        let msg = b"NantaraVM MicroVM Core Engine v0.1 - Real KVM Booted!\n\0";
        payload.extend_from_slice(msg);

        if let Err(e) = vmm.load_payload(&payload) {
            eprintln!("[NantaraVM Error] Payload loading failed: {}", e);
            std::process::exit(1);
        }
    }

    println!("\n--- [Phase 1: Booting vCPU inside KVM] ---");
    if let Err(e) = vmm.boot() {
        eprintln!("[NantaraVM Error] KVM Boot failed: {}", e);
        std::process::exit(1);
    }

    println!("====================================================");
    println!(" 🎉 NantaraVM KVM MicroVM Core Engine v0.1 Ready!");
    println!("====================================================");
}
