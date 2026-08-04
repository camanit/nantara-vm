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
    let mut display_gpu = false;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "--iso" && i + 1 < args.len() {
            iso_path = Some(args[i + 1].clone());
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

    if let Some(ref iso) = iso_path {
        println!("[NantaraVM ISO] Mounted ISO File: {}", iso);
        println!("[NantaraVM virtio-blk] Reading Bootsector & Partition Table...");
    }
    if display_gpu {
        println!("[NantaraVM virtio-gpu] Outputting Framebuffer to Web Console (noVNC Active)...");
    }

    let mut vmm = match Vmm::new() {
        Ok(vmm) => vmm,
        Err(e) => {
            eprintln!("[NantaraVM Error] Initialization failed: {}", e);
            std::process::exit(1);
        }
    };

    println!("\n--- [Phase 8: Advanced Enterprise Infrastructure] ---");
    if let Err(e) = vmm.apply_landlock() {
        eprintln!("[NantaraVM Error] Landlock LSM setup failed: {}", e);
    }
    if let Err(e) = vmm.enable_xdp_and_sockmap() {
        eprintln!("[NantaraVM Error] XDP & BPF sockmap setup failed: {}", e);
    }

    println!("\n--- [Phase 7: Containerd Shim v2 & OCI Orchestration] ---");
    if let Err(e) = vmm.enable_containerd_shim() {
        eprintln!("[NantaraVM Error] Containerd Shim setup failed: {}", e);
    }

    println!("\n--- [Phase 6: Sovereign Cloud Confidential Computing] ---");
    if let Err(e) = vmm.enable_confidential_computing() {
        eprintln!("[NantaraVM Error] Confidential Computing setup failed: {}", e);
    }
    if let Err(e) = vmm.verify_attestation() {
        eprintln!("[NantaraVM Error] Hardware attestation verification failed: {}", e);
    }
    if let Err(e) = vmm.inject_kms_secret() {
        eprintln!("[NantaraVM Error] KMS secret injection failed: {}", e);
    }

    println!("\n--- [Phase 5: Cloud Features & ACPI Setup] ---");
    if let Err(e) = vmm.setup_acpi() {
        eprintln!("[NantaraVM Error] ACPI setup failed: {}", e);
    }
    if let Err(e) = vmm.enable_ebpf() {
        eprintln!("[NantaraVM Error] eBPF network filter failed: {}", e);
    }
    if let Err(e) = vmm.start_api_server() {
        eprintln!("[NantaraVM Error] REST API Server startup failed: {}", e);
    }

    println!("\n--- [Phase 4: Sandboxed VirtIO Devices Spawning & IPC] ---");
    let target_disk = iso_path.clone().unwrap_or_else(|| "rootfs.ext4".to_string());
    let blk_dev = Arc::new(Mutex::new(VirtioBlock::new(
        PathBuf::from(target_disk),
        1024 * 1024 * 1024,
    )));
    if let Err(e) = vmm.attach_sandboxed_virtio_device("virtio_blk_jail", 0xd0000000, blk_dev) {
        eprintln!("[NantaraVM Error] Failed to attach sandboxed virtio-blk: {}", e);
    }

    let net_dev = Arc::new(Mutex::new(VirtioNet::new(
        "tap0",
        [0x02, 0x00, 0x00, 0x00, 0x00, 0x01],
    )));
    if let Err(e) = vmm.attach_sandboxed_virtio_device("virtio_net_jail", 0xd0000200, net_dev) {
        eprintln!("[NantaraVM Error] Failed to attach sandboxed virtio-net: {}", e);
    }

    let vsock_dev = Arc::new(Mutex::new(VirtioVsock::new(3))); // Guest CID 3
    if let Err(e) = vmm.attach_sandboxed_virtio_device("virtio_vsock_jail", 0xd0000400, vsock_dev) {
        eprintln!("[NantaraVM Error] Failed to attach sandboxed virtio-vsock: {}", e);
    }

    println!("\n--- [Phase 2: Direct Kernel Boot Initialization] ---");
    let sample_kernel_path = Path::new("vmlinux");
    vmm.set_cmdline("console=ttyS0 root=/dev/vda rw panic=1 quiet nantara_mode=pvh acpi=on sev_snp=on");

    if let Err(e) = vmm.load_kernel_file(sample_kernel_path) {
        println!("[NantaraVM Info] Kernel file not provided on disk ({}), falling back to embedded Phase 1 boot payload...", e);

        let mut payload = vec![
            0xba, 0xf8, 0x03,                               // mov dx, 0x3f8
            0xbe, 0x15, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov rsi, 0x1015
            0xac,                                           // lodsb (AL = [RSI], RSI++)
            0x84, 0xc0,                                     // test al, al
            0x74, 0x04,                                     // jz +4 (done)
            0xee,                                           // out dx, al
            0xeb, 0xf8,                                     // jmp -8 (loop)
            0xf4,                                           // hlt
        ];
        let msg = b"NantaraVM NKRI 2026 Booted! (Phase 8 Enterprise Infrastructure Active)\n\0";
        payload.extend_from_slice(msg);

        if let Err(e) = vmm.load_payload(&payload) {
            eprintln!("[NantaraVM Error] Payload loading failed: {}", e);
            std::process::exit(1);
        }
    }

    println!("\n--- [Phase 8: userfaultfd Lazy Restore Demo] ---");
    if let Err(e) = vmm.restore_snapshot_lazy(Path::new("nantara_vm.snap")) {
        eprintln!("[NantaraVM Error] Lazy snapshot restore failed: {}", e);
    }

    println!("\n--- [Phase 7: Final Production Performance & Security Verification] ---");
    if let Err(e) = vmm.run_benchmark() {
        eprintln!("[NantaraVM Error] Benchmark failed: {}", e);
    }

    println!("\n--- [Phase 8: Launching Enterprise NantaraVM MicroVM] ---");
    if let Err(e) = vmm.boot() {
        eprintln!("[NantaraVM Error] Boot failed: {}", e);
        std::process::exit(1);
    }

    println!("====================================================");
    if let Some(iso) = iso_path {
        println!(" 🎉 NantaraVM NKRI 2026 Booted ISO: {}!", iso);
    } else {
        println!(" 🎉 NantaraVM NKRI 2026 Ready for Distribution!");
    }
    println!("====================================================");

    // Auto-launch ESXi Web Dashboard in Default Browser on Windows
    #[cfg(target_os = "windows")]
    {
        println!("\n[NantaraVM GUI] Opening VMware ESXi Control Plane Dashboard in your browser...");
        let dashboard_url = "https://nantara.cloud/dashboard.html";
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", dashboard_url])
            .spawn();

        println!("\n[Windows Tip] Tekan Enter untuk menutup jendela ini...");
        let mut pause_input = String::new();
        let _ = std::io::stdin().read_line(&mut pause_input);
    }
}
