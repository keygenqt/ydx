#!/bin/bash

# Build core all on Linux

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

# Is Linux
if [[ "$OSTYPE" == "linux"* ]]; then
    bash scripts/android/build.sh
    # bash scripts/auroraos/build.sh
    bash scripts/linux/build.sh
    bash scripts/web/build.sh
fi

# Is macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    bash scripts/android/build.sh
    bash scripts/auroraos/build.sh
    bash scripts/harmonyos/build.sh
    bash scripts/ios/build.sh
    bash scripts/macos/build.sh
    bash scripts/web/build.sh
fi

# Is Windows
if [[ "$OSTYPE" == "msys"* ]] || [[ "$OSTYPE" == "cygwin"* ]] || [[ "$OSTYPE" == "win32"* ]] || [[ "$OSTYPE" == "win"* ]]; then
    echo "To build a Windows library (dll), use windows/build.but" >&2
    exit 1
fi
