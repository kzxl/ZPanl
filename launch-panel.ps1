# Launch ZPanl Web Panel on Windows Localhost
$port = 8888
$url = "http://localhost:$port"

Write-Host "⚡ Building and launching ZPanl on $url..." -ForegroundColor Cyan

# Ensure binary is built
cargo build

$exePath = Join-Path $PSScriptRoot "target\debug\zpanl.exe"
if (-not (Test-Path $exePath)) {
    Write-Error "Binary not found at $exePath"
    exit 1
}

# Launch browser after a short delay
Start-Job -ScriptBlock {
    param($u)
    Start-Sleep -Seconds 1
    Start-Process $u
} -ArgumentList $url | Out-Null

Write-Host "🌐 Opening browser to $url" -ForegroundColor Green
Write-Host "Press Ctrl+C in this terminal to stop the ZPanl server." -ForegroundColor Yellow

# Run daemon
& $exePath run --bind "127.0.0.1:$port"
