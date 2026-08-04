@echo off
title NantaraVM NKRI 2026 Launcher
color 0A
cls
echo ====================================================
echo  🚀 Launching NantaraVM MicroVM Hypervisor
echo ====================================================
echo.

set WIN_ISO="C:\Users\UseR\Documents\os win\Win10_22H2_EnglishInternational_x32v1.iso"

if exist %WIN_ISO% (
    echo [INFO] Berkas Windows 10 ISO Ditemukan!
    echo.
    echo 1. Jalankan NantaraVM dengan ISO Windows 10 (Boot Windows)
    echo 2. Jalankan NantaraVM Demo Mode (Simulasi Super Cepat)
    echo.
    set /p CHOICE="Pilih Mode (1/2, default 1): "
    if "%CHOICE%"=="2" (
        "%~dp0nantara-vm.exe" %*
    ) else (
        echo [NantaraVM] Loading Windows 10 ISO into virtio-blk and virtio-gpu...
        "%~dp0nantara-vm.exe" --iso %WIN_ISO% --ram 2048 --cpus 2 --display virtio-gpu %*
    )
) else (
    "%~dp0nantara-vm.exe" %*
)

echo.
echo ====================================================
echo  Session completed. Press Enter to close window...
echo ====================================================
pause > nul
