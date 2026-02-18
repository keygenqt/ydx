#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

# Remove old build
rm -rf libs
mkdir libs

##################################
##################################
# Tools
cargo install cross --git https://github.com/cross-rs/cross
cargo install wasm-pack
# if [[ "$OSTYPE" == "linux"* ]]; then
#     # sudo apt install libgirepository1.0-dev gobject-introspection libglib2.0-dev
# fi

##################################
##################################
# Android
rustup target add armv7-linux-androideabi   # for arm
rustup target add i686-linux-android        # for x86
rustup target add aarch64-linux-android     # for arm64
rustup target add x86_64-linux-android      # for x86_64

# Builds from Gradle - ./gradlew cargoBuild or task preBuild

##################################
##################################
# Aurora
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
cross run --target x86_64-unknown-linux-gnu --features headers --bin core-headers

# arm64
mkdir -p libs/linuxArm64
cross build --target aarch64-unknown-linux-gnu --release
cp target/aarch64-unknown-linux-gnu/release/libcore.a libs/linuxArm64
cp libs/libcore.h libs/linuxArm64

# x64
mkdir -p libs/linuxX64
cross build --target x86_64-unknown-linux-gnu --release
cp target/x86_64-unknown-linux-gnu/release/libcore.a libs/linuxX64
cp libs/libcore.h libs/linuxX64

# remove common header
rm libs/libcore.h


##################################
##################################
# harmonyOS
if [[ "$OSTYPE" == "darwin"* ]]; then

rustup target add aarch64-unknown-linux-ohos
cross run --target x86_64-unknown-linux-gnu --features headers --bin core-headers

mkdir -p libs/ohosArm64
cargo build --target aarch64-unknown-linux-ohos --release
cp target/aarch64-unknown-linux-ohos/release/libcore.a libs/ohosArm64
cp libs/libcore.h libs/ohosArm64

# remove common header
rm libs/libcore.h

fi

##################################
##################################
# macOS & iOS
if [[ "$OSTYPE" == "darwin"* ]]; then

##################################
# macOS
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
cargo run --target aarch64-apple-darwin --features headers --bin core-headers

# arm64
mkdir -p libs/darwinArm64
cargo build --target aarch64-apple-darwin --release
cp target/aarch64-apple-darwin/release/libcore.a libs/darwinArm64
cp libs/libcore.h libs/darwinArm64
cat > "libs/darwinArm64/module.modulemap" << EOF
module CoreYDX {
    header "libcore.h"
    export *
}
EOF

# x64
mkdir -p libs/darwinX64
cargo build --target x86_64-apple-darwin --release
cp target/x86_64-apple-darwin/release/libcore.a libs/darwinX64
cp libs/libcore.h libs/darwinX64
cat > "libs/darwinX64/module.modulemap" << EOF
module CoreYDX {
    header "libcore.h"
    export *
}
EOF

##################################
# iOS
rustup target add aarch64-apple-ios-sim
rustup target add aarch64-apple-ios

# arm64 simulator
mkdir -p libs/iosSimArm64
cargo build --target aarch64-apple-ios-sim --release
cp target/aarch64-apple-ios-sim/release/libcore.a libs/iosSimArm64
cp libs/libcore.h libs/iosSimArm64
cat > "libs/iosSimArm64/module.modulemap" << EOF
module CoreYDX {
    header "libcore.h"
    export *
}
EOF

# arm64
mkdir -p libs/iosArm64
cargo build --target aarch64-apple-ios --release
cp target/aarch64-apple-ios/release/libcore.a libs/iosArm64
cp libs/libcore.h libs/iosArm64
cat > "libs/iosArm64/module.modulemap" << EOF
module CoreYDX {
    header "libcore.h"
    export *
}
EOF

# remove common header
rm libs/libcore.h

fi

##################################
##################################
# Linux / GJS
if [[ "$OSTYPE" == "linux"* ]]; then

rustup target add x86_64-unknown-linux-gnu
cross run --target x86_64-unknown-linux-gnu --features headers --bin core-headers

# x64
mkdir -p libs/typelibX64
cross build --target x86_64-unknown-linux-gnu --release
cp target/x86_64-unknown-linux-gnu/release/libcore.so libs/typelibX64
cp libs/libcore.h libs/typelibX64

g-ir-scanner --namespace=core \
             --nsversion=1.0 \
             --library=core \
             --library-path=./libs/typelibX64 \
             --include=GObject-2.0 \
             --output=./libs/typelibX64/core-1.0.gir \
             ./libs/typelibX64/libcore.h

g-ir-compiler ./libs/typelibX64/core-1.0.gir \
    -o ./libs/typelibX64/core-1.0.typelib

# remove common header
rm libs/libcore.h

fi

##################################
##################################
# Wasm
wasm-pack build --target web --release
