@echo off

@REM Build core for application dll

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

@REM @todo
@REM @REM Build headers
@REM mkdir "%PATH_HEADERS%" 2>nul
@REM for %%t in (%TARGETS%) do (
@REM     cargo run --target %%t --%TYPE% --features headers --bin ydx_windows-headers
@REM )

@REM @REM Build cdylib libs
@REM mkdir "%PATH_HEADERS%" 2>nul
@REM for %%t in (%TARGETS%) do (
@REM     cargo build --target %%t --%TYPE% -p ydx_windows
@REM )

@REM Copy dynamic library
for %%t in (%TARGETS%) do (
    if exist "%PATH_LIB_OUT%\%%t" rmdir /s /q "%PATH_LIB_OUT%\%%t"
    mkdir "%PATH_LIB_OUT%\%%t"
    copy /y "target\%%t\%TYPE%\*.dll" "%PATH_LIB_OUT%\%%t"
    copy /y "%PATH_HEADERS%\ydx_windows.h" "%PATH_LIB_OUT%\%%t\"
    if exist "%PATH_HEADERS%" rmdir /s /q "%PATH_HEADERS%"
    if exist "..\apps\desktop\windows\YDX\Native\ydx_windows.dll" del /f /q "..\apps\desktop\windows\YDX\Native\ydx_windows.dll"
    mklink /H "..\apps\desktop\windows\YDX\Native\ydx_windows.dll" "%PATH_LIB_OUT%\%%t\ydx_windows.dll"
)
