#!/bin/sh

# binaries
cargo build --package esteem --target i686-unknown-linux-gnu --
cargo build --package esteem-error-reporter --
cargo build --package esteem-reaper --
cargo build --package esteem-zenity --

# libs
cargo build --package pipewire --target i686-unknown-linux-gnu --
cargo build --package nm --target i686-unknown-linux-gnu --
cargo build --package tier0_s --target i686-unknown-linux-gnu --
cargo build --package video --target i686-unknown-linux-gnu --
cargo build --package vstdlib_s --target i686-unknown-linux-gnu --
