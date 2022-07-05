#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlinkFeature {
    AudioWorklet,
    Badging,
    ResizeObserver,
    Worklet,
}

impl BlinkFeature {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            BlinkFeature::AudioWorklet => "AudioWorklet",
            BlinkFeature::Badging => "Badging",
            BlinkFeature::ResizeObserver => "ResizeObserver",
            BlinkFeature::Worklet => "Worklet",
        }
    }
}
