# 📖 CATATAN PENGAKTIFAN SERVER LINUX & KOMPONEN WEB INTERAKTIF
### Dokumentasi Resmi NantaraVM — Cloud VPS & Infrastructure Guide
> *Simpan catatan ini agar tidak perlu mencari-cari lagi saat setup server baru.*

---

## 🎯 1. Memahami Komponen & Fungsinya (Mengapa Butuh ttyd, noVNC, & Nginx?)

Saat kita membuka website `nantara.cloud`, ada 3 teknologi utama yang bekerja bersama di server VPS:

```
                  ┌─────────────── BROWSER CLIENT ───────────────┐
                  │   https://nantara.cloud/dashboard.html       │
                  └──────────────────────┬───────────────────────┘
                                         │
                                   Port 80 / 443
                                         ▼
                  ┌─────────────── NGINX REVERSE PROXY ───────────┐
                  │ Lokasi: /etc/nginx/sites-available/default   │
                  └──────┬───────────────────────┬───────────────┘
                         │                       │
           Path: /console/                       Path: /desktop/
                         │                       │
                         ▼                       ▼
            ┌─────────────────────────┐   ┌───────────────────────────┐
            │   ttyd (Web Terminal)   │   │  noVNC (Web GUI Desktop)  │
            │   Port: 7681            │   │  Port: 6080 (Docker)      │
            │   Fungsi: Bash Shell    │   │  Fungsi: Desktop GUI      │
            └─────────────────────────┘   └───────────────────────────┘
```

---

### A. Apa itu `ttyd`?
* **Pengertian:** `ttyd` adalah tool ringan yang membagikan terminal Linux (`bash`) ke web browser secara langsung lewat protokol WebSocket.
* **Mengapa Diperlukan?** Web browser tidak bisa langsung menjalankan perintah Linux tanpa jembatan (*bridge*). `ttyd` inilah yang menjembatani keyboard di browser sobat langsung ke prosesor Linux di VPS.
* **Port Standar:** `7681`
* **Perintah Menjalankan Manual:**
  ```bash
  ttyd -p 7681 bash
  ```

---

### B. Apa itu `noVNC`?
* **Pengertian:** `noVNC` adalah penampil desktop grafis (GUI) berbasis HTML5 murni.
* **Mengapa Diperlukan?** Tanpa noVNC, sobat harus menginstal software VNC Viewer terpisah di laptop. Dengan noVNC, tampilan desktop Linux (XFCE / Kali Linux) bisa langsung ditonton dan diklik mouse-nya langsung dari browser web.
* **Port Standar:** `6080` (atau port `80` di dalam container Docker).
* **Perintah Menjalankan (Docker):**
  ```bash
  sudo docker run -d --name nantara-desktop --restart always -p 6080:80 dorowu/ubuntu-desktop-lxde-vnc:latest
  ```

---

### C. Apa itu Nginx Reverse Proxy?
* **Pengertian:** Web server utama di VPS yang bertindak sebagai gerbang tunggal (port 80 / 443).
* **Mengapa Diperlukan?** Pengunjung tidak perlu mengetik `:7681` atau `:6080` di browser. Cukup buka:
  * `https://nantara.cloud/` ➡️ menampilkan Dashboard NantaraVM.
  * `https://nantara.cloud/console/` ➡️ otomatis disambungkan Nginx ke `ttyd` (port 7681).
  * `https://nantara.cloud/desktop/` ➡️ otomatis disambungkan Nginx ke `noVNC` (port 6080).

---

## 🚀 2. Cara Cepat Setup Server Baru (1 Perintah Otomatis)

Setelah membuat VM baru di IDCloudHost (Ubuntu 22.04 / 24.04), cukup login SSH ke VPS:
```bash
ssh caman@<IP_VPS_BARU>
```

Lalu jalankan **1 baris perintah ini** (langsung menginstal dan mengaktifkan semuanya):
```bash
curl -fsSL https://raw.githubusercontent.com/camanit/nantara-vm/main/tools/setup_server.sh | sudo bash
```

---

## 🛠️ 3. Perintah Manual Step-by-Step (Jika Ingin Dijalankan Satu Per Satu)

### Langkah 1: Update & Install Nginx + ttyd
```bash
sudo apt update -y
sudo apt install -y nginx curl git ufw wget

# Install ttyd
sudo wget -q https://github.com/tsl0922/ttyd/releases/download/1.7.7/ttyd.x86_64 -O /usr/local/bin/ttyd
sudo chmod +x /usr/local/bin/ttyd
```

