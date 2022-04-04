use super::virtual_struct;

virtual_struct! { UserStats {

} }

impl UserStats {
    pub const fn new() -> Self {
        Self { vtable: &VTable {} }
    }
}
