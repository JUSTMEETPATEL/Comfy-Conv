# comfy-conv Windows Installer
# Run: irm https://raw.githubusercontent.com/JUSTMEETPATEL/Comfy-Conv/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "🔧 Installing comfy-conv..." -ForegroundColor Cyan

# Determine architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "x86" }

# Get latest release
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/JUSTMEETPATEL/Comfy-Conv/releases/latest"
$asset = $release.assets | Where-Object { $_.name -like "*windows*$arch*" } | Select-Object -First 1

if (-not $asset) {
    Write-Host "❌ Could not find Windows binary in latest release" -ForegroundColor Red
    exit 1
}

# Create install directory
$installDir = "$env:LOCALAPPDATA\comfy-conv"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

# Download binary
$binaryPath = "$installDir\comfy-conv.exe"
Write-Host "📦 Downloading from $($asset.browser_download_url)..."
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $binaryPath

# Add to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "✅ Added to PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "✅ comfy-conv installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "🔧 Now install dependencies:" -ForegroundColor Yellow
Write-Host "   winget install LibreOffice.LibreOffice"
Write-Host "   winget install JohnMacFarlane.Pandoc"
Write-Host ""
Write-Host "Or run: comfy-conv --setup" -ForegroundColor Cyan
Write-Host ""
Write-Host "⚠️  Restart your terminal, then run: comfy-conv" -ForegroundColor Yellow
