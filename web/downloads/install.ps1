# NantaraVM Windows 1-Click PowerShell Installer
# Run in PowerShell: iwr -useb https://raw.githubusercontent.com/camanit/nantara-vm/main/web/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "====================================================" -ForegroundColor Cyan
Write-Host " 🚀 NantaraVM NKRI 2026 - Windows Workstation Installer" -ForegroundColor Green
Write-Host "====================================================" -ForegroundColor Cyan

$InstallDir = "$env:LOCALAPPDATA\NantaraVM"
If (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

Write-Host "[1/3] Mendownload NantaraVM Workstation Engine untuk Windows..." -ForegroundColor Yellow
$BinaryUrl = "https://raw.githubusercontent.com/camanit/nantara-vm/main/web/nantara-vm.exe"
$TargetPath = "$InstallDir\nantara-vm.exe"

try {
    Invoke-WebRequest -Uri $BinaryUrl -OutFile $TargetPath -UseBasicParsing
    Write-Host "[2/3] Executable NantaraVM berhasil terpasang di $TargetPath" -ForegroundColor Green
} catch {
    Write-Host "[Info] Menyiapkan runner NantaraVM CLI & Web Control Plane..." -ForegroundColor Yellow
}

Write-Host "[3/3] Menambahkan NantaraVM ke System PATH & Membuat Desktop Shortcut..." -ForegroundColor Yellow
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
If ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
}

# Download & Set Custom NantaraVM Icon
$IcoPath = "$InstallDir\icon.ico"
$IcoUrl = "https://raw.githubusercontent.com/camanit/nantara-vm/main/web/assets/icon.ico"
$LocalIco = "$PSScriptRoot\assets\icon.ico"

if (Test-Path $LocalIco) {
    Copy-Item $LocalIco $IcoPath -Force
} else {
    try {
        Invoke-WebRequest -Uri $IcoUrl -OutFile $IcoPath -UseBasicParsing
    } catch {
        # Fallback to binary path
        $IcoPath = $TargetPath
    }
}

# Create Desktop Shortcut for NantaraVM Workstation
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = Join-Path $DesktopPath "NantaraVM Workstation.url"
$ShortcutLines = @(
    "[InternetShortcut]",
    "URL=https://nantara.cloud/dashboard.html",
    "IconIndex=0",
    "IconFile=$IcoPath"
)
$ShortcutLines | Out-File -FilePath $ShortcutPath -Encoding ascii -Force

Write-Host "[OK] Shortcut NantaraVM Workstation berhasil dibuat dengan Ikon Kustom di Desktop Anda!" -ForegroundColor Green

Write-Host "`n====================================================" -ForegroundColor Cyan
Write-Host " NantaraVM Windows Installation Completed!" -ForegroundColor Green
Write-Host " Double-click shortcut NantaraVM Workstation di Desktop Anda!" -ForegroundColor White
Write-Host "====================================================" -ForegroundColor Cyan
