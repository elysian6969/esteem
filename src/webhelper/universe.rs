#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Universe {
    Dev,
    Public,
}

impl Universe {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Universe::Dev => "Dev",
            Universe::Public => "Public",
        }
    }
}
