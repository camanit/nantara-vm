<div align="center">

<img src="web/assets/logo.png" alt="NantaraVM Logo" width="180"/>

# NantaraVM 🇮🇩

**MicroVM Hypervisor Open-Source Karya Indonesia**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-Early%20Alpha-yellow.svg)]()
[![Platform](https://img.shields.io/badge/platform-Linux%20%2F%20KVM-green.svg)]()
[![Website](https://img.shields.io/badge/website-nantara.cloud-cyan.svg)](https://nantara.cloud)

*Membangun fondasi cloud sovereign NKRI — satu commit dalam satu waktu.*

</div>

---

> ⚠️ **Status: Early Alpha / Proof-of-Concept**
> Proyek ini sedang aktif dikembangkan. Arsitektur dan fondasi kode sudah ada, namun banyak fitur masih dalam tahap implementasi. Lihat [Status Pengembangan](#-status-pengembangan) untuk detail jujur per komponen.

---

## 🎯 Visi

NantaraVM bertujuan membangun **MicroVM Hypervisor berbasis Rust** yang:

- ⚡ **Cepat** — Target cold boot < 15ms dengan PVH Direct Kernel Boot
- 🔒 **Aman** — Multi-layer isolation: seccomp BPF, Linux namespaces, Landlock LSM
- 🇮🇩 **Sovereign** — Dibangun oleh engineer Indonesia, untuk kebutuhan cloud NKRI
- 🔓 **Open Source** — Kode terbuka, dikembangkan bersama komunitas

Terinspirasi dari [Firecracker (AWS)](https://firecracker-microvm.github.io/), [crosvm (Google)](https://crosvm.dev/), dan [Cloud Hypervisor (Intel)](https://github.com/cloud-hypervisor/cloud-hypervisor).

---

## 🏗️ Arsitektur

```
┌─────────────────────────────────────────────────┐
│           Host Linux System (KVM)               │
│                                                 │
│  ┌──────────────────────────────────────────┐   │
│  │     NantaraVM Control Plane (Rust)       │   │
│  │  CLI / REST API │ KVM ioctl │ vm-memory  │   │
│  └──────────┬───────────────────────────────┘   │
│             │                                   │
│  ┌──────────┼──────────────────────────────┐    │
│  │  Sandboxed Device Processes (Jailer)    │    │
│  │  ┌─────────┐ ┌─────────┐ ┌──────────┐  │    │
│  │  │virtio-  │ │virtio-  │ │virtio-   │  │    │
│  │  │  blk    │ │  net    │ │  vsock   │  │    │
│  │  └─────────┘ └─────────┘ └──────────┘  │    │
│  └──────────────────────────────────────────┘   │
│                      │                          │
│  ┌───────────────────▼──────────────────────┐   │
│  │        MicroVM Guest Space               │   │
│  │   PVH Boot → vmlinux → Guest Workload    │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

---

## 📊 Status Pengembangan

### ✅ Sudah Ada (Fase 0 — Fondasi)

| Komponen | Status | Keterangan |
|---|---|---|
| Struktur proyek Rust | ✅ Selesai | Cargo workspace, modul terorganisir |
| Integrasi `kvm-ioctls` | ✅ Selesai | Library KVM tersambung |
| Integrasi `vm-memory` | ✅ Selesai | Guest RAM allocation |
| Desain VirtIO MMIO bus | ✅ Selesai | Trait interface blk/net/vsock |
| Struktur Jailer | ✅ Selesai | Desain multi-process isolation |
| Website & dokumentasi | ✅ Selesai | [nantara.cloud](https://nantara.cloud) |

### 🔨 Sedang Dikerjakan (Fase 1 — Boot Linux)

| Komponen | Status | Keterangan |
|---|---|---|
| vCPU run loop via KVM | 🔨 Aktif | Butuh Linux + `/dev/kvm` |
| Boot kernel Linux sederhana | 🔨 Aktif | Target: Alpine/BusyBox |
| VirtIO queue logic | 🔨 Aktif | Block device operasional |
| Serial console output | 🔨 Aktif | `ttyS0` → terminal |

### 📋 Direncanakan

| Komponen | Target Fase | Keterangan |
|---|---|---|
| VirtIO network (TAP) | Fase 2 | Guest internet access |
| REST API via TCP | Fase 2 | Dashboard web terhubung |
| Binary release Linux | Fase 2 | Siap didownload & dijalankan |
| Boot Windows 10 | Fase 3+ | Butuh UEFI/OVMF emulation |
| AMD SEV-SNP nyata | Jangka panjang | Butuh hardware AMD EPYC |
| eBPF / XDP | Jangka panjang | Saat ini hanya placeholder |

---

## 🚀 Quickstart (Build dari Source)

### Prasyarat

- Linux (Ubuntu 22.04+ / Debian 12+ direkomendasikan)
- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- `/dev/kvm` tersedia (`ls /dev/kvm` — perlu KVM-enabled CPU & kernel)

### Build

```bash
# Clone repository
git clone https://github.com/camanit/nantara-vm.git
cd nantara-vm

# Build
cargo build

# Jalankan (demo/stub mode jika tanpa vmlinux)
cargo run
```

### Di Windows (via WSL2)

```bash
# 1. Aktifkan WSL2 + Ubuntu
wsl --install

# 2. Di dalam WSL2, ikuti langkah Linux di atas
# Catatan: /dev/kvm membutuhkan virtualisasi aktif di BIOS/UEFI
```

> 💡 **Catatan:** Fitur KVM penuh hanya berjalan di Linux. Di sistem lain, hanya demo/stub mode yang aktif.

---

## 🤝 Berkontribusi

Proyek ini membutuhkan kontributor! Tidak perlu ahli untuk memulai.

### Area yang Paling Dibutuhkan

- 🦀 **Rust Dev** — Implementasi vCPU run loop, VirtIO queue, TCP API server
- 🐧 **Linux Kernel Dev** — KVM ioctl, eBPF, namespaces
- 📝 **Technical Writer** — Dokumentasi, tutorial, panduan kontribusi
- 🧪 **Tester** — Build dari source, laporan bug, pengujian di berbagai distro

### Cara Berkontribusi

```bash
# Fork & clone
git clone https://github.com/camanit/nantara-vm.git
cd nantara-vm

# Buat branch baru
git checkout -b feature/nama-fitur-anda

# Buat perubahan, lalu commit
git commit -m "feat: deskripsi perubahan"

# Push dan buka Pull Request
git push origin feature/nama-fitur-anda
```

Lihat [Issues](https://github.com/camanit/nantara-vm/issues) untuk daftar task yang tersedia.

---

## 📁 Struktur Proyek

```
nantara-vm/
├── src/
│   ├── main.rs          # Entry point & CLI argument parsing
│   ├── vmm.rs           # VMM orchestrator utama
│   ├── arch/x86_64/     # CPU setup: Long Mode, GDT, Page Tables
│   ├── boot/            # PVH kernel loader & Zero Page setup
│   ├── virtio/          # VirtIO MMIO bus, blk, net, vsock
│   ├── jailer/          # Process isolation & IPC
│   ├── net/             # eBPF filter (placeholder)
│   ├── sev/             # AMD SEV-SNP (placeholder)
│   ├── api/             # REST API server
│   ├── acpi/            # ACPI table generation
│   └── bench/           # Performance benchmarks
├── web/                 # Website nantara.cloud
│   ├── index.html
│   ├── dashboard.html
│   └── docs.html
├── tools/               # Utility tools
└── Cargo.toml
```

---

## 🗺️ Roadmap 2025–2026

```
2025 Q3  ████████░░░░  Fase 0: Fondasi (✅ Selesai)
2025 Q4  ░░░░░░░░░░░░  Fase 1: Boot Linux sederhana (🔨 Aktif)
2026 Q1  ░░░░░░░░░░░░  Fase 2: Network + REST API + Binary release
2026 Q2  ░░░░░░░░░░░░  Fase 3: Jailer nyata + Snapshot dasar
2026 Q3  ░░░░░░░░░░░░  Fase 4: eBPF/XDP + Containerd shim
2026 Q4  ░░░░░░░░░░░░  Fase 5+: SEV-SNP, Windows support (jangka panjang)
```

---

## 📄 Lisensi

Apache License 2.0 — Lihat [LICENSE](LICENSE) untuk detail.

---

## 📬 Kontak

- 🌐 **Website:** [nantara.cloud](https://nantara.cloud)
- 💬 **WhatsApp:** [+62 812-6000-6666](https://wa.me/6281260006666)
- 🐛 **Bug Reports:** [GitHub Issues](https://github.com/camanit/nantara-vm/issues)

---

<div align="center">

Dibangun dengan ❤️ oleh engineer Indonesia 🇮🇩

*"Membangun teknologi sovereign bukan sehari jadi — tapi setiap langkah berarti."*

</div>
