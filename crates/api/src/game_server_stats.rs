use super::virtual_struct;

virtual_struct! { GameServerStats {

} }

impl GameServerStats {
    pub const fn new() -> Self {
        Self { vtable: &VTable {} }
    }
}
