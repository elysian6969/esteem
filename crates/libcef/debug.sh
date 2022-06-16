#!/bin/sh

export LD_LIBRARY_PATH=/usr/lib/esteem/x86_64

"/usr/lib/esteem/x86_64/steamwebhelper" \
    "-buildid=0" \
    "-cachedir=/ely/data/esteem/cef/cache" \
    "--disable-blink-features=Badging" \
    "--enable-blink-features=AudioWorklet,ResizeObserver,Worklet" \
    "--disable-gpu-watchdog" \
    "--disable-hang-monitor" \
    "/usr/lib/esteem/steamui/index.html" \
    "-lang=en_US" \
    "-logdir=/ely/data/esteem/cef/log" \
    "--log-file=/ely/data/esteem/cef/log.txt" \
    "--enable-media-stream" \
    "-realm=global" \
    "--disable-quick-menu" \
    "--no-sandbox" \
    "--disable-seccomp-filter-sandbox" \
    "--disable-smooth-scrolling" \
    "-steampid=22173" \
    "-steamuniverse=Dev"
