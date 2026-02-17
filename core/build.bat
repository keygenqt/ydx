@echo off

@REM Set root dir
cd /d "%~dp0" || exit /b 1

@REM Remove old build
if exist libs rmdir /s /q libs
mkdir libs\msvcX64

@REM Build MSVC
@REM rustup target add x86_64-pc-windows-msvc
@REM cargo run --target x86_64-pc-windows-msvc --features headers --bin shared-headers
@REM cargo build --target x86_64-pc-windows-msvc --release
@REM copy target\x86_64-pc-windows-msvc\release\shared.dll libs\msvcX64\
@REM move libs\libshared_api.h libs\msvcX64\

@REM Build MinGW
rustup target add x86_64-pc-windows-gnu
cargo run --target x86_64-pc-windows-gnu --features headers --bin shared-headers
cargo build --target x86_64-pc-windows-gnu --release
copy target\x86_64-pc-windows-gnu\release\shared.dll libs\msvcX64\
move libs\libshared_api.h libs\msvcX64\

