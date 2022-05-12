<h1 align="center"><code>esteem</code></h1>

attempting to figure out how steam works

copy your steam install to `/usr/lib/esteem`

compile components

```bash
# steam bootstrapper (required)
cargo build-esteem

# steam error reporter (recommended)
cargo build-error-reporter

# steam reaper (launch env (options soontm) from $XDG_DATA_DIR/esteem/launch_options.toml)
cargo build-reaper

# gnome's zenity cli replacement (optional)
cargo build-zenity

# networkmanager stub (optional)
cargo build-libnm

# pipewire stub (optional)
cargo build-libpipewire
```

copy components


```bash
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/esteem /usr/lib/i686/esteem
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/esteem-error-reporter /usr/lib/i686/steamerrorreporter
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/esteem-reaper /usr/lib/i686/reaper

cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libnm.so /usr/lib/i686/libnm.so
cp ${CARGO_TARGET_DIR:-target}/i686-unknown-linux-gnu/release/libpipewire.so /usr/lib/i686/libpipewire.so
```

### thanks

 - [bare minimum steam client bootstrap (windows only)](https://gist.github.com/he1a2s0/a99be14877a83a96ee72f8538c582bf7)
 - [saskenuba/steamhelper-rs](https://github.com/saskenuba/steamhelper-rs)
 - [steamdatabase/steamtracking](https://github.com/steamdatabase/steamtracking)
 - [steamworks api reference](https://partner.steamgames.com/doc/api)
 - [swagsoftware/kisak-strike/public/steam](https://github.com/swagsoftware/kisak-strike/tree/master/public/steam)
