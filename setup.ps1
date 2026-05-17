# setup.ps1 - one-time vendor file download for open-3d-viewer
# Run from PowerShell:  .\setup.ps1
# Re-running is safe; it only downloads missing files.

$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
if (-not $root) { $root = (Get-Location).Path }

Write-Host "open-3d-viewer setup" -ForegroundColor Cyan
Write-Host "Vendor target: $root\vendor" -ForegroundColor Gray

# ---- OpenSCAD WASM (Phase 4: text-to-3D) ----
$openscadDir = "$root\vendor\openscad-wasm"
$openscadBase = 'https://github.com/openscad/openscad-wasm/releases/download/2022.03.20'
$openscadFiles = @(
  @{ name='openscad.js';      bytes=745 },
  @{ name='openscad.wasm.js'; bytes=120025 },
  @{ name='openscad.wasm';    bytes=7720447 },
  @{ name='openscad.mcad.js'; bytes=491462 }
)
# Optional bigger bundle (fonts) - skip by default. Uncomment if you need text() in OpenSCAD.
# $openscadFiles += @{ name='openscad.fonts.js'; bytes=8163407 }

New-Item -ItemType Directory -Path $openscadDir -Force | Out-Null
Write-Host "`n[1/1] OpenSCAD WASM 2022.03.20" -ForegroundColor Yellow
foreach ($f in $openscadFiles) {
  $dest = "$openscadDir\$($f.name)"
  if ((Test-Path $dest) -and ((Get-Item $dest).Length -eq $f.bytes)) {
    Write-Host "  [ok]  $($f.name) ($([math]::Round($f.bytes/1MB,2)) MB)" -ForegroundColor Green
    continue
  }
  $kb = [math]::Round($f.bytes/1KB,1)
  Write-Host "  [..]  $($f.name) (~$kb KB)..." -NoNewline
  Invoke-WebRequest "$openscadBase/$($f.name)" -OutFile $dest -UseBasicParsing
  $actual = (Get-Item $dest).Length
  if ($actual -ne $f.bytes) {
    Write-Host " WARN: size mismatch (expected $($f.bytes), got $actual)" -ForegroundColor Yellow
  } else {
    Write-Host " done" -ForegroundColor Green
  }
}

Write-Host "`nSetup complete." -ForegroundColor Cyan
Write-Host "Open index.html in your browser to use the viewer." -ForegroundColor Gray
