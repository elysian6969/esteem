use super::virtual_struct;

virtual_struct! { MatchmakingServers {

} }

impl MatchmakingServers {
    pub const fn new() -> Self {
        Self { vtable: &VTable {} }
    }
}
