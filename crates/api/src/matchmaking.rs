use super::virtual_struct;

virtual_struct! { Matchmaking {

} }

impl Matchmaking {
    pub const fn new() -> Self {
        Self { vtable: &VTable {} }
    }
}
