#!/bin/bash

ROOT="$(dirname $0)"

cd $ROOT

DEFAULT_TARGET_DIR="${ROOT}/target"
TARGET_DIR="${CARGO_TARGET_DIR:-$DEFAULT_TARGET_DIR}"

cargo build --jobs=8 --package=steam-bootstrap --release --target=i686-unknown-linux-gnu

cp -av "${TARGET_DIR}/i686-unknown-linux-gnu/release/steam-bootstrap" "${ROOT}/lib/i686/bootstrap"
