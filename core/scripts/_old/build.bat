@echo off

@REM Set root dir
cd /d "%~dp0" || exit /b 1

@REM Remove old build
if exist libs rmdir /s /q libs
mkdir libs\msvcX64

@REM Build
rustup target add x86_64-pc-windows-msvc
cargo run --target x86_64-pc-windows-msvc --features headers --bin core-headers
cargo build --target x86_64-pc-windows-msvc --release
copy target\x86_64-pc-windows-msvc\release\core.dll libs\msvcX64\
move libs\libcore.h libs\msvcX64\
