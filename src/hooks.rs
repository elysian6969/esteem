use crate::{key, var_split, PREFIX};
use crate::webhelper::{WebHelper, Universe, BlinkFeature, Realm};
use std::path::Path;
use std::mem::MaybeUninit;
use std::{mem, ptr, slice, str, env};

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

    if !path2.starts_with("/proc") {
        //println!("esteem | intercepted \x1b[38;5;3mfopen\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m, mode: \x1b[38;5;2m{mode2:?}\x1b[m)");

        return real_fopen(path, mode);
    }

    //println!("esteem | \x1b[38;5;1mrejected\x1b[m \x1b[38;5;3mfopen\x1b[m(path: \x1b[38;5;2m{path2:?}\x1b[m, mode: \x1b[38;5;2m{mode2:?}\x1b[m)");

    ptr::null()
}

use std::cell::UnsafeCell;
struct A(UnsafeCell<bool>);

unsafe impl Send for A {}
unsafe impl Sync for A {}

static WEBHELPER_LOADED: A = A(UnsafeCell::new(false));

use std::fs;

#[no_mangle]
pub unsafe extern "C" fn system(command: *const u8) -> i32 {
    type Fn = unsafe extern "C" fn(command: *const u8) -> i32;

    let real_system: Fn = mem::transmute(libc::dlsym(libc::RTLD_NEXT, "system\0".as_ptr().cast()));
    let command2 = str_from_ptr(command);

    if command2.contains("steamwebhelper") && WEBHELPER_LOADED.0.get().read() == false {
        println!(
            "esteem | intercepted \x1b[38;5;3msystem\x1b[m(command: \x1b[38;5;2m{command2:?}\x1b[m)"
        );

        let i686 = PREFIX.join("i686");
        let esteem = i686.join("libesteem.so");
        let sixfour = PREFIX.join("ubuntu12_64");

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

        let mut webhelper = WebHelper::new("/usr/lib/esteem/ubuntu12_64/steamwebhelper");

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
            .library_path(ld_library_path)
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

        let pid = fs::read_to_string("/tmp/webhelper.lock")
            .ok()
            .and_then(|pid| i32::from_str_radix(&pid, 10).ok())
            .unwrap_or(-1);

        println!("pid = {pid:?}");

        if Path::new(&format!("/proc/{pid}")).exists() {
            println!("esteem | \x1b[38;5;1mrejected\x1b[m \x1b[38;5;3msystem\x1b[m(command: \x1b[38;5;2m{command2:?}\x1b[m)");

            return -1;
        } else {
            let mut child = webhelper.spawn().unwrap();
            fs::write("/tmp/webhelper.lock", child.id().to_string()).unwrap();
            WEBHELPER_LOADED.0.get().write(true);
        }

        return 0;
    }

    // fallthrough
    // `/usr/lib/esteem/i686/../ubuntu12_32/vulkandriverquery`
    // `/usr/lib/esteem/i686/../ubuntu12_32/gldriverquery`
    println!("esteem | \x1b[38;5;1mrejected\x1b[m \x1b[38;5;3msystem\x1b[m(command: \x1b[38;5;2m{command2:?}\x1b[m)");

    -1
}
