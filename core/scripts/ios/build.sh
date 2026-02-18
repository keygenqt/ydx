#!/bin/bash

# Build core for application iOS

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
TARGETS=("aarch64-apple-ios-sim" "aarch64-apple-ios")

# Params paths
PATH_HEADERS="build/headers"
PATH_LIB_OUT="build/ios"

# Install rustup targets
for target in "${TARGETS[@]}"; do
    rustup target add $target
done

# Build headers
mkdir -p "$PATH_HEADERS"
cargo run --target aarch64-apple-darwin --$TYPE --features headers --bin ydx_ios-headers

# Build cdylib libs
for target in "${TARGETS[@]}"; do
    cargo build --target $target --$TYPE -p ydx-ios
done

# Build typelib libs
for target in "${TARGETS[@]}"; do
    # Prepare build folder
    rm -rf "$PATH_LIB_OUT/$target" && mkdir -p "$PATH_LIB_OUT/$target"
    # Copy static library
    cp target/$target/$TYPE/*.a "$PATH_LIB_OUT/$target"
    # Copy header
    cp "$PATH_HEADERS/ydx_ios.h" "$PATH_LIB_OUT/$target"
    # Create modulemap
    cat > "$PATH_LIB_OUT/$target/module.modulemap" << EOF
module YdxIos {
    header "ydx_ios.h"
    export *
}
EOF
done
