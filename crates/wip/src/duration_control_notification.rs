#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[repr(i32)]
pub enum DurationControlNotification {
    None = 0,
    OneHour = 1,
    ThreeHours = 2,
    HalfProgress = 3,
    NoProgress = 4,
    ExitSoon_3h = 5,
    ExitSoon_5h = 6,
    ExitSoon_Night = 7,
}
