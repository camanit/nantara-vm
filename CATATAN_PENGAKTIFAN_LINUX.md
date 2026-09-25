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
    listen [::]:80 default_server;
    server_name nantara.cloud www.nantara.cloud _;

    root /var/www/html;
    index dashboard.html index.html;

    location / {
        try_files $uri $uri/ /dashboard.html;
    }

    # Teruskan /console/ ke ttyd
    location /console/ {
        proxy_pass http://127.0.0.1:7681/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_read_timeout 86400;
    }

    # Teruskan /desktop/ ke noVNC
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
Certbot akan otomatis memperbarui konfigurasi Nginx dan memperpanjang sertifikat SSL secara otomatis.
