#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum AccountKind {
    Invalid = 0,
    Individual = 1,     // single user account
    Multiseat = 2,      // multiseat (e.g. cybercafe) account
    GameServer = 3,     // game server account
    AnonGameServer = 4, // anonymous game server account
    Pending = 5,        // pending
    ContentServer = 6,  // content server
    Clan = 7,
    Chat = 8,
    ConsoleUser = 9, // Fake SteamID for local PSN account on PS3 or Live account on 360, etc.
    AnonUser = 10,
}
