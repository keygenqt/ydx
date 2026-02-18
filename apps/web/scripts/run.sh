#!/bin/bash

############################
## Build and Run application
############################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../../";

# Check system
if [[ ! "$OSTYPE" == "linux"* ]] && [[ ! "$OSTYPE" == "darwin"* ]]; then
    echo "Error: Unsupported operating system. This script requires Linux or macOS!" >&2
    echo "Current OSTYPE: $OSTYPE" >&2
    exit 1
fi

# Build lib
bash core/scripts/web/build.sh

# Open dir project
cd apps/web

# Update dependency
npm install ../../core/build/web
npm update

# Run application
npm run dev
