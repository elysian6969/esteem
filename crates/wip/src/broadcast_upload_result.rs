#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[repr(i32)]
pub enum BroadcastUploadResult {
    None = 0,
    Ok = 1,
    InitFailed = 2,
    FrameFailed = 3,
    Timeout = 4,
    BandwidthExceeded = 5,
    LowFPS = 6,
    MissingKeyFrames = 7,
    NoConnection = 8,
    RelayFailed = 9,
    SettingsChanged = 10,
    MissingAudio = 11,
    TooFarBehind = 12,
    TranscodeBehind = 13,
    NotAllowedToPlay = 14,
    Busy = 15,
    Banned = 16,
    AlreadyActive = 17,
    ForcedOff = 18,
    AudioBehind = 19,
    Shutdown = 20,
    Disconnect = 21,
    VideoInitFailed = 22,
    AudioInitFailed = 23,
}
