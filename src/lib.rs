#![feature(asm_sym)]
#![feature(used_with_arg)]

use std::collections::{BTreeMap, HashMap};
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

fn var_os<K: AsRef<OsStr>>(env: &HashMap<OsString, OsString>, key: K) -> OsString {
    env.get(key.as_ref())
        .map(|env| env.clone())
        .unwrap_or_default()
}

fn var_split<K: AsRef<OsStr>>(env: &HashMap<OsString, OsString>, key: K) -> Vec<PathBuf> {
    std::env::split_paths(&var_os(env, key)).collect()
}

//const PREFIX: &Path = new_path(env!("CARGO_MANIFEST_DIR"));
const PREFIX: &Path = new_path("/usr/lib/esteem");

fn print_option(option: &str) {
    println!("esteem | enabled \x1b[38;5;1m{option}\x1b[m");
}

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

// this is called by glibc after the library is loaded into a process
#[link_section = ".init_array"]
#[used(linker)]
static BOOTSTRAP: unsafe extern "C" fn() = bootstrap;

// this tricks glibc into thinking this is an executable binary
#[link_section = ".interp"]
#[used(linker)]
static INTERPRETER: [u8; 19] = *b"/lib/ld-linux.so.2\0";

#[link_section = ".text.startup"]
pub unsafe extern "C" fn bootstrap() {
    println!("esteem | bootstrap");

    let program = std::env::args_os().next();

    println!("{program:?}");
}

#[no_mangle]
pub unsafe extern "C" fn entry() {
    core::arch::asm!(
        // pop top of stack
        "pop esi",
        // move kernel block pointer to esp
        "mov ecx, esp",
        // align stack pointer
        "and esp, -16",
        // call setup
        "call {}",
        sym setup,
    );
}

#[derive(Debug)]
#[repr(C)]
struct Inner {
    argc: isize,
    argv: *const u8,
}

#[derive(Debug)]
#[repr(C)]
struct KernelInfo {
    inner: &'static Inner,
}

unsafe fn slice<'a, T>(address: *const T) -> &'a [T]
where
    T: PartialEq,
    T: 'a,
{
    let mut end = address;

    while *end != core::mem::MaybeUninit::<T>::zeroed().assume_init() {
        end = end.add(1);
    }

    let len = end.offset_from(address);

    core::slice::from_raw_parts(address, len as usize)
}

use std::os::unix::ffi::OsStringExt;

impl KernelInfo {
    unsafe fn raw_args(&self) -> &[*const u8] {
        let args = &self.inner.argv as *const *const u8;
        let args = slice(args);

        args
    }

    unsafe fn raw_envs(&self) -> &[*const u8] {
        let args = self.raw_args();
        let envs = args.as_ptr().add(args.len()).add(1);
        let envs = slice(envs);

        envs
    }

    pub fn args(&self) -> Vec<OsString> {
        unsafe {
            let args = self.raw_args();
            let args = args.iter().map(|arg| {
                let arg = slice(*arg);
                let arg = arg.into();

                OsStringExt::from_vec(arg)
            });

            args.collect()
        }
    }

    pub fn envs(&self) -> HashMap<OsString, OsString> {
        unsafe {
            let envs = self.raw_envs();
            let envs = envs.iter().map(|env| unsafe {
                let env = slice(*env);
                let mut parts = env.split(|byte| *byte == b'=');

                let key = parts.next().unwrap_unchecked().into();
                let val = parts.next().unwrap_unchecked().into();

                let key = OsStringExt::from_vec(key);
                let val = OsStringExt::from_vec(val);

                (key, val)
            });

            envs.collect()
        }
    }
}

use clap::Parser;

unsafe extern "C" fn setup(info: KernelInfo) {
    let args = info.args();
    let envs = info.envs();

    let home = envs
        .get(std::ffi::OsStr::new("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"));

    let data_dir = envs
        .get(std::ffi::OsStr::new("XDG_DATA_HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));

    let data_dir = data_dir.join("esteem");

    let options = options::Options::parse_from(args.clone());

    if envs.contains_key(std::ffi::OsStr::new(key::ESTEEM_LOAD_UI)) {
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

        println!("esteem | options");

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
        }

        println!("esteem | load \x1b[38;5;2msteamui.so\x1b[m");

        let ui = ui::SteamUI::open().expect("steamui.so failed to load");

        ui.main(args.0.iter().map(|s| s.as_ref()));
    } else {
        let i686 = PREFIX.join("i686");
        let esteem = i686.join("esteem");

        let mut path = var_split(&envs, key::PATH);
        let mut ld_library_path = var_split(&envs, key::LD_LIBRARY_PATH);

        path.insert(0, i686.clone());
        ld_library_path.insert(0, i686);

        let path = std::env::join_paths(path).expect("PATH");
        let ld_library_path = std::env::join_paths(ld_library_path).expect("LD_LIBRARY_PATH");

        if let Some(program) = args.get(0) {
            let mut command = Command::new(program);

            // we set SYSTEM_LD_LIBRARY_PATH and SYSTEM_PATH because steamui.so does the following:
            //  __stream = popen("LD_LIBRARY_PATH=\"$SYSTEM_LD_LIBRARY_PATH\" PATH=\"$SYSTEM_PATH\" lspci -mm -n", "r");

            let home_dir = home;
            let data_dir = data_dir;

            command
                .arg0(esteem)
                .args(args.get(2))
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
