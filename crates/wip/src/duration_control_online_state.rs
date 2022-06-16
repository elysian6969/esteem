#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[repr(i32)]
pub enum DurationControlOnlineState {
    Invalid = 0,
    Offline = 1,
    Online = 2,
    OnlineHighPri = 3,
}
