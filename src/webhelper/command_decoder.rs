#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandDecoder {
    Passthrough,
}

impl CommandDecoder {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            CommandDecoder::Passthrough => "passthrough",
        }
    }
}
