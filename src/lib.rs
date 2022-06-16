// stfu clap
#![allow(deprecated)]
#![feature(asm_sym)]
#![feature(used_with_arg)]

use std::env;
use std::ffi::{OsStr, OsString};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

#[allow(dead_code)]
pub mod arg;
pub mod hooks;
pub mod init;
#[allow(dead_code)]
pub mod key;
pub mod options;
#[allow(dead_code)]
pub mod shared;
pub mod ui;
pub mod webhelper;

const fn new_path(path: &'static str) -> &'static Path {
    unsafe { &*(path as *const str as *const Path) }
}

fn var_os<K: AsRef<OsStr>>(key: K) -> OsString {
    env::var_os(key).unwrap_or_default()
}

fn var_split<K: AsRef<OsStr>>(key: K) -> Vec<PathBuf> {
    env::split_paths(&var_os(key)).collect()
}

const PREFIX: &Path = new_path("/usr/lib/esteem");

fn print_option(option: &str) {
    println!("esteem | enabled \x1b[38;5;1m{option}\x1b[m");
}

fn main() {
    let home_dir = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"));

    let data_dir = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home_dir.join(".local/share"));

    let data_dir = data_dir.join("esteem");

    let options = options::Options::parse();

    if env::var_os(key::ESTEEM_LOAD_UI).is_some() {
        use std::borrow::Cow;

        struct Args<'a>(Vec<Cow<'a, str>>);

        impl<'a> Args<'a> {
            pub const fn new() -> Self {
                Self(Vec::new())
            }

            pub fn push<S>(&mut self, arg: S) -> &mut Self
            where
                S: Into<Cow<'a, str>>,
            {
                self.0.push(arg.into());
                self
            }
        }

        let mut args = Args::new();

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

        if let Some(game) = options.game {
            args.push(format!("steam://rungameid/{game}"));
        } else {
            //args.push("steam://open/largegameslist");
        }

        println!("esteem | load \x1b[38;5;2msteamui.so\x1b[m");

        let ui = ui::SteamUI::open().expect("steamui.so failed to load");

        ui.main(args.0.iter().map(|s| s.as_ref()));
    } else {
        let i686 = PREFIX.join("i686");
        let esteem = i686.join("libesteem.so");

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

            command
                .arg0(esteem.clone())
                .args(args.next())
                .current_dir(&PREFIX)
                .env(key::ESTEEM_LOAD_UI, "1")
                .env("CPU_MHZ", "3799.960")
                .env("LD_PRELOAD", esteem.clone())
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
