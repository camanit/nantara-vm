# 📖 NantaraVM NKRI 2026 - Comprehensive User Guide & Architecture Manual

Welcome to the official User Guide for **NantaraVM NKRI 2026**, Indonesia's Next-Generation Sovereign Cloud-Native MicroVM Hypervisor built in Rust.

---

## 📑 Daftar Isi
1. [Overview & Arsitektur](#1-overview--arsitektur)
2. [Instalasi & Persyaratan Sistem](#2-instalasi--persyaratan-sistem)
3. [Panduan Penggunaan CLI](#3-panduan-penggunaan-cli)
4. [Aktivasi Lisensi Enterprise Pro](#4-aktivasi-lisensi-enterprise-pro)
5. [Manajemen REST API & Unix Socket](#5-manajemen-rest-api--unix-socket)
6. [Opsi Tampilan GUI Ala VMware (VirtIO-GPU, noVNC & ESXi Dashboard)](#6-opsi-tampilan-gui-ala-vmware)

---

## 1. Overview & Arsitektur

NantaraVM menggabungkan keunggulan tiga hypervisor terkemuka dunia:
* **Firecracker**: Kecepatan *cold boot* sub-10ms dan penghematan memori RAM ($< 5\text{ MB}$).
* **Crosvm**: Isolasi sandboxing perangkat VirtIO multi-proses (Linux Namespaces, UID/GID 65534, Seccomp BPF, & Landlock LSM).
* **Cloud Hypervisor**: Fitur cloud modern seperti ACPI 6.3, eBPF Zero-Trust Filter, XDP Line-Rate Driver, dan AMD SEV-SNP / Intel TDX Hardware Memory Encryption.

---

## 2. Instalasi & Persyaratan Sistem

### Persyaratan Sistem (Host Node):
* **OS**: Linux Kernel 5.13+ (Disarankan Ubuntu 22.04 LTS / Debian 12 / RHEL 9) atau Windows (Stub Mode).
* **CPU**: x86_64 dengan dukungan KVM (`/dev/kvm`). Dukungan AMD SEV-SNP atau Intel TDX opsional untuk Mode Enterprise Pro.
* **RAM**: Minimal 512 MB pada Host.

### Cara Instalasi Instan (Linux):
```bash
curl -fsSL https://nantara.cloud/install.sh | sh
```

### Cara Instalasi Windows:
Unduh paket ZIP dari `https://nantara.cloud/downloads/nantara-vm-windows.zip`, ekstrak, lalu jalankan `run-nantara-vm.bat` atau `nantara-vm.exe`.

---

## 3. Panduan Penggunaan CLI

### Jalankan NantaraVM Mode Standar (Community Edition):
```bash
./nantara-vm
```

### Output Terminal Eksekusi:
```text
====================================================
 🚀 NantaraVM NKRI 2026 - Cloud-Native MicroVM VMM
====================================================
[NantaraVM] Allocating 16 MB Guest Physical RAM...
[NantaraVM License] Mode: NantaraVM Community Edition (Free Open Source).
[NantaraVM Landlock] Installing Linux Kernel 5.13+ Landlock LSM rules...
[NantaraVM Boot] Direct Kernel Boot configured. RIP = 0x100000 (< 10 ms latency)
[NantaraVM Serial COM1] NantaraVM NKRI 2026 Booted!
```

---

## 4. Aktivasi Lisensi Enterprise Pro

Lisensi Enterprise Pro mengunci fitur keamanan Confidential Computing (AMD SEV-SNP / Intel TDX) dan XDP Line-Rate NIC mode.

### Cara Mengaktifkan Lisensi:
1. Pasang Environment Variable sebelum menjalankan NantaraVM:
   ```bash
   export NANTARA_PRO_LICENSE="NANTARA-PRO-2026-KEMENTERIAN-KOMINFO-8F3A2B10"
   ./nantara-vm
   ```
2. Atau tempatkan file lisensi di `/etc/nantara/license.key`:
   ```bash
   echo "NANTARA-PRO-2026-KEMENTERIAN-KOMINFO-8F3A2B10" > /etc/nantara/license.key
   ./nantara-vm
   ```

---

## 5. Manajemen REST API & Unix Socket

NantaraVM menyediakan REST API bawaan yang berjalan di Unix Domain Socket `/tmp/nantara_api.sock`:

### Endpoint API yang Tersedia:
| HTTP Method | Endpoint | Fungsi |
| :---: | :--- | :--- |
| `GET` | `/api/v1/vm/info` | Mendapatkan status statistik MicroVM (vCPU, RAM, Uptime). |
| `POST` | `/api/v1/vm/pause` | Menjeda (*pause*) eksekusi vCPU MicroVM. |
| `POST` | `/api/v1/vm/resume` | Melanjutkan (*resume*) eksekusi MicroVM. |
| `POST` | `/api/v1/vm/snapshot` | Membuat state snapshot instant MicroVM. |

### Contoh Pengujian via `curl`:
```bash
curl --unix-socket /tmp/nantara_api.sock http://localhost/api/v1/vm/info
```

---

## 6. Opsi Tampilan GUI Ala VMware

Untuk memenuhi kebutuhan pengguna yang terbiasa dengan antarmuka **VMware Workstation / ESXi Dashboard**, NantaraVM mendukung 3 pendekatan visual:

### 🎨 A. VMware Console Window (`virtio-gpu` + VNC / noVNC Web Console)
Jika Anda ingin melihat tampilan layar Desktop OS Guest (seperti Windows 11 atau Ubuntu Desktop):
1. **Emulasi VirtIO-GPU**: NantaraVM menyediakan modul emulasi `virtio-gpu` untuk mengekspor framebuffer tampilan grafik VM.
2. **Embedded VNC Server**: Framebuffer diteruskan ke VNC Server di Host.
3. **noVNC Web Interface**: Pengguna membuka browser dan melihat layar monitor virtual VM secara real-time lengkap dengan input mouse & keyboard.

### 🏢 B. Dashboard Management VMware ESXi / vCenter Style (Web Control Plane)
Tampilan Dashboard Management berbasis Web UI untuk membuat, menghentikan, membuat snapshot, serta memantau grafik CPU & RAM secara visual.
* **Frontend**: SPA HTML5/JS / React dengan WebSockets untuk statistik real-time.
* **Backend**: Terhubung langsung ke REST API `/tmp/nantara_api.sock`.

### 🔌 C. Integrasi Orchestrator Kubernetes / Kata Containers
Melalui bridge `containerd-shim-vmm-v2`, NantaraVM terintegrasi penuh ke Kubernetes Dashboard dan OpenShift Virtualization.

---

&copy; 2026 NantaraVM NKRI (nantara.cloud). All Rights Reserved.
