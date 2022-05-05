<h1 align="center"><code>esteem</code></h1>

attempting to figure out how steam works

copy libraries and binaries from an existing steam installation (`$XDG_DATA_HOME/Steam`)

```shell
$ cargo copy
```

compile esteem

```shell
$ cargo build-esteem
```

copy esteem to `./lib/i686` (required for steamwebhelper to not break, it resolves it's path based on esteems location)

```shell
$ cp ${CARGO_TARGET_DIR:target}/i686-unknown-linux-gnu/release/esteem ./lib/i686/esteem
```

run esteem

```shell
$ cd lib
$ apulse esteem --no-browser --no-sandbox --zenity /milk/global/zenity
```

### thanks

 - [bare minimum steam client bootstrap (windows only)](https://gist.github.com/he1a2s0/a99be14877a83a96ee72f8538c582bf7)
 - [saskenuba/steamhelper-rs](https://github.com/saskenuba/steamhelper-rs)
 - [steamdatabase/steamtracking](https://github.com/steamdatabase/steamtracking)
 - [steamworks api reference](https://partner.steamgames.com/doc/api)
 - [swagsoftware/kisak-strike/public/steam](https://github.com/swagsoftware/kisak-strike/tree/master/public/steam)