### Langkah 2: Buat Service Background untuk ttyd
```bash
sudo bash -c 'cat <<EOF > /etc/systemd/system/ttyd.service
[Unit]
Description=NantaraVM Web Terminal Console (ttyd)
After=network.target

[Service]
ExecStart=/usr/local/bin/ttyd -p 7681 -t fontSize=14 bash
Restart=always
User=root
WorkingDirectory=/root

[Install]
WantedBy=multi-user.target
EOF'

sudo systemctl daemon-reload
sudo systemctl enable ttyd
sudo systemctl restart ttyd
```

### Langkah 3: Install Docker & Jalankan Desktop GUI
```bash
curl -fsSL https://get.docker.com | sudo sh
sudo systemctl enable docker
sudo systemctl start docker

# Jalankan Desktop GUI (Port 6080)
sudo docker run -d --name nantara-desktop --restart always -p 6080:80 -e USER=caman -e PASSWORD=nantara -e RESOLUTION=1280x720 dorowu/ubuntu-desktop-lxde-vnc:latest
```

### Langkah 4: Konfigurasi Nginx Reverse Proxy
Edit file `/etc/nginx/sites-available/default`:
```nginx
server {
    listen 80 default_server;
    # CATATAN PENTING: Jangan tambahkan `listen [::]:80` jika kernel VPS menonaktifkan IPv6
    # agar Nginx tidak gagal bind (Address family not supported by protocol).
    server_name nantara.cloud www.nantara.cloud _;

    root /var/www/html;
    index dashboard.html index.html;

    location / {
        try_files $uri $uri/ /dashboard.html;
    }

    # Teruskan /console/ ke ttyd (Terminal Bash WebSocket)
    location /console/ {
        proxy_pass http://127.0.0.1:7681/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_read_timeout 86400;
    }

    # Teruskan /desktop/ ke noVNC (Desktop Grafis GUI)
    location /desktop/ {
        proxy_pass http://127.0.0.1:6080/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_read_timeout 86400;
    }
}
```
Lalu uji dan restart Nginx:
```bash
sudo nginx -t
sudo systemctl restart nginx
```

### Langkah 5: Salin File Web NantaraVM
```bash
git clone --depth 1 https://github.com/camanit/nantara-vm.git /tmp/nantara-web
sudo cp -r /tmp/nantara-web/web/* /var/www/html/
sudo chown -R www-data:www-data /var/www/html
rm -rf /tmp/nantara-web
```

---

## 🔍 4. Perintah Penting untuk Monitoring & Troubleshooting

| Kebutuhan | Perintah Terminal |
|---|---|
| **Cek Port Aktif** | `sudo ss -tulpn \| grep -E ':80\|:7681\|:6080'` |
| **Cek Status Nginx** | `sudo systemctl status nginx --no-pager` |
| **Cek Status ttyd** | `sudo systemctl status ttyd --no-pager` |
| **Cek Status Docker** | `sudo docker ps` |
| **Restart Nginx** | `sudo systemctl restart nginx` |
| **Restart ttyd** | `sudo systemctl restart ttyd` |
| **Lihat Log Error Nginx** | `sudo tail -f /var/log/nginx/error.log` |
| **Lihat Log Desktop GUI**| `sudo docker logs --tail 50 nantara-desktop` |

---

## 🔒 5. Mengaktifkan SSL Gratis (HTTPS) via Certbot
Agar website aman dengan gembok hijau (`https://`):
```bash
sudo apt install -y certbot python3-certbot-nginx
sudo certbot --nginx -d nantara.cloud -d www.nantara.cloud
```
Certbot akan otomatis memperbarui konfigurasi Nginx dan memperpanjang sertifikat SSL secara berkala.

---

## 🌐 6. Mengatasi Google Chrome Tidak Bisa Dibuka di noVNC

### Mengapa Google Chrome Awalnya Tidak Bisa Terbuka?
Google Chrome memiliki sistem keamanan ketat bernama **Sandbox (Kernel Namespaces)**. Karena desktop GUI berjalan di dalam container Docker tanpa privilege root kernel host, Chrome mendeteksi hilangnya akses sandbox dan menolak terbuka (demi keamanan default Chromium).

### Solusi 1: Buka Firefox (Sudah Terpasang & Langsung Jalan)
Di dalam Desktop noVNC, buka menu aplikasi ➡️ **Internet** ➡️ **Firefox Web Browser**. Firefox tidak membutuhkan hak sandbox khusus di Docker dan langsung lancar membuka web.

### Solusi 2: Aktifkan Google Chrome dengan Flag `--no-sandbox`
Jika sobat ingin ikon Google Chrome di desktop langsung bisa diklik dan terbuka sempurna, jalankan perintah 1 baris ini di terminal SSH IDCloudHost:
```bash
sudo docker exec -u 0 nantara-desktop bash -c 'echo -e "#!/bin/bash\nexec /usr/bin/google-chrome-stable --no-sandbox \"\$@\"" > /usr/local/bin/google-chrome-stable && chmod +x /usr/local/bin/google-chrome-stable'
```
*Setelah menjalankan perintah di atas, klik ikon Google Chrome di desktop noVNC akan langsung membuka browser tanpa error sandbox lagi.*

