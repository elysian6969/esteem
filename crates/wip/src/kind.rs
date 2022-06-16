/// [EAccountType](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Kind {
    /// Used for invalid Steam IDs
    Invalid = 0,
    /// single user account
    Individual = 1,
    /// multiseat (e.g. cybercafe) account
    MultiSeat = 2,
    /// game server account
    GameServer = 3,
    /// anonymous game server account
    AnonGameServer = 4,
    /// pending
    Pending = 5,
    /// content server
    ContentServer = 6,
    /// Steam Group (clan)
    Clan = 7,
    /// Steam group chat or lobby
    Chat = 8,
    /// Fake SteamID for local PSN account on PS3 or Live account on 360, etc
    ConsoleUser = 9,
    /// Anonymous user account. (Used to create an account or reset a password)
    AnonUser = 10,
    Max = 11,
}
