#!/bin/bash

ROOT="$(dirname $0)"

cd $ROOT

DEFAULT_TARGET_DIR="${ROOT}/target"
TARGET_DIR="${CARGO_TARGET_DIR:-$DEFAULT_TARGET_DIR}"

cargo build --jobs=8 --package=pipewire --release --target=i686-unknown-linux-gnu
cargo build --jobs=8 --package=pipewire --release --target=x86_64-unknown-linux-gnu

cp -av "${TARGET_DIR}/i686-unknown-linux-gnu/release/libpipewire.so" "${ROOT}/lib/i686/libpipewire-0.3.so.0"
cp -av "${TARGET_DIR}/x86_64-unknown-linux-gnu/release/libpipewire.so" "${ROOT}/lib/x86_64/libpipewire-0.3.so.0"
