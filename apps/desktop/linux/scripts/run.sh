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

# Build lib
bash ../../../core/scripts/linux/build.sh

# Export lib
export GI_TYPELIB_PATH=$GI_TYPELIB_PATH:$PWD/../../../core/build/linux/x86_64-unknown-linux-gnu
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$PWD/../../../core/build/linux/x86_64-unknown-linux-gnu

# Build application
FOLDER='build'
meson setup \
    -Dbindir=/home/$USER/.local/bin \
    -Ddatadir=/home/$USER/.local/share \
    -Dlibdir=/home/$USER/.local/lib \
    -Dprefix=/home/$USER/.local \
    $FOLDER

# Install application
ninja -C $FOLDER install

# Run application
/home/$USER/.local/bin/com.keygenqt.ydx
