use link::Library;
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::{mem, ptr};

type MainFn = unsafe extern "C" fn(argc: i32, argv: *const *const i8);

pub struct Flage {
    cef_disable_sandbox: bool,
    cef_disable_breakpad: bool,
    cef_disable_hang_timeouts: bool,
    disable_partner_licenses: bool,
    fs_log: bool,
    fs_log_bins: bool,
    log_net_api: bool,
    no_big_picture: bool,
    no_browser: bool,
    no_cef_sandbox: bool,
    no_crash_monitor: bool,
    no_intro: bool,
    no_sandbox: bool,
    no_shared_textures: bool,
    open_devtools: bool,
    open_overlay_devtools: bool,
    skip_streaming_drivers: bool,
    vr_skip: bool,
    windowed: bool,
}

pub struct SteamUI {
    library: Library,
}

impl SteamUI {
    pub fn open() -> Result<Self, link::OpenError> {
        let library = unsafe { Library::load("steamui.so")? };

        Ok(Self { library })
    }

    pub fn main<I, S>(&self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args: Vec<_> = args
            .into_iter()
            .flat_map(|arg| {
                let arg = arg.as_ref();
                let arg = arg.as_bytes();
                let arg = CString::new(arg).ok()?;

                Some(arg)
            })
            .collect();

        let mut args: Vec<*const i8> = args.iter().map(|arg| arg.as_ptr()).collect();
        let len = args.len() as i32;

        args.push(ptr::null());

        unsafe {
            // ghidra gives us
            //
            // void SteamDllMain(int argc, char **argv) {
            //     SteamDllMainEx(argc,argv, (char **)NULL);
            //     return;
            // }
            //
            // void SteamDllMainEx(int argc, char **argv, char **possibly_envp) {
            //     possibly_environ = possibly_envp;
            //     main(argc, argv);
            //     return;
            // }
            //
            let main = self.library.symbol_ptr::<_, u8>("SteamDllMain\0").unwrap();
            let main: MainFn = mem::transmute(main);

            // FIXME
            main(len, args.as_ptr());
        }
    }
}
