#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum AuthSessionResponse {
    Ok = 0,
    UserNotConnectedToSteam = 1,
    NoLicenseOrExpired = 2,
    VacBanned = 3,
    LoggedInElsewhere = 4,
    VacCheckTimedOut = 5,
    TicketCancelled = 6,
    AlreadyUsed = 7,
    Invalid = 8,
    PublisherIssuedBan = 9,
}
