@echo off

title YDX API Server (WSL)
setlocal enabledelayedexpansion

@REM @todo
echo ===============================================
echo   YDX API Server (WSL)
echo ===============================================
echo.

@REM Set root dir (go up from scripts/windows/ to project root)
cd /d "%~dp0.." || (
    echo Failed to change directory.
    pause
    exit /b 1
)

set WSL_DISTRO=Ubuntu-24.04

@REM Check if Ubuntu is installed
wsl -d %WSL_DISTRO% -- echo "test" >nul 2>&1
if %errorlevel% neq 0 (
    echo.
    echo Ubuntu 24.04 is NOT installed!
    echo.
    echo To install it:
    echo   1. Open PowerShell as Administrator
    echo   2. Run: wsl --install -d Ubuntu-24.04
    echo.
    echo Official guide: https://ubuntu.com/tutorials/install-ubuntu-on-wsl2-on-windows-11-with-gui-support
    echo.
    echo After installation, run this script again.
    pause
    exit /b
)

@REM Check curl
wsl -d %WSL_DISTRO% -- bash -c "command -v curl" >nul 2>&1
if %errorlevel% neq 0 (
    echo Installing curl...
    wsl -d %WSL_DISTRO% -- sudo apt update
    wsl -d %WSL_DISTRO% -- sudo apt install -y curl
)

@REM Check build tools (gcc)
wsl -d %WSL_DISTRO% -- bash -c "command -v gcc" >nul 2>&1
if %errorlevel% neq 0 (
    echo Installing build tools...
    wsl -d %WSL_DISTRO% -- sudo apt install -y build-essential
)

@REM Check Rust (with proper PATH)
wsl -d %WSL_DISTRO% -- bash -c "source \$HOME/.cargo/env && command -v cargo" >nul 2>&1
if %errorlevel% neq 0 (
    echo Installing Rust...
    wsl -d %WSL_DISTRO% -- bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
)

@REM Check and setup PostgreSQL
wsl -d %WSL_DISTRO% -- bash -c "command -v psql" >nul 2>&1
if %errorlevel% neq 0 (
    echo Installing PostgreSQL...
    wsl -d %WSL_DISTRO% -- sudo apt install -y postgresql postgresql-contrib
    wsl -d %WSL_DISTRO% -- sudo service postgresql start
    wsl -d %WSL_DISTRO% -- sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'postgres';"
    wsl -d %WSL_DISTRO% -- sudo -u postgres createdb ydx_dev 2>nul || echo Database ydx_dev already exists
) else (
    echo PostgreSQL already installed, ensuring service is running...
    wsl -d %WSL_DISTRO% -- sudo service postgresql start
)

@REM Add to run.sh executable
echo Making script executable...
wsl -d %WSL_DISTRO% -- bash -c "chmod +x $(pwd)/scripts/run.sh"

@REM Convert all shell scripts to Unix format
echo Converting all scripts to Unix format...
wsl -d %WSL_DISTRO% -- bash -c "find $(pwd)/scripts -name '*.sh' -exec sed -i 's/\r$//' {} \;"
wsl -d %WSL_DISTRO% -- bash -c "find $(pwd)/../docker/scripts -name '*.sh' -exec sed -i 's/\r$//' {} \;"

@REM Run the server
echo Starting server...
echo.
wsl -d %WSL_DISTRO% -- bash -c "source \$HOME/.cargo/env && cd $(pwd) && bash scripts/run.sh"

if %errorlevel% neq 0 (
    echo.
    echo Failed to start. Check errors above.
)