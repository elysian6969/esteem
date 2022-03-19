#!/bin/bash

ROOT="$(dirname $0)"
STEAM_ROOT="$1"

rm -rfv "${ROOT}/lib"

mkdir -pv "${ROOT}/lib"
mkdir -pv "${ROOT}/lib/i686"
mkdir -pv "${ROOT}/lib/x86_64"

ln -sv "i686" "${ROOT}/lib/ubuntu12_32"
ln -sv "i686" "${ROOT}/lib/ubuntu12_64"

function extract_library() {
	cp -av "${STEAM_ROOT}/ubuntu12_32/${1}" "${ROOT}/lib/i686/${1}"
}

echo " * Extracting runtime libraries..."

cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/lib/i386-linux-gnu/libudev.so.0" "${ROOT}/lib/i686/libudev.so.0"
cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/lib/i386-linux-gnu/libudev.so.0.13.0" "${ROOT}/lib/i686/libudev.so.0.13.0"

cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libappindicator.so.1" "${ROOT}/lib/i686/libappindicator.so.1"
cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libappindicator.so.1.0.0" "${ROOT}/lib/i686/libappindicator.so.1.0.0"

cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libdbusmenu-glib.so.4" "${ROOT}/lib/i686/libdbusmenu-glib.so.4"
cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libdbusmenu-glib.so.4.0.13" "${ROOT}/lib/i686/libdbusmenu-glib.so.4.0.13"

cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libdbusmenu-gtk.so.4" "${ROOT}/lib/i686/libdbusmenu-gtk.so.4"
cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libdbusmenu-gtk.so.4.0.13" "${ROOT}/lib/i686/libdbusmenu-gtk.so.4.0.13"

cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libgtk-x11-2.0.so.0" "${ROOT}/lib/i686/libgtk-x11-2.0.so.0"
cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libgtk-x11-2.0.so.0.2400.10" "${ROOT}/lib/i686/libgtk-x11-2.0.so.0.2400.10"

cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libgudev-1.0.so.0" "${ROOT}/lib/i686/libgudev-1.0.so.0"
cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libgudev-1.0.so.0.1.1" "${ROOT}/lib/i686/libgudev-1.0.so.0.1.1"

cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libindicator.so.7" "${ROOT}/lib/i686/libindicator.so.7"
cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libindicator.so.7.0.0" "${ROOT}/lib/i686/libindicator.so.7.0.0"

cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libnm.so.0" "${ROOT}/lib/i686/libnm.so.0"
cp -av "${STEAM_ROOT}/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libnm.so.0.1.0" "${ROOT}/lib/i686/libnm.so.0.1.0"

echo " * Extracting libraries..."

extract_library "chromehtml.so"
extract_library "crashhandler.so"
extract_library "filesystem_stdio.so"
extract_library "friendsui.so"
extract_library "libtier0_s.so"
extract_library "libvideo.so"
extract_library "libvstdlib_s.so"
extract_library "serverbrowser.so"
extract_library "steamclient.so"
extract_library "steamservice.so"
extract_library "steamui.so"
extract_library "vgui2_s.so"
extract_library "libicui18n.so"
extract_library "libicuuc.so"
extract_library "libv8.so"
extract_library "libavcodec.so.58"
extract_library "libavfilter.so.7"
extract_library "libavformat.so.58"
extract_library "libavresample.so.4"
extract_library "libavutil.so.56"
extract_library "libswscale.so.5"
extract_library "libvpx.so.6"
extract_library "libopenvr_api.so"
extract_library "libSDL2-2.0.so.0"

echo " * Extracting binaries..."

extract_library "gldriverquery"
extract_library "vulkandriverquery"
