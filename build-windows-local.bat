@echo off
REM Build local de Concat en Windows - doble clic o ejecutar en cmd
REM Requiere: Rust (ya lo tenes: 1.99.1), cmake, Visual Studio Build Tools con C++, libclang (LLVM)
setlocal EnableDelayedExpansion

set "REPO=C:\Users\user\Downloads\Concat-main-lote1\Concat-main"
set "FFMPEG_DIR=C:\Users\user\Downloads\liga\ffmpeg-n9.0-latest-win64-gpl-shared-9.0"
set "PATH=C:\Users\user\.cargo\bin;%PATH%"
set "PATH=%FFMPEG_DIR%\bin;%PATH%"
if exist "C:\llvm\bin" set "PATH=C:\llvm\bin;%PATH%"
if exist "C:\llvm\bin\libclang.dll" set "LIBCLANG_PATH=C:\llvm\bin"

echo === Concat build local ===
echo REPO=%REPO%
echo FFMPEG_DIR=%FFMPEG_DIR%
cargo --version || (echo [ERROR] cargo no encontrado & pause & exit /b 1)
rustc --version
cmake --version || (echo [ERROR] cmake no encontrado en PATH. Instalalo desde https://cmake.org/download/ y reabri la terminal & pause & exit /b 1)
where cl >nul 2>&1
if errorlevel 1 (
  echo [AVISO] 'cl' (MSVC) no esta en PATH en esta terminal.
  echo Abri "x64 Native Tools Command Prompt for VS" y corre este mismo script desde ahi,
  echo o ejecuta antes VsDevCmd.bat
)
if not exist "%FFMPEG_DIR%\include\libavutil\version.h" (
  echo [ERROR] FFMPEG_DIR no apunta a un build shared valido
  pause & exit /b 1
)
if "%LIBCLANG_PATH%"=="" (
  echo [AVISO] LIBCLANG_PATH no configurado (bindgen lo necesita).
  echo Instala LLVM 21.1.0 en C:\llvm desde https://github.com/llvm/llvm-project/releases
)
echo.
echo === Compilando (perfil quick: optimizado pero rapido, ideal para probar) ===
cd /d "%REPO%\src" || (echo [ERROR] no existe %REPO%\src & pause & exit /b 1)
cargo run --profile quick -p concat
echo.
echo Exit code: %ERRORLEVEL%
pause
