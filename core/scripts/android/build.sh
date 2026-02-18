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

# Install rustup targets
rustup target add armv7-linux-androideabi   # for arm
rustup target add i686-linux-android        # for x86
rustup target add aarch64-linux-android     # for arm64
rustup target add x86_64-linux-android      # for x86_64

# Build with Gradle
# The Android plugin automatically connects libraries
# https://github.com/mozilla/rust-android-gradle
if [[ "$OSTYPE" == "linux"* ]]; then
    # @todo
else
    cd ../apps/mobile/android && ./gradlew cargoBuild \
        -Prust.cargoCommand="$HOME/.cargo/bin/cargo" \
        -Prust.rustcCommand="$HOME/.cargo/bin/rustc" \
        -Prust.pythonCommand='/opt/homebrew/bin/python3'
fi
