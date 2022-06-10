#!/bin/sh

./build_all_debug.sh

# binaries
sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/debug/esteem /usr/lib/esteem/i686/esteem
sesh cp -v ${CARGO_TARGET_DIR:-target}/debug/esteem-error-reporter /usr/lib/esteem/i686/steamerrorreporter
sesh cp -v ${CARGO_TARGET_DIR:-target}/debug/esteem-reaper /usr/lib/esteem/i686/reaper
sesh cp -v ${CARGO_TARGET_DIR:-target}/debug/esteem-zenity /usr/lib/esteem/i686/zenity

# libraries
sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/debug/libnm.so /usr/lib/esteem/i686/libnm.so.0
sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/debug/libpipewire.so /usr/lib/esteem/i686/libpipewire-0.3.so.0
#sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/debug/libtier0_s.so /usr/lib/esteem/i686/libtier0_s.so
sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/debug/libvideo.so /usr/lib/esteem/i686/libvideo.so
#sesh cp -v ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/debug/libvstdlib_s.so /usr/lib/esteem/i686/libvstdlib_s.so
