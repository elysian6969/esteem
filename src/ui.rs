use libloading::os::unix::{Library, Symbol};
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::{mem, ptr};

type MainFn = unsafe extern "C" fn(argc: i32, argv: *const *const i8);

pub struct SteamUI {
    library: Library,
}

impl SteamUI {
    pub fn open() -> Result<Self, libloading::Error> {
        let library = unsafe { Library::new("steamui.so")? };

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
            let main: Symbol<MainFn> = self.library.get(b"SteamDllMain\0").unwrap();
            let main: MainFn = mem::transmute(main.into_raw());

            // FIXME
            main(len, args.as_ptr());
        }
    }
}