---

## 🏛️ 7. Arsitektur Dua Edisi (Dual-Tier Architecture)

NantaraVM dirancang memiliki **2 Edisi Resmi** yang saling melengkapi:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           ARSITEKTUR DUA EDISI                              │
├──────────────────────────────────────┬──────────────────────────────────────┤
│    🌐 EDISI 1: BROWSER WASM TIER     │    ⚡ EDISI 2: DEDICATED CLOUD TIER  │
│    https://nantara-vm.vercel.app/    │    https://nantara.cloud/ (VPS)      │
├──────────────────────────────────────┼──────────────────────────────────────┤
│ • Hosting: Vercel Edge Server        │ • Hosting: IDCloudHost KVM VPS       │
│ • Biaya: 100% Gratis Selamanya       │ • Biaya: VPS Hourly / Saldo Akun     │
│ • Engine: WebAssembly (WASM) di RAM  │ • Engine: Hardware Kernel Ubuntu 24  │
│ • Ketersediaan: Online 24/7/365      │ • Ketersediaan: Tergantung VPS Aktif │
│ • Terminal: Sandbox Shell Instan     │ • Terminal: Real Root Bash (ttyd)    │
│ • Desktop: GUI Workstation Preview   │ • Desktop: Real Container noVNC GUI  │
│ • Cocok untuk: Demo & Edukasi Cepat  │ • Cocok untuk: Produksi & Dev Berat  │
└──────────────────────────────────────┴──────────────────────────────────────┘
```

### Mengapa Script Instalasi (`install.ps1` & `install.sh`) Mengarah ke `nantara.cloud`?
1. **Instalasi Lokal Butuh Server Nyata:** Saat pengguna mengunduh NantaraVM ke laptopnya melalui perintah:
   ```powershell
   iwr -useb https://raw.githubusercontent.com/camanit/nantara-vm/main/web/install.ps1 | iex
   ```
   Aplikasi CLI di laptop membutuhkan endpoint kontrol pusat untuk mengelola VM, mengambil template image OS (Ubuntu/Kali), dan berkomunikasi dengan hypervisor. Karena Vercel adalah *serverless static hosting*, Vercel tidak bisa menerima koneksi socket CLI background. Oleh karena itu, target instalasi diarahkan ke **`https://nantara.cloud/`**.
2. **Dashboard yang Sama, Mode yang Berbeda:**
   Dashboard NantaraVM (`dashboard.html`) dilengkapi fitur **Smart Auto-Detection**:
   - Jika dibuka di `nantara-vm.vercel.app`, dashboard otomatis menyalakan **Mode WebAssembly**, memunculkan tombol interaktif bantuan, dan memberikan opsi beralih ke Dedicated VPS jika butuh tenaga server asli.
   - Jika dibuka di `nantara.cloud` atau `103.226.138.53`, dashboard otomatis menyalakan **Mode Dedicated Cloud**, menghubungkan iframe terminal langsung ke `ttyd` (port 7681) dan GUI langsung ke `noVNC` (port 6080).

---

## 🔄 8. Cara Update File Web di VPS Jika Ada Perubahan di GitHub

Jika sobat melakukan edit pada file HTML/CSS/JS di repository GitHub, cukup login ke SSH VPS dan jalankan perintah update kilat ini:

```bash
sudo git clone --depth 1 https://github.com/camanit/nantara-vm.git /tmp/nantara-update && sudo cp -r /tmp/nantara-update/web/* /var/www/html/ && sudo rm -rf /tmp/nantara-update
```
*Perintah ini akan langsung menyalin versi terbaru ke `/var/www/html/` tanpa mengganggu proses `ttyd` atau `Docker` yang sedang berjalan.*

---

## 🌐 9. Troubleshooting DNS Cache ISP / Router Rumah

Jika sobat membuka `https://nantara.cloud/` tapi masih melihat tampilan 404 Vercel, jangan panik:
* **Penyebab:** DNS router WiFi rumah (biasanya Telkom/Indihome/FirstMedia) menyimpan cache domain lama selama 2 - 4 jam (TTL).
* **Solusi Cepat:**
  1. Akses langsung melalui IP VPS: **`http://103.226.138.53/`**
  2. Atau ganti DNS di Windows ke **Google DNS (`8.8.8.8`)** atau **Cloudflare (`1.1.1.1`)**.
  3. Atau di Google Chrome: Buka **Settings** ➡️ **Privacy and security** ➡️ **Security** ➡️ Pilih **Use secure DNS** ➡️ Pilih **Cloudflare (1.1.1.1)**.
