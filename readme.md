<h1 align="center"><code>esteem</code></h1>

attempting to figure out how steam works

copy your steam install to `/usr/lib/esteem`

rename `ubuntu12_32` to `i686` (may be broken, may require a symlink, also yet to rename `ubuntu12_64`)

yet to make some actual installer, for now cope with

```bash
./install_all.sh
```

copy components


```bash
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/esteem /usr/lib/i686/esteem
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/esteem-error-reporter /usr/lib/i686/steamerrorreporter
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/esteem-reaper /usr/lib/i686/reaper

cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libnm.so /usr/lib/i686/libnm.so
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libpipewire.so /usr/lib/i686/libpipewire.so
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libvideo.so /usr/lib/i686/libvideo.so
```

### thanks

 - [bare minimum steam client bootstrap (windows only)](https://gist.github.com/he1a2s0/a99be14877a83a96ee72f8538c582bf7)
 - [saskenuba/steamhelper-rs](https://github.com/saskenuba/steamhelper-rs)
 - [steamdatabase/steamtracking](https://github.com/steamdatabase/steamtracking)
 - [steamworks api reference](https://partner.steamgames.com/doc/api)
 - [swagsoftware/kisak-strike/public/steam](https://github.com/swagsoftware/kisak-strike/tree/master/public/steam)
