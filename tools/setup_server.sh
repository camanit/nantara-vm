#!/bin/bash
# ==============================================================================
# NantaraVM — 1-Click Server Setup Script untuk Ubuntu (IDCloudHost / Cloud VPS)
# ==============================================================================
# Fungsi: Menginstal & menjalankan Nginx, Real Bash Web Terminal (ttyd),
#         Docker Desktop Container (Kali Linux GUI via noVNC), dan Web Dashboard.
# ==============================================================================

set -e

echo "=========================================================="
echo " 🚀 NantaraVM — Cloud Server Auto-Provisioner (NKRI 2026)"
echo "=========================================================="
echo ""

# 1. Update sistem & dependensi dasar
echo "[1/6] Mengupdate paket sistem Ubuntu..."
sudo apt-get update -y
sudo apt-get install -y curl git ufw wget ca-certificates gnupg lsb-release
sudo apt-get install -y nginx || true

# Perbaiki issue IPv6 IDCloudHost jika dinonaktifkan di kernel
if [ -f "/etc/nginx/sites-available/default" ]; then
    sudo sed -i 's/listen \[::\]:80 default_server;/# listen [::]:80 default_server;/g' /etc/nginx/sites-available/default
    sudo sed -i 's/listen \[::\]:80;/# listen [::]:80;/g' /etc/nginx/sites-available/default
fi
sudo apt-get install -f -y

# 2. Pasang Web Terminal (ttyd) untuk /console/
echo "[2/6] Memasang ttyd (Web Terminal Engine)..."
if ! command -v ttyd &> /dev/null; then
    sudo apt-get install -y ttyd || {
        echo "Mengunduh binary ttyd rilis terbaru..."
        sudo wget -q https://github.com/tsl0922/ttyd/releases/download/1.7.7/ttyd.x86_64 -O /usr/local/bin/ttyd
        sudo chmod +x /usr/local/bin/ttyd
    }
fi

# Buat Systemd Service untuk ttyd (Port 7681)
sudo bash -c 'cat <<EOF > /etc/systemd/system/ttyd.service
[Unit]
Description=NantaraVM Web Terminal Console (ttyd)
After=network.target

[Service]
ExecStart=/usr/bin/ttyd -p 7681 -t fontSize=14 -t fontFamily="Fira Code, monospace" bash
Restart=always
User=root
WorkingDirectory=/root

[Install]
WantedBy=multi-user.target
EOF'

# Jika binary terpasang di /usr/local/bin/ttyd, sesuaikan path
if [ -f "/usr/local/bin/ttyd" ] && [ ! -f "/usr/bin/ttyd" ]; then
    sudo sed -i 's|/usr/bin/ttyd|/usr/local/bin/ttyd|g' /etc/systemd/system/ttyd.service
fi

sudo systemctl daemon-reload
sudo systemctl enable ttyd
sudo systemctl restart ttyd
echo "✅ Service ttyd aktif di port 7681."

# 3. Pasang Docker Engine (jika belum ada)
echo "[3/6] Memasang Docker Engine untuk Container Desktop GUI..."
if ! command -v docker &> /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo systemctl enable docker
    sudo systemctl start docker
    rm -f get-docker.sh
fi

# 4. Jalankan Desktop Kali Linux / Ubuntu GUI noVNC di port 6080
echo "[4/6] Menjalankan Container Desktop GUI noVNC..."
if sudo docker ps -a --format '{{.Names}}' | grep -q '^nantara-desktop$'; then
    echo "Menghapus container lama..."
    sudo docker rm -f nantara-desktop
fi

sudo docker run -d \
    --name nantara-desktop \
    --restart always \
    -p 6080:80 \
    -e USER=caman \
    -e PASSWORD=nantara \
    -e RESOLUTION=1280x720 \
    dorowu/ubuntu-desktop-lxde-vnc:latest || {
        echo "[WARNING] Mencoba image cadangan kasmweb/kali-rolling-desktop..."
        sudo docker run -d --name nantara-desktop --restart always -p 6080:6901 -e VNC_PW=password kasmweb/kali-rolling-desktop:1.15.0
    }
echo "✅ Container Desktop GUI noVNC aktif di port 6080."

# 5. Konfigurasi Nginx Reverse Proxy & Web
echo "[5/6] Mengonfigurasi Nginx Reverse Proxy (/console/ & /desktop/)..."
sudo bash -c 'cat <<EOF > /etc/nginx/sites-available/default
server {
    listen 80 default_server;
    server_name nantara.cloud www.nantara.cloud _;

    root /var/www/html;
    index dashboard.html index.html;

    location / {
        try_files \$uri \$uri/ /dashboard.html;
    }

    # Web Terminal ttyd (WebSocket)
    location /console/ {
        proxy_pass http://127.0.0.1:7681/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host \$host;
        proxy_read_timeout 86400;
        proxy_send_timeout 86400;
    }

    # Graphical Desktop noVNC (WebSocket)
    location /desktop/ {
        proxy_pass http://127.0.0.1:6080/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host \$host;
        proxy_read_timeout 86400;
        proxy_send_timeout 86400;
    }
}
EOF'

# 6. Salin berkas Frontend NantaraVM ke /var/www/html
echo "[6/6] Mengunduh & men-deploy berkas Web NantaraVM ke /var/www/html..."
sudo rm -rf /tmp/nantara-vm-temp
git clone --depth 1 https://github.com/camanit/nantara-vm.git /tmp/nantara-vm-temp
sudo rm -rf /var/www/html/*
sudo cp -r /tmp/nantara-vm-temp/web/* /var/www/html/
sudo rm -rf /tmp/nantara-vm-temp
sudo chown -R www-data:www-data /var/www/html

# Restart & Verifikasi Nginx
sudo nginx -t
sudo systemctl restart nginx
sudo systemctl enable nginx

echo ""
echo "=========================================================="
echo " 🎉 SETUP SELESAI! SEMUA SERVICE BERHASIL AKTIF!"
echo "=========================================================="
echo " 🌐 Web Dashboard  : http://IP_SERVER/ (atau https://nantara.cloud)"
echo " 💻 Web Terminal   : http://IP_SERVER/console/ (ttyd Port 7681)"
echo " 🖥️ Desktop noVNC  : http://IP_SERVER/desktop/ (noVNC Port 6080)"
echo "=========================================================="
echo " Cek status port aktif:"
sudo ss -tulpn | grep -E ':80|:7681|:6080'
echo "=========================================================="
