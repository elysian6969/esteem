use std::ffi::OsStr;
use std::io::ErrorKind;
use std::os::unix;
use std::path::Path;
use std::{fs, io};

// re-export so we can just fs::creatr_dir_all
pub use fs::create_dir_all;

pub fn copy_from<F, T, N>(from_dir: F, to_dir: T, name: N) -> io::Result<()>
where
    F: AsRef<Path>,
    T: AsRef<Path>,
    N: AsRef<OsStr>,
{
    let name = name.as_ref();
    let from = from_dir.as_ref().join(name);
    let to = to_dir.as_ref().join(name);

    fs::copy(from, to)?;

    Ok(())
}

pub fn symlink<F, T>(from: F, to: T) -> io::Result<()>
where
    F: AsRef<OsStr>,
    T: AsRef<OsStr>,
{
    let from = from.as_ref();
    let to = to.as_ref();

    let result = match unix::fs::symlink(to, from) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            fs::remove_file(from).and_then(|_| unix::fs::symlink(to, from))
        }
        Err(error) => Err(error),
    };

    result
}
