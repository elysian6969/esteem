#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum VanityUrlKind {
    Individual = 1,
    Group = 2,
    GameGroup = 3,
}
