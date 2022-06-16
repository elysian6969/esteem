use crate::webhelper::{BlinkFeature, Realm, Universe, WebHelper};
use crate::{key, var_split, PREFIX};
use std::ffi::OsString;
use std::mem::MaybeUninit;
use std::os::unix::ffi::OsStringExt;
use std::path::Path;
use std::{env, mem, ptr, slice, str};

#[inline]
pub unsafe fn slice_from_ptr<'a, T>(ptr: *const T) -> &'a [T]
where
    T: PartialEq,
    T: 'a,
{
    let mut end = ptr;

    while end.read_unaligned() != MaybeUninit::<T>::zeroed().assume_init() {
        end = end.add(1);
    }

    let len = end.offset_from(ptr) as usize;
    let slice = slice::from_raw_parts(ptr, len);

    slice
}

#[inline]
pub unsafe fn str_from_ptr<'a>(ptr: *const u8) -> &'a str {
    let slice = slice_from_ptr(ptr);
    let string = str::from_utf8_unchecked(slice);

    string
}

#[no_mangle]
pub unsafe extern "C" fn fopen(path: *const u8, mode: *const u8) -> *const u8 {
    type Fn = unsafe extern "C" fn(path: *const u8, mode: *const u8) -> *const u8;

    let real_fopen: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "fopen\0".as_ptr().cast()));

    let path2 = str_from_ptr(path);
    let mode2 = str_from_ptr(mode);

    if !(path2.starts_with("/proc")
        || path2.starts_with("/sys")
        || path2.contains("crash_reporter.cfg"))
    {
        println!("esteem | intercepted \x1b[38;5;3mfopen\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m, mode: \x1b[38;5;2m{mode2:?}\x1b[m)");

        return real_fopen(path, mode);
    }

    println!("esteem | \x1b[38;5;1mrejected\x1b[m \x1b[38;5;3mfopen\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m, mode: \x1b[38;5;2m{mode2:?}\x1b[m)");

    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn system(command: *const u8) -> i32 {
    type Fn = unsafe extern "C" fn(command: *const u8) -> i32;

    let real_system: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "system\0".as_ptr().cast()));
    let command2 = str_from_ptr(command);

    if command2.contains("steamwebhelper") {
        println!(
            "esteem | intercepted \x1b[38;5;3msystem\x1b[m(command: \x1b[38;5;2m{command2:?}\x1b[m)"
        );

        let i686 = PREFIX.join("i686");
        let esteem = i686.join("libesteem.so");
        let sixfour = PREFIX.join("x86_64");

        let mut path = var_split(key::PATH);
        let mut ld_library_path = var_split(key::LD_LIBRARY_PATH);

        path.insert(0, i686.clone());
        ld_library_path.insert(0, sixfour);
        ld_library_path.insert(0, i686);

        let path = env::join_paths(path).expect("PATH");
        let ld_library_path = env::join_paths(ld_library_path).expect("LD_LIBRARY_PATH");

        let pos = command2.find("-steampid").unwrap();
        let res = &command2[pos.saturating_add(10)..];
        let pos2 = res.find(" ").unwrap();
        let pid = &res[..pos2.saturating_sub(1)];
        println!("pid = {pid:?}");
        let pid: u32 = pid.parse().unwrap();

        let mut webhelper = WebHelper::new("/usr/lib/esteem/x86_64/steamwebhelper");

        webhelper
            .build_id(0)
            .cache_dir("/ely/data/esteem/cef/cache")
            .current_dir("/usr/lib/esteem")
            .disable_features([BlinkFeature::Badging])
            .enable_features([
                BlinkFeature::AudioWorklet,
                BlinkFeature::ResizeObserver,
                BlinkFeature::Worklet,
            ])
            .gpu_watchdog(false)
            .hang_monitor(false)
            .index("/usr/lib/esteem/steamui/index.html")
            .lang("en_US")
            .library_path(&ld_library_path)
            .log_dir("/ely/data/esteem/cef/log")
            .log_file("/ely/data/esteem/cef/log.txt")
            .media_stream(true)
            .realm(Realm::Global)
            .quick_menu(false)
            .sandbox(false)
            .seccomp_filter(false)
            .smooth_scrolling(false)
            .steam_pid(pid)
            .universe(Universe::Dev);

        println!("esteem | executing {webhelper:?}");

        std::env::set_var("LD_LIBRARY_PATH", ld_library_path);

        let mut command = OsString::from("/usr/lib/esteem/x86_64/steamwebhelper ");

        for arg in webhelper.command().get_args() {
            command.push("'");
            command.push(arg);
            command.push("' ");
        }

        command.push("\0");
        let mut bytes = command.into_vec();
        let res = real_system(bytes.as_ptr());

        return res;
    }

    if command2.contains("disk-free") {
        const FREE: &str = r#"statvfs64 /: ret 0 errno 0
  f_bsize : 4096
  f_frsize: 4096
  f_blocks: 117149184
  f_bfree : 79630706
  f_bavail: 79536474
323584 MB avail
479232 MB total space
323584 MB free space
323584
479232
323584"#;

        let pos = command2.find(">").unwrap();
        let path = &command2[pos.saturating_add(1)..];
        println!("path = {path:?}");
        std::fs::write(path, FREE).unwrap();
        return 0;
    }

    // fallthrough
    // `/usr/lib/esteem/i686/../ubuntu12_32/vulkandriverquery`
    // `/usr/lib/esteem/i686/../ubuntu12_32/gldriverquery`
    println!("esteem | \x1b[38;5;1mrejected\x1b[m \x1b[38;5;3msystem\x1b[m(command: \x1b[38;5;2m{command2:?}\x1b[m)");

    -1
}

