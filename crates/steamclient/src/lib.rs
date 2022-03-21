macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            ()
        }
    };
}

// todo
