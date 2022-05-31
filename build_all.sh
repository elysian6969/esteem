#!/bin/sh

# binaries
cargo build --package esteem --target i686-unknown-linux-gnu --release --
cargo build --package esteem-error-reporter --release --
cargo build --package esteem-reaper --release --
cargo build --package esteem-zenity --release --

# libs
cargo build --package pipewire --target i686-unknown-linux-gnu --release --
cargo build --package nm --target i686-unknown-linux-gnu --release --
cargo build --package tier0_s --target i686-unknown-linux-gnu --release --
cargo build --package video --target i686-unknown-linux-gnu --release --
cargo build --package vstdlib_s --target i686-unknown-linux-gnu --release --
