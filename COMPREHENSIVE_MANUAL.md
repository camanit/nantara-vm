# 📚 NantaraVM NKRI 2026 - Master Manual & Panduan Lengkap Pengguna

Selamat datang di Buku Panduan Utama **NantaraVM NKRI 2026**. Dokumen ini merangkum seluruh arsitektur, panduan instalasi, konsep operasional, hingga strategi komersialisasi NantaraVM.

---

## 📑 Daftar Isi
1. [Bab 1: Pengenalan & Arsitektur NantaraVM](#bab-1-pengenalan--arsitektur-nantaravm)
2. [Bab 2: Panduan Instalasi & Eksekusi (Linux, Windows & WSL2)](#bab-2-panduan-instalasi--eksekusi)
3. [Bab 3: Perbandingan VMware Workstation vs NantaraVM MicroVM](#bab-3-perbandingan-vmware-vs-nantaravm)
4. [Bab 4: Cara Menjalankan OS Linux & Windows 10/11](#bab-4-cara-menjalankan-os-linux--windows-1011)
5. [Bab 5: Tampilan GUI VMware ESXi Dashboard & noVNC](#bab-5-tampilan-gui-vmware-esxi-dashboard--novnc)
6. [Bab 6: Keamanan Multi-Tenancy & Isolasi Data Klien](#bab-6-keamanan-multi-tenancy--isolasi-data-klien)
7. [Bab 7: SOP Lisensi Enterprise Pro & Template WhatsApp Sales](#bab-7-sop-lisensi-enterprise-pro--template-whatsapp)

---

## Bab 1: Pengenalan & Arsitektur NantaraVM

NantaraVM NKRI 2026 adalah **Sovereign Cloud-Native MicroVM Hypervisor** berbasis bahasa pemrograman Rust safe-memory yang menggabungkan keunggulan:
* **Firecracker (AWS)**: Cold-boot instan $< 10\text{ ms}$ dan footprint memori idle amat hemat ($3.8\text{ MB}$).
* **Crosvm (Google)**: Isolasi sandboxing perangkat VirtIO multi-proses (Linux Namespaces, UID/GID 65534, Seccomp BPF, & Landlock LSM).
* **Cloud Hypervisor (Intel)**: ACPI 6.3, eBPF Zero-Trust Packet Filter, XDP Line-Rate Driver (9.4 Gbps), dan Enkripsi Memori Hardware AMD SEV-SNP / Intel TDX (AES-256).

---

## Bab 2: Panduan Instalasi & Eksekusi

### A. Instalasi di Linux Server (Rekomendasi Host KVM Production):
```bash
curl -fsSL https://nantara.cloud/install.sh | sh
```
Atau kompilasi dari source via Rust Cargo:
```bash
cargo install nantara-vm
```

### B. Panduan Menyalakan WSL2 KVM di Komputer Windows (Khusus Pengguna Windows):
Untuk mendapatkan dukungan kernel KVM `/dev/kvm` asli di Windows:
1. Buka PowerShell (Run as Administrator), lalu ketik:
   ```powershell
   wsl --install
   ```
2. Restart komputer jika diminta oleh Windows.
3. Buka terminal WSL2 Ubuntu, lalu jalankan NantaraVM dengan akomodasi `/dev/kvm`:
   ```bash
   cd /mnt/c/Users/UseR/Documents/NantaraVM
   ./target/debug/nantara-vm --iso "/mnt/c/Users/UseR/Documents/os win/Win10_22H2_EnglishInternational_x32v1.iso"
   ```

### C. Eksekusi Cepat Windows Double-Click Launcher:
1. Unduh paket `nantara-vm-windows.zip` dari `https://nantara.cloud/downloads/nantara-vm-windows.zip`.
2. Ekstrak folder zip tersebut.
3. **Double-Click** file `run-nantara-vm.bat` atau `nantara-vm.exe`. Jendela terminal akan otomatis mendeteksi ISO Windows 10 dan membuka **VMware ESXi Dashboard Web App** di browser Anda.

---

## Bab 3: Perbandingan VMware Workstation vs NantaraVM

| Fitur | 💻 VMware Workstation | 🚀 NantaraVM MicroVM |
| :--- | :--- | :--- |
| **Media Booting** | Menggunakan File ISO Installer (`.iso`). | Menggunakan **Kernel (`vmlinux`)** + **Root Disk (`rootfs.ext4`)**. |
| **Proses Instalasi** | Manual klik-klik wizard (20-30 menit). | **TIDAK PERLU INSTALASI!** OS langsung siap pakai instan. |
| **Kecepatan Boot** | 15 - 30 Detik. | **11.2 Milidetik** ($< 0.01\text{ detik}$). |
| **Penggunaan Memori** | 2.000 MB - 4.000 MB RAM per VM. | **3.8 MB RAM** per MicroVM. |
| **Keamanan Memori** | Standard (Unencrypted). | **Hardware Memory Encryption (AMD SEV-SNP AES-256)**. |

---

## Bab 4: Cara Menjalankan OS Linux & Windows 10/11

### A. Menjalankan OS Linux (Ubuntu / Alpine / Debian):
Persiapkan berkas kernel `vmlinux` dan disk image `rootfs.ext4`, lalu jalankan:
```bash
./nantara-vm --kernel vmlinux --rootfs rootfs.ext4
```

### B. Menjalankan OS Windows 10 / Windows 11:
Persiapkan disk image Windows (`windows11.qcow2` / `windows10.raw` / `.iso`) dan Driver Red Hat VirtIO (`virtio-win`):
```bash
./nantara-vm --iso "Win10_22H2.iso" --ram 2048 --cpus 2 --display virtio-gpu
```
Tampilan Desktop Windows 10/11 akan langsung tayang di noVNC Web Console (`dashboard.html`).

---

## Bab 5: Tampilan GUI VMware ESXi Dashboard & noVNC

NantaraVM menyediakan antarmuka GUI modern berbasis Web UI di **`https://nantara.cloud/dashboard.html`**:
* **📺 noVNC Graphical Console**: Layar monitor grafik virtual (`virtio-gpu`) untuk berinteraksi dengan OS Desktop (input mouse & keyboard).
* **📊 Live ESXi Meters**: Grafik real-time CPU %, RAM MB, VirtIO Net throughput (Gbps), dan status enkripsi AMD SEV-SNP.
* **⚙️ Control Buttons**: Tombol interaktif **Start**, **Pause**, **Resume**, dan **Snapshot** ($< 4.2\text{ ms}$).

---

## Bab 6: Keamanan Multi-Tenancy & Isolasi Data Klien

Setiap instalasi NantaraVM bersifat **Self-Hosted Local Engine**:
* **Data 100% Terisolasi**: Pengguna Klien A di komputernya sendiri HANYA BISA melihat data server miliknya sendiri. Data pengguna lain **TIDAK AKAN BISA TERLIHAT ATAU BOCOR**.
* **Proteksi Lisensi**: Pengguna versi gratis tidak dapat membuat lisensi Pro sendiri; penerbitan lisensi resmi komersial dikunci penuh di bawah wewenang Admin via Terminal (`cargo run --bin gen_license`).

---

## Bab 7: SOP Lisensi Enterprise Pro & Template WhatsApp Sales

### A. Perintah Penerbitan Lisensi via Terminal Admin:
```bash
cargo run --bin gen_license
```

### B. Template Chat WhatsApp untuk Klien (+62 812-6000-6666):
> Yth. Tim IT **[NAMA_INSTANSI]**,
> 
> Terima kasih telah berlangganan **NantaraVM Enterprise Pro**!
> 
> 🔑 **LICENSE KEY**: `NANTARA-PRO-2026-[KODE_KLIEN]-[HASH]`
> 
> **Cara Mengaktifkan di Server Linux**:
> ```bash
> export NANTARA_PRO_LICENSE="NANTARA-PRO-2026-[KODE_KLIEN]-[HASH]"
> ./nantara-vm
> ```
> 
> Salam hangat,  
> **Tim Nantara Cloud Platform** (`nantara.cloud`)

---

&copy; 2026 NantaraVM NKRI (nantara.cloud). All Rights Reserved.
