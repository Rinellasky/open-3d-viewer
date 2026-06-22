# setup.ps1 - one-time vendor file download for open-3d-viewer
# Run from PowerShell:  .\setup.ps1
# Re-running is safe; it only downloads/extracts missing files.

$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
if (-not $root) { $root = (Get-Location).Path }

Write-Host "open-3d-viewer setup" -ForegroundColor Cyan
Write-Host "Vendor target: $root\web\vendor" -ForegroundColor Gray

# ---- [1/2] Three.js (Phase 7: offline-first) ----
$threeDir = "$root\web\vendor\three"
$threeVer = '0.160.0'
$threeMarker = "$threeDir\.installed-$threeVer"

Write-Host "`n[1/2] Three.js $threeVer (offline-first)" -ForegroundColor Yellow
if (Test-Path $threeMarker) {
  Write-Host "  [ok]  already installed (delete .installed-$threeVer to force reinstall)" -ForegroundColor Green
} else {
  $tarUrl  = "https://registry.npmjs.org/three/-/three-$threeVer.tgz"
  $tarFile = "$env:TEMP\three-$threeVer.tgz"
  $stage   = "$env:TEMP\open3dv-three-extract"

  if (Test-Path $stage)    { Remove-Item $stage -Recurse -Force }
  if (Test-Path $threeDir) { Remove-Item $threeDir -Recurse -Force }
  New-Item -ItemType Directory -Path $stage    | Out-Null
  New-Item -ItemType Directory -Path $threeDir | Out-Null

  Write-Host "  [..]  downloading tarball..." -NoNewline
  Invoke-WebRequest $tarUrl -OutFile $tarFile -UseBasicParsing
  Write-Host " $(( [math]::Round((Get-Item $tarFile).Length/1MB, 1)) ) MB"

  Write-Host "  [..]  extracting build/ and examples/jsm/..." -NoNewline
  # tar from npm extracts into ./package/
  & tar.exe -xzf $tarFile -C $stage
  Copy-Item "$stage\package\build"        "$threeDir\build"            -Recurse
  Copy-Item "$stage\package\examples\jsm" "$threeDir\examples\jsm"     -Recurse -Container
  Write-Host " done"

  # Trim weight: examples/jsm has ~30MB of stuff we don't load. We only need
  # the subdirs that index.html actually imports from.
  $keep = @('controls','loaders','environments','libs')
  $jsmDir = "$threeDir\examples\jsm"
  Get-ChildItem $jsmDir -Force | Where-Object {
    $_.PSIsContainer -and ($_.Name -notin $keep)
  } | ForEach-Object { Remove-Item $_.FullName -Recurse -Force }

  # Cleanup
  Remove-Item $tarFile, $stage -Recurse -Force

  # Mark installed
  Set-Content "$threeMarker" "three $threeVer installed $(Get-Date -Format o)"
  $totalMB = [math]::Round((Get-ChildItem $threeDir -Recurse -File | Measure-Object Length -Sum).Sum / 1MB, 1)
  Write-Host "  [ok]  $threeVer installed ($totalMB MB on disk)" -ForegroundColor Green
}

# ---- [2/2] OpenSCAD WASM (Phase 4: text-to-3D) ----
$openscadDir = "$root\web\vendor\openscad-wasm"
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
Write-Host "`n[2/2] OpenSCAD WASM 2022.03.20" -ForegroundColor Yellow
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
Write-Host "Open web\index.html in a browser, or run the Tauri app via" -ForegroundColor Gray
Write-Host "  cd src-tauri" -ForegroundColor Gray
Write-Host "  cargo run        # debug build" -ForegroundColor Gray
Write-Host "  tauri build      # installer" -ForegroundColor Gray
