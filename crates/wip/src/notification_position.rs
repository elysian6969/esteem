#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum NotificationPosition {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 4,
}
