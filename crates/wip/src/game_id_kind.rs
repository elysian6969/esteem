#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[repr(i32)]
pub enum GameIdKind {
    App = 0,
    GameMode = 1,
    Shortcut = 3,
    P2P = 3,
}
