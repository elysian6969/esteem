use super::{api_fn, debug, virtual_struct};

virtual_struct! { Controller {
    fn init(&self) -> bool,
} }

impl Controller {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                init: SteamAPI_ISteamController_Init,
            },
        }
    }
}

api_fn! { Init(&Controller) -> bool {
    debug!();

    false
} }
