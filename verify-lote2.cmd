@echo off
REM Verificacion del lote 2: pruebas de sfx y check de la ventana, con
REM el entorno de Visual Studio que bindgen necesita para FFmpeg.
call "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\VsDevCmd.bat" -arch=amd64
set PATH=C:\Users\user\.cargo\bin;C:\Users\user\Downloads\liga\ffmpeg-n9.0-latest-win64-gpl-shared-9.0\bin;%PATH%
set FFMPEG_DIR=C:\Users\user\Downloads\liga\ffmpeg-n9.0-latest-win64-gpl-shared-9.0
set LIBCLANG_PATH=C:\Program Files\LLVM\bin
set BINDGEN_EXTRA_CLANG_ARGS=-I"C:\Program Files\LLVM\lib\clang\23\include" -I"C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.51.36231\include" -I"C:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\ucrt" -I"C:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\um"
cd /d c:\Users\user\Downloads\Concat-main-lote1\Concat-main\src
cargo test -p concat-host sfx > c:\Users\user\Downloads\Concat-main-lote1\Concat-main\test-host.log 2>&1
cargo check -p concat > c:\Users\user\Downloads\Concat-main-lote1\Concat-main\check-concat.log 2>&1
echo done > c:\Users\user\Downloads\Concat-main-lote1\Concat-main\verify.done