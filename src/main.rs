use clap::Parser;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Parser)]
struct Options {
    /// Enable apulse
    #[clap(long)]
    apulse: bool,

    /// Only CLI.
    #[clap(long)]
    cli: bool,

    /// Enable devtools in CEF.
    #[clap(long)]
    devtools: bool,

    /// Run GDB on steam itself.
    #[clap(long)]
    gdb: bool,

    /// Path to the pipewire stub.
    #[clap(long)]
    pipewire_stub: Option<PathBuf>,

    /// Enable some more steam logs.
    #[clap(long)]
    more_log: bool,

    /// Disable CEF entirely.
    #[clap(long)]
    no_browser: bool,

    /// Disable CEF sandbox.
    #[clap(long)]
    no_sandbox: bool,

    /// Disable SteamVR initialization.
    #[clap(long)]
    no_vr: bool,

    /// Set the `STEAM_ZENITY` environment variable.
    #[clap(default_value = "/usr/bin/zenity", long)]
    zenity: PathBuf,
}

pub const STEAM_SCRIPTS: &Path =
    unsafe { &*(env!("CARGO_MANIFEST_DIR") as *const str as *const Path) };

pub const GDB: &str = "gdb";

pub mod key {
    pub const HOME: &str = "HOME";
    pub const LD_LIBRARY_PATH: &str = "LD_LIBRARY_PATH";
    pub const PATH: &str = "PATH";
    pub const SDL_VIDEO_X11_DGAMOUSE: &str = "SDL_VIDEO_X11_DGAMOUSE";
    pub const STEAMSCRIPT: &str = "STEAMSCRIPT";
    pub const STEAMSCRIPT_VERSION: &str = "STEAMSCRIPT_VERSION";
    pub const STEAM_RUNTIME: &str = "STEAM_RUNTIME";
    pub const STEAM_RUNTIME_LIBRARY_PATH: &str = "STEAM_RUNTIME_LIBRARY_PATH";
    pub const STEAM_ZENITY: &str = "STEAM_ZENITY";
    pub const SYSTEM_LD_LIBRARY_PATH: &str = "SYSTEM_LD_LIBRARY_PATH";
    pub const SYSTEM_PATH: &str = "PATH";
    pub const TEXTDOMAIN: &str = "TEXTDOMAIN";
    pub const TEXTDOMAINDIR: &str = "TEXTDOMAINDIR";
    pub const XDG_DATA_HOME: &str = "XDG_DATA_HOME";
}

pub mod arg {
    pub const CEF_DISABLE_SANDBOX: &str = "-cef-disable-sandbox";
    pub const CEF_DISABLE_BREAKPAD: &str = "-cef-disable-breakpad";
    pub const CEF_DISABLE_HANG_TIMEOUTS: &str = "-cef-disable-hang-timeouts";
    pub const DISABLE_PARTNER_LICENSES: &str = "-disablepartnerlicenses";
    pub const FS_LOG: &str = "-fs_log";
    pub const FS_LOG_BINS: &str = "-fs_logbins";
    pub const LOG_NET_API: &str = "-lognetapi";
    pub const NO_BIG_PICTURE: &str = "-nobigpicture";
    pub const NO_BROWSER: &str = "-no-browser";
    pub const NO_CEF_SANDBOX: &str = "-no-cef-sandbox";
    pub const NO_CRASH_MONITOR: &str = "-nocrashmonitor";
    pub const NO_INTRO: &str = "-nointro";
    pub const NO_SANDBOX: &str = "-no-sandbox";
    pub const NO_SHARED_TEXTURES: &str = "-no-shared-textures";
    pub const OPEN_DEV_TOOLS: &str = "-opendevtools";
    pub const OPEN_OVERLAY_DEV_TOOLS: &str = "-openoverlaydevtools";
    pub const SKIP_STREAMING_DRIVERS: &str = "-skipstreamingdrivers";
    // requires steamconsole.so
    pub const TEXT_CLIENT: &str = "-textclient";
    pub const VR_SKIP: &str = "-vrskip";
    pub const WINDOWED: &str = "-windowed";
}

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

