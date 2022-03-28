use clap::Parser;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Parser)]
struct Options {
    /// Enable apulse
    #[clap(long)]
    apulse: bool,

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

pub const LD_LIBRARY_PATH: &str = "LD_LIBRARY_PATH";
pub const PATH: &str = "PATH";
pub const SDL_VIDEO_X11_DGAMOUSE: &str = "SDL_VIDEO_X11_DGAMOUSE";
pub const STEAMSCRIPT: &str = "STEAMSCRIPT";
pub const STEAMSCRIPT_VERSION: &str = "STEAMSCRIPT_VERSION";
pub const STEAM_RUNTIME: &str = "STEAM_RUNTIME";
pub const STEAM_RUNTIME_LIBRARY_PATH: &str = "STEAM_RUNTIME_LIBRARY_PATH";
pub const STEAM_ZENITY: &str = "STEAM_ZENITY";

fn main() {
    let prefix = if let Some(xdg) = env::var_os("XDG_DATA_HOME") {
        xdg.into()
    } else {
        let home = env::var_os("HOME").expect("HOME");
        let home = Path::new(&home);

        home.join(".local/share")
    };

    let steam = prefix.join("Steam");
    let steam_bin = steam.join("ubuntu12_32/steam");
    let steam_runtime = steam.join("ubuntu12_32/steam-runtime");

    println!(" * Steam root: {}", steam.display());
    println!(" * Steam binary: {}", steam_bin.display());
    println!(" * Steam runtime: {}", steam_runtime.display());

    let options = Options::parse();

    let mut command = if options.gdb {
        let mut command = Command::new(GDB);

        command.arg("--args");
        command.arg(steam_bin);
        command
    } else {
        Command::new(steam_bin)
    };

    let path = env::var_os(PATH).unwrap_or_else(|| "".into());
    let ld_library_path = env::var_os(LD_LIBRARY_PATH).unwrap_or_else(|| "".into());

    let path = env::split_paths(&path);
    let ld_library_path = env::split_paths(&ld_library_path);

    let mut path_append: Vec<PathBuf> = Vec::new();
    let mut ld_library_path_append: Vec<PathBuf> = Vec::new();

    let path1 = steam_runtime.join("amd64/bin");
    let path2 = steam_runtime.join("amd64/usr/bin");
    let path3 = steam_runtime.join("usr/bin");

    path_append.push(path1);
    path_append.push(path2);
    path_append.push(path3);

    ld_library_path_append.push(STEAM_SCRIPTS.join("lib/i686"));
    ld_library_path_append.push(STEAM_SCRIPTS.join("lib/x86_64"));

    if options.apulse {
        ld_library_path_append.push("/usr/lib64/apulse".into());
        ld_library_path_append.push("/usr/lib/apulse".into());
    }

    if let Some(stub) = options.pipewire_stub {
        let x86_64 = stub.join("x86_64");
        let i686 = stub.join("i686");

        ld_library_path_append.push(x86_64);
        ld_library_path_append.push(i686);
    }

    ld_library_path_append.push("/lib64".into());
    ld_library_path_append.push("/lib".into());
    ld_library_path_append.push("/usr/lib64".into());
    ld_library_path_append.push("/usr/lib".into());
    ld_library_path_append.push("/usr/lib/gcc/x86_64-pc-linux-gnu/11.2.0/".into());
    ld_library_path_append.push("/usr/lib/gcc/x86_64-pc-linux-gnu/11.2.0/32".into());

    let iter = path_append.into_iter().chain(path);
    let path = match env::join_paths(iter) {
        Ok(path) => path,
        Err(_) => {
            println!(" !  Invalid {PATH}.");

            return;
        }
    };

    let iter = ld_library_path_append.into_iter().chain(ld_library_path);
    let ld_library_path = match env::join_paths(iter) {
        Ok(ld_library_path) => ld_library_path,
        Err(_) => {
            println!(" !  Invalid {LD_LIBRARY_PATH}.");

            return;
        }
    };

    command
        .env(LD_LIBRARY_PATH, &ld_library_path)
        .env(PATH, &path)
        .env(SDL_VIDEO_X11_DGAMOUSE, "0")
        .env(STEAM_RUNTIME, steam_runtime)
        .env(STEAM_RUNTIME_LIBRARY_PATH, &path)
        .env(STEAM_ZENITY, options.zenity)
        .env("SYSTEM_LD_LIBRARY_PATH", ld_library_path)
        .env("SYSTEM_PATH", path)
        .env("TEXTDOMAIN", "steam")
        .env("TEXTDOMAINDIR", "/usr/share/locale");

    command
        .arg("-cef-disable-breakpad")
        .arg("-cef-disable-hang-timeouts")
        .arg("-disablepartnerlicenses")
        .arg("-no-shared-textures")
        .arg("-nobigpicture")
        .arg("-nocrashmonitor")
        .arg("-nointro")
        .arg("-skipstreamingdrivers")
        .arg("-windowed");

    if options.devtools {
        command.arg("-opendevtools");
        command.arg("-openoverlaydevtools");
    }

    if options.more_log {
        command.arg("-fs_log");
        command.arg("-fs_logbins");
        command.arg("-lognetapi");
    }

    if options.no_browser {
        command.arg("-no-browser");
    }

    if options.no_sandbox {
        command.arg("-cef-disable-sandbox");
        command.arg("-no-cef-sandbox");
    }

    if options.no_vr {
        command.arg("-vrskip");
    }

    let mut child = command.spawn().expect("spawn steam");
    let status = child.wait().expect("wait steam");

    println!("{status:?}");
}
