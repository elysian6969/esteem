use findshlibs::{Segment, SharedLibrary, TargetSharedLibrary};
use std::arch::asm;
use std::thread;
use std::{process, ptr};

// .init_array entries are invoked by glibc
#[link_section = ".init_array"]
#[used(linker)]
static BOOTSTRAP: unsafe extern "C" fn() = bootstrap;

// function in above section
#[link_section = ".text.startup"]
pub unsafe extern "C" fn bootstrap() {
    println!("esteem | bootstrap");

    thread::spawn(main2);
}

fn main2() {
    return;
    thread::sleep(std::time::Duration::from_secs(5));

    TargetSharedLibrary::each(|shlib| {
        println!("{}", shlib.name().to_string_lossy());

        for seg in shlib.segments() {
            println!(
                "    {}: segment {}",
                seg.actual_virtual_memory_address(shlib),
                seg.name()
            );
        }
    });
}

// we'd like to be interpreted, force interpretation
#[link_section = ".interp"]
#[used(linker)]
static INTERPRETER: [u8; 19] = *b"/lib/ld-linux.so.2\0";

// link with `-e_start` to redirect the entry to here
// regular start procedures are broken
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        // mark outermost stack frame by zeroing ebp
        "xor ebp, ebp",
        // pop top of stack
        "pop esi",
        // move kernel block pointer to esp
        "mov ecx, esp",
        // align stack pointer
        "and esp, -16",
        // call setup
        "call {}",
        sym pre_main,
        options(noreturn),
    );
}

// normally this is called by crt init routines, however we overrode that, replicate the behaviour
#[inline]
unsafe fn init_rust_args(argc: i32, argv: *const *const u8) {
    // https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/args.rs#L110
    extern "C" {
        #[link_name = "_ZN3std3sys4unix4args3imp15ARGV_INIT_ARRAY17hda426f9ff8bc0e9eE"]
        static ARGV_INIT_ARRAY:
            unsafe extern "C" fn(argc: i32, argv: *const *const u8, envp: *const *const u8);
    }

    (ARGV_INIT_ARRAY)(argc, argv, ptr::null());
}

// initialize things, call main, and exit
#[inline(never)]
unsafe extern "C" fn pre_main(info: *const u8) {
    #[repr(C)]
    struct Info {
        argc: isize,
        argv: *const u8,
    }

    let info = info.cast::<Info>();
    let argc = (*info).argc;
    let argv = &(*info).argv;

    // fix `std::env::args()`
    init_rust_args(argc as i32, argv);

    crate::main();

    process::exit(0);
}
