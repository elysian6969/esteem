#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Instance;

impl Instance {
    pub const MMSLobby: i32 = 0x20000;
    pub const Lobby: i32 = 0x40000;
    pub const Clan: i32 = 0x80000;
}
