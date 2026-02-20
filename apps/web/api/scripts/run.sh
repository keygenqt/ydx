#!/bin/bash

############################
## Build and Run application
############################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

# Init config
bash scripts/init.sh

# Run application
cargo run -- start --config ../docker/configs/api.conf
