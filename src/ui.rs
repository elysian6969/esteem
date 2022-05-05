use libloading::os::unix::{Library, Symbol};
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::{mem, ptr};

type MainFn = unsafe extern "C" fn(argc: i32, argv: *const *const i8);

pub struct SteamUI {
    library: Library,
}

impl SteamUI {
    pub fn open() -> Option<Self> {
        let library = unsafe { Library::new("steamui.so").ok()? };

        Some(Self { library })
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

        let mut args: Vec<*const i8> = args.into_iter().map(|arg| arg.as_ptr()).collect();

        args.push(ptr::null());

        unsafe {
            let main: Symbol<MainFn> = self.library.get(b"SteamDllMain\0").unwrap();
            let main: MainFn = mem::transmute(main.into_raw());

            main(args.len().saturating_sub(1) as i32, args.as_ptr());
        }
    }
}
