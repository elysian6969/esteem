use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::ffi::OsStr;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

mod key;

pub mod env2 {
    use std::env;
    use std::path::{Path, PathBuf};

    /// Returns the environment variable `HOME` or `/`.
    #[inline]
    pub fn home_dir() -> PathBuf {
        env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/"))
    }

    /// Returns the environment variable `XDG_DATA_HOME/elysh` or `{home}/.local/share/elysh`.
    #[inline]
    pub fn data_dir<P>(home: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let home = home.as_ref().to_path_buf();
        let data_dir = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".local/share"));

        data_dir.join("esteem")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Options {
    vars: Vec<String>,
}

fn main() {
    let data_dir = env2::home_dir();
    let launch_options = data_dir.join("launch_options.toml");
    let bytes = fs::read(&launch_options).expect("launch_options.toml");
    let options: BTreeMap<String, Options> = toml::from_slice(&bytes).expect("toml");

    let show_removed_vars = env::var_os("REAPER_SHOW_REMOVED_VARS").is_some();
    let show_vars = env::var_os("REAPER_SHOW_VARS").is_some();
    let key_filter = key::KEY_FILTER
        .into_iter()
        .map(|key| OsStr::new(key))
        .collect::<HashSet<&'static OsStr>>();

    println!("esteem | environment");

    for (key, val) in env::vars_os().collect::<BTreeMap<_, _>>() {
        if !key_filter.contains(&key.as_os_str()) {
            if show_removed_vars {
                println!("esteem | \x1b[38;5;1m{key:?}\x1b[m=\x1b[38;5;1m{val:?}\x1b[m");
            }

            env::remove_var(key);

            continue;
        }

        if show_vars {
            println!("esteem | \x1b[38;5;2m{key:?}\x1b[m=\x1b[38;5;2m{val:?}\x1b[m");
        }
    }

    println!("esteem | command line");
    print!("esteem | ");

    for arg in env::args_os() {
        print!("\x1b[38;5;2m{arg:?}\x1b[m ");
    }

    println!();

    let args = env::args_os().collect::<Vec<_>>();
    let skip = args.iter().position(|arg| arg == "--").unwrap_or(0) + 1;
    let rest = args.iter().skip(skip).collect::<Vec<_>>();
    let program = rest.first().unwrap();
    let args = rest.iter().skip(1);
    let mut command = Command::new(program);

    let path = Path::new(&program);
    let name = path.file_name().unwrap();
    let name = name.to_str().unwrap();

    if let Some(options) = options.get(name) {
        //let linux32 = path.with_file_name("bin");
        let linux64 = path.with_file_name("bin/linux64");

        // fixes csgo being unable to find it's own libraries?
        env::set_var(
            "LD_LIBRARY_PATH",
            format!("{}:/usr/lib/esteem/i686", linux64.display()),
        );

        println!("esteem | launch options");

        let vars = options.vars.iter().flat_map(|var| var.split_once('='));

        for (key, val) in vars {
            println!("esteem | \x1b[38;5;2m{key:?}\x1b[m=\x1b[38;5;2m{val:?}\x1b[m");
            command.env(key, val);
        }
    }

    println!("esteem | running game");

    let _ = command.args(args).exec();
}
