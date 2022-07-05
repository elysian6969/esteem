#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gl {
    Angle,
    Desktop,
    Egl,
}

impl Gl {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Gl::Angle => "angle",
            Gl::Desktop => "desktop",
            Gl::Egl => "egl",
        }
    }
}
