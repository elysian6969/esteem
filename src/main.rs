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

const PREFIX: &Path = new_path(env!("CARGO_MANIFEST_DIR"));

fn main() {
    if env::var_os(key::ESTEEM_LOAD_UI).is_some() {
        println!(" > loading steamui.so");

        let args: Vec<_> = env::args_os().skip(1).collect();
        let vars: BTreeMap<_, _> = env::vars_os().collect();

        println!("args = {args:?}");
        println!("vars = {vars:?}");

        let ui = ui::SteamUI::open().expect("steamui.so failed to load");

        ui.main(args);
    } else {
        let options = options::Options::parse();

        let lib = PREFIX.join("lib");
        let i686 = lib.join("i686");
        let esteem = lib.join("esteem");

        let path = var_split(key::PATH);
        let mut ld_library_path = var_split(key::LD_LIBRARY_PATH);

        ld_library_path.insert(0, i686);

        let path = env::join_paths(path).expect("PATH");
        let ld_library_path = env::join_paths(ld_library_path).expect("LD_LIBRARY_PATH");
        let mut args = env::args_os();

        if let Some(program) = args.next() {
            let mut command = Command::new(program);

            command
                .arg(arg::CEF_DISABLE_BREAKPAD)
                .arg(arg::CEF_DISABLE_HANG_TIMEOUTS)
                .arg(arg::DISABLE_PARTNER_LICENSES)
                .arg(arg::NO_SHARED_TEXTURES)
                .arg(arg::NO_BIG_PICTURE)
                .arg(arg::NO_CRASH_MONITOR)
                .arg(arg::NO_INTRO)
                .arg(arg::SKIP_STREAMING_DRIVERS)
                .arg(arg::WINDOWED);

            println!(" > running steam with");

            /*if options.cli {
                println!("   - cli");

                command.arg(arg::TEXT_CLIENT);
            }*/

            if options.devtools {
                println!("   - devtools");

                command
                    .arg(arg::OPEN_DEV_TOOLS)
                    .arg(arg::OPEN_OVERLAY_DEV_TOOLS);
            }

            if options.more_log {
                println!("   - more-log");

                command
                    .arg(arg::FS_LOG)
                    .arg(arg::FS_LOG_BINS)
                    .arg(arg::LOG_NET_API);
            }

            if options.no_browser {
                println!("   - no-browser");

                command.arg(arg::NO_BROWSER);
            }

            if options.no_sandbox {
                println!("   - no-sandbox");

                command
                    .arg(arg::CEF_DISABLE_SANDBOX)
                    .arg(arg::NO_CEF_SANDBOX)
                    .arg(arg::NO_SANDBOX);
            }

            if options.no_vr {
                println!("   - no-vr");

                command.arg(arg::VR_SKIP);
            }

            // we set SYSTEM_LD_LIBRARY_PATH and SYSTEM_PATH because steamui.so does the following:
            //  __stream = popen("LD_LIBRARY_PATH=\"$SYSTEM_LD_LIBRARY_PATH\" PATH=\"$SYSTEM_PATH\" lspci -mm -n", "r");

            command
                .arg0(esteem)
                .current_dir(&lib)
                .env(key::ESTEEM_LOAD_UI, "1")
                .env(key::LD_LIBRARY_PATH, &ld_library_path)
                .env(key::STEAM_ZENITY, options.zenity)
                .env(key::SYSTEM_LD_LIBRARY_PATH, &ld_library_path)
                .env(key::SYSTEM_PATH, &path)
                .exec();
        }
    }
}
