#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Install tools
sudo apt install libgirepository1.0-dev gobject-introspection libglib2.0-dev
