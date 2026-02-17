#!/bin/bash

##########################
## Build meson application
##########################

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Tools
# sudo apt install libglib2.0-dev gettext meson ninja-build

export GI_TYPELIB_PATH=$GI_TYPELIB_PATH:$PWD/../../shared/rust/libs/typelibX64
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$PWD/../../shared/rust/libs/typelibX64

FOLDER='build-dir'

meson setup \
    -Dbindir=/home/$USER/.local/bin \
    -Ddatadir=/home/$USER/.local/share \
    -Dlibdir=/home/$USER/.local/lib \
    -Dprefix=/home/$USER/.local \
    $FOLDER

ninja -C $FOLDER install

/home/$USER/.local/bin/com.keygenqt.DemoRust
