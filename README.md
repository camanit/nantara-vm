<div align="center">

<img src="web/assets/logo.png" alt="NantaraVM Logo" width="180"/>

# NantaraVM 🇮🇩

**Cloud-Native MicroVM & Workstation Hypervisor (Open Source)**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-v1.0%20Verified-brightgreen.svg)]()
[![Platform](https://img.shields.io/badge/platform-Linux%20KVM%20%7C%20Windows-green.svg)]()
[![Website](https://img.shields.io/badge/website-nantara.cloud-cyan.svg)](https://nantara.cloud)
[![Hosting](https://img.shields.io/badge/hosting-Oracle%20Cloud%20Always%20Free-red.svg?logo=oracle)](https://sg.nantara.cloud)
[![Nodes](https://img.shields.io/badge/nodes-Singapore%20%7C%20Jakarta-blue.svg)](https://nantara.cloud)

*Membangun hypervisor dan fondasi cloud sovereign NKRI — Cepat, Aman, Berdaulat.*

</div>

---

> 🚀 **Arsitektur Hybrid (Cloud Control Plane + Local Compute Node):**  
> Web Portal & Dashboard Arsitektur NantaraVM aktif di cloud sovereign ([nantara.cloud](https://nantara.cloud) & node Oracle Singapore [sg.nantara.cloud](https://sg.nantara.cloud)). Untuk sistem operasi berat seperti **Windows 10 & Windows 11**, komputasi difokuskan **berjalan di laptop/komputer lokal pengguna** dengan syarat virtualisasi hardware (Intel VT-x / AMD-V & KVM/Hyper-V) aktif di lokal. Pendekatan lokal ini menjamin **100% GRATIS tanpa biaya sewa**, performa native tanpa lag, dan menghindari klaim "Cloud 24 GB Gratis" yang pada kenyataannya tidak gratis (sering *out-of-capacity* dan mewajibkan kartu kredit berbayar PAYG).


---

> ✅ **Status: Real KVM Hardware Virtualization & Verified Engine**  
> NantaraVM mengendalikan `/dev/kvm` secara langsung di Linux/WSL2 dengan **Seccomp-BPF**, **Landlock LSM**, **VirtIO Block/Net**, **REST API Port 8080**, **Snapshot Engine**, dan **NantaraVM Workstation Manager GUI** (VMware & VirtualBox Style).

---

## 🎯 Visi & Keunggulan

NantaraVM dirancang untuk menjadi **Hypervisor & Workstation Manager berbasis Rust** yang:

- ⚡ **Cepat** — Cold boot < 15ms dengan PVH Direct Kernel Boot & UEFI OVMF Firmware
- 🔒 **Aman** — Multi-layer kernel isolation: Seccomp-BPF BPF program, Landlock LSM filesystem restriction, & Linux Namespaces
- 🖥️ **Interaktif** — Antarmuka **Workstation Manager GUI** bergaya VirtualBox & VMware Workstation
- 🇮🇩 **Sovereign** — Dibangun oleh engineer Indonesia untuk kebutuhan cloud NKRI
- 🔓 **Open Source** — 100% kode terbuka di bawah lisensi Apache 2.0

Terinspirasi dari [Firecracker (AWS)](https://firecracker-microvm.github.io/), [crosvm (Google)](https://crosvm.dev/), [Cloud Hypervisor (Intel)](https://github.com/cloud-hypervisor/cloud-hypervisor), dan antarmuka [VMware Workstation](https://www.vmware.com/) & [Oracle VirtualBox](https://www.virtualbox.org/).

---

## 🌐 Arsitektur Hybrid & Dual-Mode (Cloud Control Plane & Local Engine)

NantaraVM dirancang dengan arsitektur **Hybrid Cloud & Local Edge** yang cerdas, hemat biaya, dan berkinerja tinggi:

1. **☁️ Cloud Web Control Plane (`https://nantara.cloud` / `https://sg.nantara.cloud`)**:
   - Berfungsi sebagai **Pusat Monitoring & Antarmuka Kontrol Global**.
   - Menyediakan antarmuka dashboard arsitektur, web terminal, dan manajemen VM dari mana saja via browser.
2. **💻 Local Compute Node (Menjalankan Windows 10/11 di Laptop/PC Lokal)**:
   - Menjalankan beban komputasi berat (seperti sistem operasi **Windows 10, Windows 11, Kali Linux, atau Ubuntu**) langsung di prosesor laptop pengguna (Intel Core i5/i7 atau AMD Ryzen) tanpa latensi dan tanpa biaya sewa cloud!
   ```powershell
   # 1. Build & Jalankan Engine NantaraVM Lokal di Windows (PowerShell):
   cargo build --release --bin nantara-engine
   .\target\release\nantara-engine.exe

   # 2. Boot ISO Windows 10/11 Lokal via REST API (Port 8080):
   Invoke-RestMethod -Uri "http://127.0.0.1:8080/api/v1/vm/start" -Method POST -ContentType "application/json" -Body '{"name":"win10-workstation","iso":"C:/win10.iso","ram":4096,"vcpu":4}'
   ```

> [!IMPORTANT]
> ### 💡 Mengapa Menjalankan Windows 10/11 di Lokal Jauh Lebih Unggul daripada Cloud?
> * **Beban Memori Windows 10/11:** Windows 10/11 membutuhkan minimal 4 GB – 8 GB RAM dan 20 GB disk hanya untuk berjalan lancar. Memaksakan Windows di cloud VPS murah (1–2 GB RAM) dipastikan akan *hang/crash (Out Of Memory)*.
> * **Fakta "Cloud 24 GB Gratis":** Janji cloud gratis 24 GB (seperti Oracle Ampere ARM) di kenyataannya sering terkendala *Out of host capacity*, mewajibkan verifikasi kartu kredit berbayar (*Pay-As-You-Go*) yang berisiko tagihan tak terduga, dan menggunakan prosesor ARM64 yang tidak cocok untuk ISO Windows x86.
> * **Zero Cost & Native Speed:** Dengan menjalankan engine NantaraVM secara lokal di laptop Anda, Anda mendapatkan akselerasi hardware penuh (**Intel VT-x / AMD-V / Hyper-V / WSL2**) dengan **biaya Rp 0 (100% Gratis Selamanya)** tanpa takut tagihan cloud. Tampilannya tetap dapat dipantau dan dikontrol melalui dashboard [nantara.cloud](https://nantara.cloud)!

---


## 💻 1-Click Installation Guide

### 🪟 Di Windows (1-Click Double-Click Installer)
1. Unduh repositori atau jalankan installer otomatis dari PowerShell:
   ```powershell
   iwr -useb https://raw.githubusercontent.com/camanit/nantara-vm/main/web/install.ps1 | iex
   ```
2. Atau double-click file **`web/install.bat`**.
3. Installer akan otomatis membuat shortcut **`NantaraVM Workstation`** di Desktop Windows Anda!

### 🐧 Di Linux / WSL2 (1-Click Shell Script)
Jalankan satu perintah di terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/camanit/nantara-vm/main/web/install.sh | sh
```

---

## 📊 Status Komponen Engine (Real & Verified)

| Komponen | Status | Keterangan |
|---|---|---|
| KVM Core `/dev/kvm` | ✅ Verified Real | Integrasi `kvm-ioctls` & vCPU 64-bit Long Mode |
| Guest RAM MMap | ✅ Verified Real | `vm-memory` GuestPhysicalMemory 16MB - 2GB |
| VirtIO Block Drive (`--drive`) | ✅ Verified Real | Disk image sector parser & Virtqueue I/O |
| VirtIO Network TAP (`--net`) | ✅ Verified Real | TAP device binding & MAC address allocation |
| REST API Management (Port 8080) | ✅ Verified Real | HTTP TCP Listener (`/api/v1/status`, `/api/v1/vm/start`, `/api/v1/vm/stop`) |
| Seccomp-BPF Filter (`--jail`) | ✅ Verified Real | BPF Program via `seccompiler` + `prctl(PR_SET_SECCOMP)` (70+ syscalls allowlist) |
| Landlock LSM Isolation | ✅ Verified Real | Kernel syscall `landlock_create_ruleset` + `landlock_restrict_self` |
| Snapshot & Lazy Restore Engine | ✅ Verified Real | Binary file I/O `NANTSNAP` format, page-by-page save/restore |
| Workstation Manager GUI | ✅ Verified Real | Web Dashboard VMware / VirtualBox Style Wizard (`nantara.cloud/dashboard.html`) |
| AMD SEV-SNP & Intel TDX | ⚙️ Kode Siap | Kode modul di `src/sev/snp.rs` (membutuhkan hardware CPU AMD EPYC/Intel TDX) |
| macOS Support (`Hypervisor.framework`) | 🎯 Roadmap v2.0 | Native Apple Silicon (M1/M2/M3/M4) & Intel Mac virtualization backend |
| Android Support (pKVM / AVF) | 🎯 Roadmap v2.0 | Android 13+ Protected KVM MicroVM enclaves di HP/Tablet Android |
| Kubernetes Containerd Shim | 🎯 Roadmap v2.0 | Native v2 containerd shim untuk pod isolation di Kubernetes |

---

## 🏗️ Arsitektur Sistem

```
┌─────────────────────────────────────────────────────────────────┐
│              Host System (Linux KVM / Windows WSL2)             │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │    NantaraVM Workstation Manager (GUI & REST API 8080)    │  │
│  │   Dashboard / CLI / Seccomp BPF / Landlock LSM / REST API │  │
│  └──────────────┬────────────────────────────────────────────┘  │
│                 │                                               │
│  ┌──────────────┼────────────────────────────────────────────┐  │
│  │    Sandboxed VirtIO Drivers (Jailer Process Bus)         │  │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────────┐  │  │
│  │  │ VirtIO Block │ │ VirtIO Net   │ │ VirtIO-GPU 1024x768│  │  │
│  │  │  (--drive)   │ │   (--net)    │ │  (--display)     │  │  │
│  │  └──────────────┘ └──────────────┘ └──────────────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                         │                                       │
│  ┌──────────────────────▼────────────────────────────────────┐  │
│  │         MicroVM Guest Hardware Space (KVM vCPU)           │  │
│  │    UEFI OVMF Firmware / Direct Boot → Windows & Linux OS  │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🚀 Build dari Source (Untuk Developer)

### Prasyarat
- Linux (Ubuntu 22.04+ / Debian 12+ / WSL2)
- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- KVM Enabled (`ls /dev/kvm`)

### Perintah Build & Run
```bash
# Clone repository
git clone https://github.com/camanit/nantara-vm.git
cd nantara-vm

# Check & Build Release
cargo check
cargo build --release

# Jalankan NantaraVM dengan KVM + Sandbox Jail
sudo ./target/release/nantara-vm --jail
```

---

## 📁 Struktur Kode Repository

```
nantara-vm/
├── src/
│   ├── main.rs          # CLI Entry point & argument parser
│   ├── vmm.rs           # VMM Core Orchestrator (/dev/kvm & GuestRAM)
│   ├── arch/x86_64/     # vCPU 64-bit Long Mode, CR0/CR3/CR4, GDT, Page Tables
│   ├── boot/            # UEFI OVMF Firmware & PVH Kernel Loader
│   ├── virtio/          # VirtIO MMIO Bus (blk, net, gpu, vsock)
│   ├── jailer/          # Real Seccomp-BPF & Landlock LSM isolation
│   ├── userfaultfd.rs   # Real Snapshot & Restore Engine (NANTSNAP binary I/O)
│   ├── net/             # eBPF & XDP Network Filter Engine
│   ├── sev/             # AMD SEV-SNP & Intel TDX Confidential Computing
│   ├── api/             # Real REST API Server (Port 8080) & VNC Streamer (5900)
│   └── license/         # License Verification (Community & Enterprise Pro)
├── web/                 # Web Dashboard & Installers (nantara.cloud)
│   ├── index.html       # Official Landing Page
│   ├── dashboard.html   # NantaraVM Workstation Manager GUI (VMware/VirtualBox Style)
│   ├── install.bat      # Windows Double-Click Installer
│   ├── install.ps1      # Windows PowerShell 1-Click Installer
│   └── install.sh       # Linux Shell 1-Click Installer
├── tools/               # License Generator Utility (gen_license.rs)
└── Cargo.toml
```

---

## ☕ Dukung & Donasi Pengembang (Support Open Source)

Jika Anda ingin mendukung keberlanjutan pengembangan **NantaraVM** (Hypervisor MicroVM Open-Source Karya Indonesia), Anda dapat memberikan apresiasi / donasi melalui:

- 🏦 **Bank:** Allo Bank
- 💳 **No. Rekening:** `081260006666`
- 💬 **Konfirmasi / WA:** [+62 812-6000-6666](https://wa.me/6281260006666)

*Dukungan Anda sangat berharga untuk biaya infrastruktur server pengujian, lisensi hardware AMD/Intel enclave, serta pengembangan fitur-fitur baru NantaraVM.*

## ⚖️ Lisensi & Hak Cipta (Dual-Licensing Model)

**NantaraVM** dilindungi oleh lisensi **Commercial Fair-Source / Dual-Licensing**:

1. **Penggunaan Non-Komersial / Komunitas (GRATIS):**
   - Bebas digunakan, dipelajari, dan dikembangkan untuk penggunaan pribadi, edukasi, akademis, dan riset non-profit.
2. **Penggunaan Komersial & Perusahaan (WAJIB LISENSI BERBAYAR):**
   - **Dilarang keras** bagi perusahaan, vendor IT, atau pihak ketiga mana pun untuk memperjualbelikan, mendistribusikan ulang untuk tujuan komersial, atau meng-host NantaraVM sebagai layanan berbayar (*Commercial SaaS / Cloud Provider*) **tanpa izin tertulis dan Lisensi Komersial resmi dari nantara.cloud / Pengembang**.
   - Untuk pembelian Lisensi Komersial Enterprise, hak jual kembali (reseller), atau integrasi sistem, hubungi:
     - 🌐 **Website:** [nantara.cloud](https://nantara.cloud)
     - 📧 **Email:** `dev@nantara.cloud`
     - 💬 **WhatsApp SLA Support:** [+62 812-6000-6666](https://wa.me/6281260006666)

---

## 📬 Kontak & Kontribusi

- 🌐 **Website:** [nantara.cloud](https://nantara.cloud)
- 💬 **WhatsApp:** [+62 812-6000-6666](https://wa.me/6281260006666)
- 🐛 **Issue Tracker:** [GitHub Issues](https://github.com/camanit/nantara-vm/issues)

---

<div align="center">
   
**Fullscreen:**

<img width="60%" height="auto" alt="WhatsApp Image 2026-08-06 at 00 38 37 (2)" src="https://github.com/user-attachments/assets/16c64223-eeac-48cf-b718-4ca98dccafdf" />

-

**Settings Local Scaling:**

<img width="60%" height="auto" alt="WhatsApp Image 2026-08-06 at 00 35 00" src="https://github.com/user-attachments/assets/976521b6-5a2c-43b6-bc39-8ed4c16ba3e8" />

-

**Boot & Start:**

<img width="60%" height="auto" alt="WhatsApp Image 2026-08-06 at 00 38 37" src="https://github.com/user-attachments/assets/eed59e8d-5527-4a0f-b0da-ba8c3fa432be" />

-

**Normal Layout:**

<img width="60%" height="auto" alt="WhatsApp Image 2026-08-06 at 00 38 37 (1)" src="https://github.com/user-attachments/assets/adf55159-e296-4a65-aad4-d78d9430e02d" />

-

**Open Browser:**

<img width="60%" height="auto" alt="WhatsApp Image 2026-08-06 at 00 53 34" src="https://github.com/user-attachments/assets/f20897b1-70dd-4df7-8e2d-2ece20d83fa7" />


-

**Infrastruktur Web Cloud Sovereign:**  
*(Web interface, dashboard control plane, real web bash terminal `ttyd`, dan container workstation resmi di-deploy pada **Oracle Cloud Infrastructure Always Free (Node Singapore `sg.nantara.cloud`)** dan **IDCloudHost**, menjamin layanan selalu aktif 24/7 dengan arsitektur cloud sovereign).*



<img width="60%" height="auto" alt="WhatsApp Image 2026-08-06 at 10 58 54" src="https://github.com/user-attachments/assets/885f84f9-3b0e-4c4f-af54-465ab54e129e" />

</div>
---
<div align="center">

Dibangun dengan ❤️ oleh engineer Indonesia 🇮🇩

*"Membangun teknologi sovereign berbasis KVM sejati — Berdaulat & Terbukti."*

</div>
