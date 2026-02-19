#!/bin/bash

#######################
## Run docker container
#######################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

# Init configuration
bash scripts/init.sh

# Remove docker containers
docker compose down

# Create docker containers
docker compose up --no-build -d

# Done
echo -e "\033[1;32mRun successfully.\033[00m"
