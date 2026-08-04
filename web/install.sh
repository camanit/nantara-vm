#!/bin/sh
# NantaraVM — 1-Click Auto Installer
# MicroVM Hypervisor Karya Anak Bangsa 🇮🇩

set -e

echo "===================================================="
echo " NantaraVM — Open Source MicroVM Hypervisor v0.1"
echo " Karya Indonesia 🇮🇩"
echo "===================================================="
echo ""

# Check Linux OS
if [ "$(uname -s)" != "Linux" ]; then
    echo "[ERROR] NantaraVM Native Engine memerlukan OS Linux (atau WSL2 di Windows)."
    exit 1
fi

# Check /dev/kvm
if [ ! -c /dev/kvm ]; then
    echo "[WARNING] /dev/kvm tidak ditemukan atau belum aktif."
    echo "[HINT] Pastikan Virtualization (Intel VT-x/AMD-V) aktif di BIOS/WSL."
fi

INSTALL_DIR="/usr/local/bin"
BINARY_NAME="nantara-vm"

echo "[1/3] Mengunduh Binary Release NantaraVM v0.1..."
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "https://raw.githubusercontent.com/camanit/nantara-vm/main/web/downloads/nantara-vm-linux-x86_64" -o "$BINARY_NAME" || true
elif command -v wget >/dev/null 2>&1; then
    wget -q "https://raw.githubusercontent.com/camanit/nantara-vm/main/web/downloads/nantara-vm-linux-x86_64" -O "$BINARY_NAME" || true
fi

if [ -f "$BINARY_NAME" ]; then
    echo "[2/3] Memasang NantaraVM ke $INSTALL_DIR..."
    chmod +x "$BINARY_NAME"
    if [ -w "$INSTALL_DIR" ]; then
        mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    else
        sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    fi
    echo "[3/3] Pemasangan Berhasil! 🎉"
    echo ""
    echo "Jalankan NantaraVM dengan mengetik:"
    echo "  nantara-vm"
else
    echo "[INFO] Mengunduh source code untuk kompilasi lokal..."
    git clone https://github.com/camanit/nantara-vm.git
    cd nantara-vm
    cargo build --release
    echo "Binary tersedia di: target/release/nantara-vm"
fi

echo ""
echo "Dokumentasi: https://nantara.cloud/docs.html"
echo "GitHub: https://github.com/camanit/nantara-vm"
echo ""
