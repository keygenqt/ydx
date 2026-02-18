#!/bin/bash

# Build core for application Linux / GJS

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Check system
if [[ ! "$OSTYPE" == "linux"* ]]; then
    echo "Error: Unsupported operating system. This script requires Linux!" >&2
    echo "Current OSTYPE: $OSTYPE" >&2
    exit 1
fi

# Check tools
if [[ -z $(g-ir-compiler --version 2> /dev/null) ]]; then
    echo "Install tools...".
    bash scripts/linux/tools.sh || exit
fi

# Params build
TYPE="release"
TARGETS=("x86_64-unknown-linux-gnu")

# Params paths
PATH_HEADERS="build/headers"
PATH_LIB_OUT="build/linux"

# Install rustup targets
for target in "${TARGETS[@]}"; do
    rustup target add $target
done

# Build headers
mkdir -p "$PATH_HEADERS"
for target in "${TARGETS[@]}"; do
    cargo run --target $target --$TYPE --features headers --bin ydx_linux-headers
    # Fix GTK-Doc warnings
    sed -i '/\/\*\* \\brief/,/\*\//d' "$PATH_HEADERS/ydx_linux.h"
    sed -i '/\/\*\* <No documentation available> \*\//d' "$PATH_HEADERS/ydx_linux.h"
    break
done

# Build cdylib libs
for target in "${TARGETS[@]}"; do
    cargo build --target $target --$TYPE --package ydx-linux
done

# Build typelib libs
for target in "${TARGETS[@]}"; do
    # Prepare build folder
    rm -rf "$PATH_LIB_OUT/$target" && mkdir -p "$PATH_LIB_OUT/$target"

    # Copy dynamic library
    cp target/$target/$TYPE/*.so "$PATH_LIB_OUT/$target"

    # Build gir
    g-ir-scanner --warn-all \
                --namespace=YdxLinux \
                --nsversion=1.0 \
                --library=ydx_linux \
                --library-path="$PATH_LIB_OUT/$target" \
                --include=GObject-2.0 \
                --output="$PATH_LIB_OUT/$target/YdxLinux-1.0.gir" \
                "$PATH_HEADERS/ydx_linux.h" || { exit 1; }

    # Build typelib
    g-ir-compiler "$PATH_LIB_OUT/$target/YdxLinux-1.0.gir" \
        -o "$PATH_LIB_OUT/$target/YdxLinux-1.0.typelib"
done
