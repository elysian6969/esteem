use super::{debug, virtual_struct};

virtual_struct! { HTMLSurface {
    //fn deconstructor(&self) -> (),
    fn init(&self) -> (),
    fn shutdown(&self) -> (),
    fn create_browser(&self, user_agent: *const u8, css: *const u8) -> i32,
} }

impl HTMLSurface {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                //deconstructor,
                init,
                shutdown,
                create_browser,
            },
        }
    }
}

//extern "C" fn deconstructor(this: *const HTMLSurface) {
//    debug!();
//}

extern "C" fn init(this: *const HTMLSurface) {
    debug!();
}

extern "C" fn shutdown(this: *const HTMLSurface) {
    debug!();
}

extern "C" fn create_browser(
    this: *const HTMLSurface,
    user_agent: *const u8,
    css: *const u8,
) -> i32 {
    debug!();

    69
}
