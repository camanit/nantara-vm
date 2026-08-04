Add-Type -AssemblyName System.Drawing

$pngPath = "c:\Users\UseR\Documents\NantaraVM\web\assets\icon.png"
$icoPath = "c:\Users\UseR\Documents\NantaraVM\web\assets\icon.ico"
$icoPathRoot = "c:\Users\UseR\Documents\NantaraVM\web\icon.ico"

$img = [System.Drawing.Image]::FromFile($pngPath)
$bmp = New-Object System.Drawing.Bitmap $img, 64, 64
$hIcon = $bmp.GetHicon()
$icon = [System.Drawing.Icon]::FromHandle($hIcon)

$fs1 = [System.IO.File]::Create($icoPath)
$icon.Save($fs1)
$fs1.Close()

$fs2 = [System.IO.File]::Create($icoPathRoot)
$icon.Save($fs2)
$fs2.Close()

Write-Host "Created icon.ico successfully!"
