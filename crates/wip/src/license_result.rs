#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum LicenseResult {
    Ok = 0,
    None = 1,
    Unathenticated = 2,
}
