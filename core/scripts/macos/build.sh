#!/bin/bash

# Build core for application macOS

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
TARGETS=("aarch64-apple-darwin" "x86_64-apple-darwin")

# Params paths
PATH_HEADERS="build/headers"
PATH_LIB_OUT="build/macos"

# Install rustup targets
for target in "${TARGETS[@]}"; do
    rustup target add $target
done

# Build headers
mkdir -p "$PATH_HEADERS"
for target in "${TARGETS[@]}"; do
    cargo run --target $target --$TYPE --features headers --bin ydx_macos-headers
    break
done

# Build cdylib libs
for target in "${TARGETS[@]}"; do
    cargo build --target $target --$TYPE -p ydx-macos
done

# Build typelib libs
for target in "${TARGETS[@]}"; do
    # Prepare build folder
    rm -rf "$PATH_LIB_OUT/$target" && mkdir -p "$PATH_LIB_OUT/$target"
    # Copy static library
    cp target/$target/$TYPE/*.a "$PATH_LIB_OUT/$target"
    # Copy header
    cp "$PATH_HEADERS/ydx_macos.h" "$PATH_LIB_OUT/$target"
    # Create modulemap
    cat > "$PATH_LIB_OUT/$target/module.modulemap" << EOF
module YdxMacos {
    header "ydx_macos.h"
    export *
}
EOF
done
