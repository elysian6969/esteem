#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum GameSearchErrorCode {
    Ok = 1,
    SearchAlreadyInProgress = 2,
    NoSearchInProgress = 3,
    NotLobbyLeader = 4,
    NoHostAvailable = 5,
    SearchParamsInvalid = 6,
    Offline = 7,
    NotAuthorized = 8,
    UnknownError = 9,
}
