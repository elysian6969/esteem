use core::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AppId(pub u32);

impl fmt::Debug for AppId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl fmt::Display for AppId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}
