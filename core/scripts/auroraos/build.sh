#!/bin/bash

# Build core for application Aurora OS

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Check system
if [[ ! "$OSTYPE" == "linux"* ]] && [[ ! "$OSTYPE" == "darwin"* ]]; then
    echo "Error: Unsupported operating system. This script requires Linux or macOS!" >&2
    echo "Current OSTYPE: $OSTYPE" >&2
    exit 1
fi

# Check tools
if [[ -z $(cross --version 2> /dev/null) ]]; then
    echo "Install tools..."
    if [[ ! "$OSTYPE" == "linux"* ]]; then
        bash scripts/linux/tools_linux.sh || exit
    fi
    if [[ ! "$OSTYPE" == "darwin"* ]]; then
        bash scripts/linux/tools_macos.sh || exit
    fi
fi

# Params build
TYPE="release"
TARGETS=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "armv7-unknown-linux-gnueabihf")

# Params paths
PATH_HEADERS="build/headers"
PATH_LIB_OUT="build/auroraos"

# Install rustup targets
for target in "${TARGETS[@]}"; do
    rustup target add $target
done

# Build headers
mkdir -p "$PATH_HEADERS"
for target in "${TARGETS[@]}"; do
    cross run --target $target --$TYPE --features headers --bin ydx_auroraos-headers
    break
done

# Build cdylib libs
for target in "${TARGETS[@]}"; do
    cross build --target $target --$TYPE -p ydx-auroraos
done

# Build typelib libs
for target in "${TARGETS[@]}"; do
    # Prepare build folder
    rm -rf "$PATH_LIB_OUT/$target" && mkdir -p "$PATH_LIB_OUT/$target"
    # Copy dynamic library
    cp target/$target/$TYPE/*.a "$PATH_LIB_OUT/$target"
    # Copy header
    cp "$PATH_HEADERS/ydx_auroraos.h" "$PATH_LIB_OUT/$target"
done
