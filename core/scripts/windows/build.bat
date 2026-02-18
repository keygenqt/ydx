@echo off

@REM Set root dir
cd /d "%~dp0..\.." || exit /b 1

@REM Params build
set "TYPE=release"
set "TARGETS=x86_64-pc-windows-msvc"

@REM Params paths
set "PATH_HEADERS=build\headers"
set "PATH_LIB_OUT=build\windows"

@REM Install rustup targets
for %%t in (%TARGETS%) do (
    rustup target add %%t
)

@REM Build headers
mkdir "%PATH_HEADERS%" 2>nul
for %%t in (%TARGETS%) do (
    cargo run --target %%t --%TYPE% --features headers --bin ydx_windows-headers
)

@REM Build cdylib libs
mkdir "%PATH_HEADERS%" 2>nul
for %%t in (%TARGETS%) do (
    cargo build --target %%t --%TYPE% -p ydx-windows
)

@REM Copy dynamic library
for %%t in (%TARGETS%) do (
    if exist "%PATH_LIB_OUT%\%%t" rmdir /s /q "%PATH_LIB_OUT%\%%t"
    mkdir "%PATH_LIB_OUT%\%%t"
    copy /y "target\%%t\%TYPE%\*.dll" "%PATH_LIB_OUT%\%%t"
    if exist "..\apps\desktop\windows\YDX\Native\ydx_windows.dll" del /f /q "..\apps\desktop\windows\YDX\Native\ydx_windows.dll"
    mklink /H "..\apps\desktop\windows\YDX\Native\ydx_windows.dll" "%PATH_LIB_OUT%\%%t\ydx_windows.dll"
)
