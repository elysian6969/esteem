#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum PlayerResult {
    FailedToConnect = 1,
    Abandoned = 2,
    Kicked = 3,
    Incomplete = 4,
    Completed = 5,
}
