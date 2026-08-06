@echo off
title NantaraVM NKRI 2026 Launcher
color 0A
cls
echo ====================================================
echo  🚀 Launching NantaraVM MicroVM Hypervisor Engine
echo ====================================================
echo.

echo [1/2] Memulai NantaraVM REST API Server (Port 8080)...
if exist "%~dp0nantara-engine.exe" (
    start /b "" "%~dp0nantara-engine.exe" >nul 2>&1
) else if exist "%~dp0..\..\target\debug\nantara-engine.exe" (
    start /b "" "%~dp0..\..\target\debug\nantara-engine.exe" >nul 2>&1
) else (
    echo [INFO] Menjalankan Nantara Engine via Cargo...
    start /b "" cargo run --bin nantara-engine >nul 2>&1
)

echo [2/2] Membuka NantaraVM Workstation Dashboard...
timeout /t 2 >nul
start https://nantara.cloud/dashboard.html

echo.
echo ====================================================
echo  ✅ NantaraVM Engine Aktif di Background (Port 8080)
echo  Dashboard Web telah dibuka di Browser Anda!
echo ====================================================
timeout /t 5 >nul

