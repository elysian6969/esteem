#![allow(unused_variables)]

macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            frosting::println!();

            ()
        }
    };
}

stub!(CreateVideoPlayer);
stub!(DeleteVideoPlayer);
stub!(RescaleBase);