use std::ffi::{OsStr, OsString};

fn var_os<K: AsRef<OsStr>>(key: K) -> OsString {
    env::var_os(key).unwrap_or_default()
}

fn var_split<K: AsRef<OsStr>>(key: K) -> Vec<PathBuf> {
    env::split_paths(&var_os(key)).collect()
}

fn main() {
    let root_dir = Path::new(ROOT);
    let steam_bin = root_dir.join("lib/i686/bootstrap");

    let options = Options::parse();

    let mut command = match (options.apulse, options.gdb) {
        (true, true) => {
            let mut command = Command::new("apulse");

            command.arg("gdb");
            command.arg("--args");
            command.arg(steam_bin);
            command
        }
        (false, true) => {
            let mut command = Command::new("gdb");

            command.arg("--args");
            command.arg(steam_bin);
            command
        }
        (true, false) => {
            let mut command = Command::new("apulse");

            command.arg(steam_bin);
            command
        }
        (false, false) => {
            let mut command = Command::new(steam_bin);

            command
        }
    };

    command.current_dir(root_dir.join("lib/i686"));

    let mut path = var_split(key::PATH);
    let mut ld_library_path = var_split(key::LD_LIBRARY_PATH);

    ld_library_path.insert(0, root_dir.join("lib/i686"));

    if let Some(stub) = &options.pipewire_stub {
        //let x86_64 = stub.join("x86_64");
        //let i686 = stub.join("i686");

        //ld_library_path.push(x86_64);
        //ld_library_path.push(i686);
    }

    for path in &ld_library_path {
        println!("   - {:?}", path);
    }

    let path = env::join_paths(path).expect("PATH");
    let ld_library_path = env::join_paths(ld_library_path).expect("LD_LIBRARY_PATH");

    command
        .env(key::LD_LIBRARY_PATH, &ld_library_path)
        .env(key::PATH, &path)
        //.env(key::SDL_VIDEO_X11_DGAMOUSE, "0")
        //.env(key::STEAM_RUNTIME, steam_runtime)
        //.env(key::STEAM_RUNTIME_LIBRARY_PATH, &path)
        .env(key::STEAM_ZENITY, options.zenity);
        //.env(key::SYSTEM_LD_LIBRARY_PATH, ld_library_path)
        //.env(key::SYSTEM_PATH, path)
        //.env(key::TEXTDOMAIN, "steam")
        //.env(key::TEXTDOMAINDIR, "/usr/share/locale");

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

    if options.apulse {
        println!("   - apulse");
    }

    if options.cli {
        println!("   - cli");

        command.arg(arg::TEXT_CLIENT);
    }

    if options.devtools {
        println!("   - devtools");

        command.arg(arg::OPEN_DEV_TOOLS);
        command.arg(arg::OPEN_OVERLAY_DEV_TOOLS);
    }

    if options.more_log {
        println!("   - more-log");

        command.arg(arg::FS_LOG);
        command.arg(arg::FS_LOG_BINS);
        command.arg(arg::LOG_NET_API);
    }

    if options.no_browser {
        println!("   - no-browser");

        command.arg(arg::NO_BROWSER);
    }

    if options.no_sandbox {
        println!("   - no-sandbox");

        command.arg(arg::CEF_DISABLE_SANDBOX);
        command.arg(arg::NO_CEF_SANDBOX);
        command.arg(arg::NO_SANDBOX);
    }

    if options.no_vr {
        println!("   - no-vr");

        command.arg(arg::VR_SKIP);
    }

    if options.pipewire_stub.is_some() {
        println!("   - pipewire-stub");
    }

    println!("{:?}", command);
    let mut child = command.spawn().expect("spawn steam");
    let status = child.wait().expect("wait steam");

    println!("{status:?}");
}
