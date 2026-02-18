#!/bin/bash

# Build core for application Wasm

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Check system
if [[ ! "$OSTYPE" == "linux"* ]] && [[ ! "$OSTYPE" == "darwin"* ]]; then
    echo "Error: Unsupported operating system. This script requires Linux or macOS!" >&2
    echo "Current OSTYPE: $OSTYPE" >&2
    exit 1
fi

# Check tools
if [[ -z $(wasm-pack --version 2> /dev/null) ]]; then
    echo "Install tools...".
    if [[ ! "$OSTYPE" == "linux"* ]]; then
        bash scripts/linux/tools_linux.sh || exit
    fi
    if [[ ! "$OSTYPE" == "darwin"* ]]; then
        bash scripts/linux/tools_macos.sh || exit
    fi
fi

# Params build
TYPE="release"
TARGETS=("web")

# Params paths
PATH_LIB_OUT="build"

for target in "${TARGETS[@]}"; do
    # Prepare build folder
    rm -rf "$PATH_LIB_OUT/$target" && mkdir -p "$PATH_LIB_OUT"
    # Build cdylib libs
    wasm-pack build ydx_wasm --target $target --$TYPE
    # Move
    mv ydx_wasm/pkg "$PATH_LIB_OUT/$target"
done
