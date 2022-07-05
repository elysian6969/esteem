#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Realm {
    Global,
}

impl Realm {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Realm::Global => "global",
        }
    }
}
