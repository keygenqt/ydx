#!/bin/bash

############################
## Build and Run application
############################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

# Linux / GJS
if [[ ! "$OSTYPE" == "linux"* ]]; then
    echo "Error: OSTYPE not Linux!" >&2
    exit 1
fi

# Check tools
if [[ -z $(meson --version 2> /dev/null) ]]; then
    echo "Install tools...".
    source scripts/tools.sh || exit
fi

# Build lib @todo
# bash ../../../core/scripts/linux/build.sh || exit 1

# Export lib paths
CORE_LIB_PATH="$PWD/../../../core/build/linux/x86_64-unknown-linux-gnu"
if [[ -d "$CORE_LIB_PATH" ]]; then
    echo "Exporting library paths..."
    export GI_TYPELIB_PATH="$GI_TYPELIB_PATH:$CORE_LIB_PATH"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CORE_LIB_PATH"
else
    echo "Warning: Rust library not found at $CORE_LIB_PATH"
fi

# Build application
FOLDER='build'
echo "Setting up build directory..."
meson setup \
    --prefix=/home/$USER/.local \
    --bindir=/home/$USER/.local/bin \
    --datadir=/home/$USER/.local/share \
    --libdir=/home/$USER/.local/lib \
    --buildtype=debugoptimized \
    $FOLDER

# Install application
echo "Building and installing..."
ninja -C $FOLDER install || { echo "Build failed!"; exit 1; }

# Run application
echo "Starting application..."
/home/$USER/.local/bin/com.keygenqt.ydx
