use libloading::os::unix::{Library, Symbol};
use std::{env, ptr, mem};
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;

type MainFn = unsafe extern "C" fn(argc: *const *const i8, envp: *const *const i8);

fn main() {
    unsafe {
        env::set_current_dir("..").unwrap();

        let steamui_path = "steamui.so";
        let steamui = Library::new(steamui_path).unwrap();
        let mainfn: Symbol<MainFn> = steamui.get(b"SteamDllMain\0").unwrap();
        let mainfn: MainFn = mem::transmute(mainfn.into_raw());

        let args: Vec<_> = env::args_os().map(|arg| CString::new(arg.into_vec()).unwrap()).collect();
        let mut args: Vec<*const i8> = args.into_iter().map(|arg| arg.as_ptr()).collect();

        args.push(ptr::null());

        let vars: Vec<_> = env::vars_os().map(|(key, val)| {
            let mut var = key;

            var.push("=");
            var.push(val);

            CString::new(var.into_vec()).unwrap()
        }).collect();

        let mut vars: Vec<*const i8> = vars.into_iter().map(|var| var.as_ptr()).collect();

        vars.push(ptr::null());

        mainfn(args.as_ptr(), vars.as_ptr());
    }
}
