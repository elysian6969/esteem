use super::virtual_struct;

virtual_struct! { GameServer {

} }

impl GameServer {
    pub const fn new() -> Self {
        Self { vtable: &VTable {} }
    }
}
