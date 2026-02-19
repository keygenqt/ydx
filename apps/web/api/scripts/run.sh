#!/bin/bash

############################
## Build and Run application
############################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

# Init config
bash ../docker/scripts/init.sh

# Set localhost
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/postgres:5432/localhost:5432/' ../docker/configs/api.conf
else
    sed -i 's/postgres:5432/localhost:5432/' ../docker/configs/api.conf
fi

# Run application
cargo run -- start --config ../docker/configs/api.conf
