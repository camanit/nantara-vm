#!/bin/sh
# NantaraVM NKRI 2026 Instant Auto-Installer Script
set -e

echo "===================================================="
echo " 🚀 Installing NantaraVM MicroVM Hypervisor"
echo "===================================================="

INSTALL_DIR="/usr/local/bin"
BINARY_URL="https://nantara.cloud/downloads/nantara-vm"

echo "[NantaraVM Installer] Downloading latest NantaraVM binary from nantara.cloud..."
curl -fsSL "$BINARY_URL" -o /tmp/nantara-vm || {
    echo "[NantaraVM Installer Error] Download failed. Building from source via Cargo..."
    cargo install nantara-vm
    exit 0
}

chmod +x /tmp/nantara-vm
sudo mv /tmp/nantara-vm "$INSTALL_DIR/nantara-vm"

echo "[NantaraVM Installer] Installation successful!"
echo "[NantaraVM Installer] Run 'nantara-vm' in your terminal to start."
