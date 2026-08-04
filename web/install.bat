@echo off
title NantaraVM Windows Workstation 1-Click Installer
color 0A
echo ====================================================
echo  🚀 NantaraVM NKRI 2026 - Windows Workstation Installer
echo ====================================================
echo.
echo [1/3] Memeriksa Hak Akses Administrator...
net session >nul 2>&1
if %errorLevel% == 0 (
    echo [OK] Hak Akses Administrator Diterima.
) else (
    echo [INFO] Menjalankan installer NantaraVM...
)

echo.
echo [2/3] Memasang NantaraVM Workstation Engine...
powershell -NoProfile -ExecutionPolicy Bypass -Command "iwr -useb https://raw.githubusercontent.com/camanit/nantara-vm/main/web/install.ps1 | iex"

echo.
echo ====================================================
echo  🎉 NantaraVM Berhasil Terpasang di Sistem Windows!
echo  Buka Dashboard Web: https://nantara.cloud/dashboard.html
echo ====================================================
pause
