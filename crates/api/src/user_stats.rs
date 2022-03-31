use super::{api_fn, debug, virtual_struct};
use core::ptr;

virtual_struct! { UserStats {

} }

impl UserStats {
    pub const fn new() -> Self {
        Self { vtable: &VTable {} }
    }
}
