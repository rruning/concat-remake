# Build Concat en Windows (x64) - replica .github/workflows/build-app.yml
# Uso: powershell -ExecutionPolicy Bypass -File scripts\build-windows-local.ps1 [-Profile app]
param([string]$Profile = "app")
$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
Set-Location $root
Write-Host "== 1/6 Rust 1.93.1 =="
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  $rustup = "$env:TEMP\rustup-init.exe"
  Invoke-WebRequest https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile $rustup
  & $rustup -y --profile minimal --default-toolchain 1.93.1
  $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
}
cargo --version; rustc --version
Write-Host "== 2/6 cmake + LLVM (scoop) =="
if (-not (Get-Command cmake -ErrorAction SilentlyContinue)) { scoop install cmake }
if (-not (Test-Path C:\llvm\bin\libclang.dll)) {
  scoop install llvm
  $env:LIBCLANG_PATH = "$(scoop prefix llvm)\bin"
} else { $env:LIBCLANG_PATH = "C:\llvm\bin" }
Write-Host "== 3/6 FFmpeg BtbN shared (FFMPEG_DIR) =="
if (-not $env:FFMPEG_DIR -or -not (Test-Path $env:FFMPEG_DIR)) {
  $lock = Get-Content scripts\ci\deps.lock | Where-Object { $_ -match '^ffmpeg-win64\s' }
  $url = ($lock -split '\s+')[1]
  $zip = "$env:TEMP\ffmpeg-win.zip"
  if (-not (Test-Path $zip)) { Invoke-WebRequest $url -OutFile $zip }
  Expand-Archive $zip -DestinationPath ffdl -Force
  $dir = (Get-ChildItem ffdl -Directory | Select-Object -First 1).FullName
  $env:FFMPEG_DIR = $dir
  [Environment]::SetEnvironmentVariable("FFMPEG_DIR", $dir, "User")
}
Write-Host "FFMPEG_DIR=$env:FFMPEG_DIR"
Write-Host "== 4/6 ONNX Runtime + sherpa-onnx =="
Write-Host "(los descarga el build con SHERPA_ONNX_LIB_DIR/ORT_LIB_LOCATION si faltan; ver build-app.yml pasos Windows)"
Write-Host "== 5/6 VS C++ (cl.exe) =="
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vswhere) { & $vswhere -latest -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath }
Write-Host "== 6/6 cargo build --profile $Profile -p concat =="
Set-Location src
cargo build --profile $Profile -p concat
Write-Host "OK -> src\target\…\concat.exe"
