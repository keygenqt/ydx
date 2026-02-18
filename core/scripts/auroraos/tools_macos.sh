#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Install tools
cargo install cross --git https://github.com/cross-rs/cross
