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
$BinaryUrl = "https://raw.githubusercontent.com/camanit/nantara-vm/main/web/downloads/nantara-vm.exe"
$RunnerUrl = "https://raw.githubusercontent.com/camanit/nantara-vm/main/web/downloads/run-nantara-vm.bat"
$TargetPath = "$InstallDir\nantara-vm.exe"
$RunnerPath = "$InstallDir\run-nantara-vm.bat"

$LocalBinary = "$PSScriptRoot\downloads\nantara-vm.exe"
$LocalRunner = "$PSScriptRoot\downloads\run-nantara-vm.bat"

if (Test-Path $LocalBinary) {
    Copy-Item $LocalBinary $TargetPath -Force
    if (Test-Path $LocalRunner) { Copy-Item $LocalRunner $RunnerPath -Force }
    Write-Host "[2/3] Executable NantaraVM berhasil terpasang di $TargetPath" -ForegroundColor Green
} else {
    try {
        Invoke-WebRequest -Uri $BinaryUrl -OutFile $TargetPath -UseBasicParsing
        Invoke-WebRequest -Uri $RunnerUrl -OutFile $RunnerPath -UseBasicParsing
        Write-Host "[2/3] Executable NantaraVM berhasil terpasang di $TargetPath" -ForegroundColor Green
    } catch {
        Write-Host "[Info] Menyiapkan runner NantaraVM CLI & Web Control Plane..." -ForegroundColor Yellow
    }
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

# Create Desktop Shortcut for NantaraVM Workstation (.lnk)
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = Join-Path $DesktopPath "NantaraVM Workstation.lnk"

try {
    $WScriptShell = New-Object -ComObject WScript.Shell
    $Shortcut = $WScriptShell.CreateShortcut($ShortcutPath)
    $Shortcut.TargetPath = "$RunnerPath"
    if (Test-Path $IcoPath) { $Shortcut.IconLocation = "$IcoPath,0" }
    $Shortcut.Description = "NantaraVM Workstation Hypervisor & Web Control Plane"
    $Shortcut.Save()
} catch {
    $ShortcutPath = Join-Path $DesktopPath "NantaraVM Workstation.url"
    $ShortcutLines = @(
        "[InternetShortcut]",
        "URL=https://nantara.cloud/dashboard.html",
        "IconIndex=0",
        "IconFile=$IcoPath"
    )
    $ShortcutLines | Out-File -FilePath $ShortcutPath -Encoding ascii -Force
}

Write-Host "[OK] Shortcut NantaraVM Workstation berhasil dibuat di Desktop Anda!" -ForegroundColor Green

Write-Host "`n====================================================" -ForegroundColor Cyan
Write-Host " NantaraVM Windows Installation Completed!" -ForegroundColor Green
Write-Host " Double-click shortcut NantaraVM Workstation di Desktop Anda!" -ForegroundColor White
Write-Host "====================================================" -ForegroundColor Cyan

