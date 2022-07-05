#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PasswordStore {
    Basic,
}

impl PasswordStore {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            PasswordStore::Basic => "basic",
        }
    }
}
