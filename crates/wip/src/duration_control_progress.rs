#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[repr(i32)]
pub enum DurationControlProgress {
    Full = 0,
    Half = 1,
    None = 2,
    Exit3h = 3,
    Exit5h = 4,
    ExitNight = 5,
}
