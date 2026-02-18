#!/bin/bash

# Build core for application harmonyOS

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Check system
if [[ ! "$OSTYPE" == "darwin"* ]]; then
    echo "Error: Unsupported operating system. This script requires macOS!" >&2
    echo "Current OSTYPE: $OSTYPE" >&2
    exit 1
fi

# Params build
TYPE="release"
TARGETS=("aarch64-unknown-linux-ohos")

# Params paths
PATH_HEADERS="build/headers"
PATH_LIB_OUT="build/harmonyos"

# Install rustup targets
for target in "${TARGETS[@]}"; do
    rustup target add $target
done

# Build headers
mkdir -p "$PATH_HEADERS"
cargo run --target aarch64-apple-darwin --$TYPE --features headers --bin ydx_harmonyos-headers

# Build cdylib libs
for target in "${TARGETS[@]}"; do
    cargo build --target $target --$TYPE -p ydx-harmonyos
done

# Build typelib libs
for target in "${TARGETS[@]}"; do
    # Prepare build folder
    rm -rf "$PATH_LIB_OUT/$target" && mkdir -p "$PATH_LIB_OUT/$target"
    # Copy static library
    cp target/$target/$TYPE/*.a "$PATH_LIB_OUT/$target"
    # Copy header
    cp "$PATH_HEADERS/ydx_harmonyos.h" "$PATH_LIB_OUT/$target"
done
