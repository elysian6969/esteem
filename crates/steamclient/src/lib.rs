macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            ()
        }
    };
}

static INTERFACE: usize = 0;

#[no_mangle]
pub extern "C" fn CreateInterface() -> *const usize {
    &INTERFACE as *const usize
}
