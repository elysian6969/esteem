#![feature(used_with_arg)]

use std::collections::BTreeMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

mod arg;
mod key;
mod options;
mod shared;
mod ui;

const fn new_path(path: &'static str) -> &'static Path {
    unsafe { &*(path as *const str as *const Path) }
}

fn var_os<K: AsRef<OsStr>>(key: K) -> OsString {
    env::var_os(key).unwrap_or_default()
}

fn var_split<K: AsRef<OsStr>>(key: K) -> Vec<PathBuf> {
    env::split_paths(&var_os(key)).collect()
}

//const PREFIX: &Path = new_path(env!("CARGO_MANIFEST_DIR"));
const PREFIX: &Path = new_path("/usr/lib/esteem");

fn print_option(option: &str) {
    println!("   - \x1b[38;5;1m{option}\x1b[m");
}

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

fn main() {
    let options = options::Options::parse();

    if env::var_os(key::ESTEEM_LOAD_UI).is_some() {
        let mut args = Vec::new();

        // crash reporter
        args.push(arg::CEF_DISABLE_BREAKPAD);
        // dont need this
        args.push(arg::CEF_DISABLE_HANG_TIMEOUTS);
        // cringe
        args.push(arg::DISABLE_PARTNER_LICENSES);
        // cringe
        args.push(arg::NO_CRASH_MONITOR);
        // definitely dont need this
        args.push(arg::NO_INTRO);

        println!(" > running steam with: ");

        if options.devtools {
            print_option("devtools");

            args.push(arg::OPEN_DEV_TOOLS);
            args.push(arg::OPEN_OVERLAY_DEV_TOOLS);
        }

        if options.more_log {
            print_option("more-log");

            args.push(arg::FS_LOG);
            args.push(arg::FS_LOG_BINS);
            args.push(arg::LOG_NET_API);
        }

        if options.no_browser {
            print_option("no-browser");

            args.push(arg::NO_BROWSER);
        }

        if options.no_sandbox {
            print_option("no-sandbox");

            args.push(arg::CEF_DISABLE_SANDBOX);
            args.push(arg::NO_CEF_SANDBOX);
            args.push(arg::NO_SANDBOX);
        }

        if options.no_vr {
            print_option("no-vr");

            args.push(arg::VR_SKIP);
        }

        println!(" > loading \x1b[38;5;2msteamui.so\x1b[m");

        let ui = ui::SteamUI::open().expect("steamui.so failed to load");

        ui.main(args);
    } else {
        let i686 = PREFIX.join("i686");
        let esteem = i686.join("esteem");

        let mut path = var_split(key::PATH);
        let mut ld_library_path = var_split(key::LD_LIBRARY_PATH);

        path.insert(0, i686.clone());
        ld_library_path.insert(0, i686);

        let path = env::join_paths(path).expect("PATH");
        let ld_library_path = env::join_paths(ld_library_path).expect("LD_LIBRARY_PATH");
        let mut args = env::args_os();

        if let Some(program) = args.next() {
            let mut command = Command::new(program);

            // we set SYSTEM_LD_LIBRARY_PATH and SYSTEM_PATH because steamui.so does the following:
            //  __stream = popen("LD_LIBRARY_PATH=\"$SYSTEM_LD_LIBRARY_PATH\" PATH=\"$SYSTEM_PATH\" lspci -mm -n", "r");

            let home_dir = env2::home_dir();
            let data_dir = env2::data_dir(home_dir);

            command
                .arg0(esteem)
                .args(env::args_os().skip(1))
                .current_dir(&PREFIX)
                .env(key::ESTEEM_LOAD_UI, "1")
                .env(key::LD_LIBRARY_PATH, &ld_library_path)
                .env(key::HOME, data_dir)
                .env(key::PATH, &path)
                .env(key::STEAM_ZENITY, options.zenity)
                .env(key::SYSTEM_LD_LIBRARY_PATH, &ld_library_path)
                .env(key::SYSTEM_PATH, &path)
                .exec();
        }
    }
}