#[no_mangle]
pub unsafe extern "C" fn stat(path: *const u8, buf: *mut u8) -> i32 {
    type Fn = unsafe extern "C" fn(path: *const u8, buf: *mut u8) -> i32;

    let real_stat: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "stat\0".as_ptr().cast()));

    let path2 = str_from_ptr(path);

    println!("esteem | intercepted \x1b[38;5;3mstat\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m)");

    real_stat(path, buf)
}

#[no_mangle]
pub unsafe extern "C" fn fstat(fd: i32, buf: *mut u8) -> i32 {
    type Fn = unsafe extern "C" fn(fd: i32, buf: *mut u8) -> i32;

    let real_stat: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "fstat\0".as_ptr().cast()));

    println!("esteem | intercepted \x1b[38;5;3mfstat\x1b[m(fd: \x1b[38;5;2m{fd:?}\x1b[m)");

    real_stat(fd, buf)
}

#[no_mangle]
pub unsafe extern "C" fn lstat(path: *const u8, buf: *mut u8) -> i32 {
    type Fn = unsafe extern "C" fn(path: *const u8, buf: *mut u8) -> i32;

    let real_stat: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "lstat\0".as_ptr().cast()));

    let path2 = str_from_ptr(path);

    println!("esteem | intercepted \x1b[38;5;3mlstat\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m)");

    real_stat(path, buf)
}

#[no_mangle]
pub unsafe extern "C" fn statfs(path: *const u8, buf: *mut u8) -> i32 {
    type Fn = unsafe extern "C" fn(path: *const u8, buf: *mut u8) -> i32;

    let real_stat: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "statfs\0".as_ptr().cast()));

    let path2 = str_from_ptr(path);

    println!("esteem | intercepted \x1b[38;5;3mstatfs\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m)");

    real_stat(path, buf)
}

#[no_mangle]
pub unsafe extern "C" fn statfs64(path: *const u8, buf: *mut u8) -> i32 {
    type Fn = unsafe extern "C" fn(path: *const u8, buf: *mut u8) -> i32;

    let real_stat: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "statfs64\0".as_ptr().cast()));

    let path2 = str_from_ptr(path);

    println!("esteem | intercepted \x1b[38;5;3mstatfs64\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m)");

    real_stat(path, buf)
}

#[no_mangle]
pub unsafe extern "C" fn open(path: *const u8, mode: i32) -> i32 {
    type Fn = unsafe extern "C" fn(path: *const u8, mode: i32) -> i32;

    let real_stat: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "open\0".as_ptr().cast()));

    let path2 = str_from_ptr(path);

    if !(path2.starts_with("/proc") || path2.starts_with("/sys")) {
        println!("esteem | intercepted \x1b[38;5;3mopen\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m, mode: \x1b[38;5;3m{mode:?}\x1b[m)");

        return real_stat(path, mode);
    }

    println!("esteem | \x1b[38;5;1mrejected\x1b[m \x1b[38;5;3mopen\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m, mode: \x1b[38;5;3m{mode:?}\x1b[m)");

    -1
}
