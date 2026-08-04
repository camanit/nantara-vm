@echo off
title NantaraVM NKRI 2026 Launcher
color 0A
cls
echo ====================================================
echo  🚀 Launching NantaraVM MicroVM Hypervisor
echo ====================================================
echo.

set WIN_ISO="%~dp0Win10.iso"

if exist %WIN_ISO% (
    echo [INFO] Berkas ISO Ditemukan!
    "%~dp0nantara-vm.exe" --iso %WIN_ISO% --ram 2048 --cpus 2 %*
) else (
    "%~dp0nantara-vm.exe" %*
)

echo.
echo ====================================================
echo  Session completed. Press Enter to close window...
echo ====================================================
pause > nul
