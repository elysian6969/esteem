#!/bin/sh

./build_all.sh

# binaries
sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/esteem /usr/lib/esteem/i686/esteem
sesh cp -v ${CARGO_TARGET_DIR:-target}/release/esteem-error-reporter /usr/lib/esteem/i686/steamerrorreporter
sesh cp -v ${CARGO_TARGET_DIR:-target}/release/esteem-reaper /usr/lib/esteem/i686/reaper
sesh cp -v ${CARGO_TARGET_DIR:-target}/release/esteem-zenity /usr/lib/esteem/i686/zenity

# libraries
sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libnm.so /usr/lib/esteem/i686/libnm.so.0
sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libpipewire.so /usr/lib/esteem/i686/libpipewire-0.3.so.0
# segmentation fault
# sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libtier0_s.so /usr/lib/esteem/i686/libtier0_s.so
sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libvideo.so /usr/lib/esteem/i686/libvideo.so
# segmentation fault
# sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libvstdlib_s.so /usr/lib/esteem/i686/libvstdlib_s.so
