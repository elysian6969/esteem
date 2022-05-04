use std::env;
use std::path::{Path, PathBuf};

/// Returns the environment variable `HOME` or `/`.
#[inline]
pub fn home_dir() -> PathBuf {
    env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"))
}

/// Returns the environment variable `XDG_DATA_HOME/Steam` or `{home}/.local/share/Steam`.
#[inline]
pub fn data_dir<P>(home: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let home = home.as_ref().to_path_buf();
    let data_dir = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));

    data_dir.join("Steam")
}
