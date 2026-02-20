#!/bin/bash

############################
## Build and Run application
############################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

# Init config
bash ../docker/scripts/init.sh

# Params dev config
CONFIG=(
    "s|/var\/www|$PWD|g"
    "s|postgres:5432|localhost:5432|g"
    "s|development = false|development = true|g"
)

# Set dev config
for value in "${CONFIG[@]}"; do
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "$value" ../docker/configs/api.conf
    else
        sed -i "$value" ../docker/configs/api.conf
    fi
done

# Done
echo -e "\033[1;32mInit dev successfully.\033[00m"
