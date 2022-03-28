use super::{debug, UserHandle};
use core::ptr;

#[repr(C)]
pub struct User {
    vtable: &'static VTable,
}

impl User {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                get_handle: SteamAPI_ISteamUser_GetHSteamUser,
                logged_on: SteamAPI_ISteamUser_BLoggedOn,
                get_steam_id: SteamAPI_ISteamUser_GetSteamID,
                initiate_game_connection: SteamAPI_ISteamUser_InitiateGameConnection,
                terminate_game_connection: SteamAPI_ISteamUser_TerminateGameConnection,
                track_app_usage_event: SteamAPI_ISteamUser_TrackAppUsageEvent,
                get_user_data_folder: SteamAPI_ISteamUser_GetUserDataFolder,
                start_voice_recording: SteamAPI_ISteamUser_StartVoiceRecording,
                stop_voice_recording: SteamAPI_ISteamUser_StopVoiceRecording,
                get_available_voice: SteamAPI_ISteamUser_GetAvailableVoice,
                get_voice: SteamAPI_ISteamUser_GetVoice,
                decompress_voice: SteamAPI_ISteamUser_DecompressVoice,
                get_voice_optimal_sample_rate: SteamAPI_ISteamUser_GetVoiceOptimalSampleRate,
                get_auth_session_ticket: SteamAPI_ISteamUser_GetAuthSessionTicket,
                begin_auth_session: SteamAPI_ISteamUser_BeginAuthSession,
                end_auth_session: SteamAPI_ISteamUser_EndAuthSession,
                cancel_auth_ticket: SteamAPI_ISteamUser_CancelAuthTicket,
                user_has_license_for_app: SteamAPI_ISteamUser_UserHasLicenseForApp,
                is_behind_nat: SteamAPI_ISteamUser_BIsBehindNAT,
                advertise_game: SteamAPI_ISteamUser_AdvertiseGame,
                request_encrypted_app_ticket: SteamAPI_ISteamUser_RequestEncryptedAppTicket,
                get_encrypted_app_ticket: SteamAPI_ISteamUser_GetEncryptedAppTicket,
                get_game_badge_level: SteamAPI_ISteamUser_GetGameBadgeLevel,
                get_player_steam_level: SteamAPI_ISteamUser_GetPlayerSteamLevel,
                request_store_url: SteamAPI_ISteamUser_RequestStoreAuthURL,
            },
        }
    }
}

#[repr(C)]
struct VTable {
    get_handle: extern "C" fn(this: *const User) -> UserHandle,

    logged_on: extern "C" fn(this: *const User) -> bool,

    get_steam_id: extern "C" fn(this: *const User) -> u64,

    initiate_game_connection: extern "C" fn(
        this: *const User,
        auth_blob: *const (),
        auth_blob_max: i32,
        steam_id_game_server: u64,
        ip: u32,
        port: u16,
        secure: bool,
    ) -> i32,

    terminate_game_connection: extern "C" fn(this: *const User, ip: u32, port: u16),

    track_app_usage_event: extern "C" fn(
        this: *const User,
        game_id: u64,
        app_usage_event: i32,
        pch_extra_info: *const u8,
    ),

    get_user_data_folder:
        extern "C" fn(this: *const User, pch_buffer: *mut u8, cub_buffer: i32) -> bool,

    start_voice_recording: extern "C" fn(this: *const User),

    stop_voice_recording: extern "C" fn(this: *const User),

    // TODO: voice result
    get_available_voice: extern "C" fn(
        this: *const User,
        pcb_compressed: *const u32,
        pcb_uncompressed: *const u32,
        uncompressed_voice_desired_sample_rate: u32,
    ) -> i32,

    // TODO: voice result
    get_voice: extern "C" fn(
        this: *const User,
        want_compressed: bool,
        dst_buf: *mut (),
        dst_buf_len: u32,
        bytes_written: u32,
        want_uncompress: bool,
        uncompressed_dst_buf: *mut (),
        uncompressed_dst_buf_len: u32,
        uncompressed_bytes_written: u32,
        uncompressed_voice_desired_sample_rate: u32,
    ) -> i32,

    // TODO: voice result
    decompress_voice: extern "C" fn(
        this: *const User,
        compressed_buf: *const (),
        compressed: u32,
        dst_buf: *mut (),
        dst_buf_len: u32,
        bytes_written: u32,
        desired_sample_rate: u32,
    ) -> i32,

    get_voice_optimal_sample_rate: extern "C" fn(this: *const User) -> u32,

    get_auth_session_ticket: extern "C" fn(
        this: *const User,
        ticket: *const (),
        auth_ticket: *const (),
        pcb_ticket: *const u32,
    ) -> i32,

    // todo auth result
    begin_auth_session: extern "C" fn(
        this: *const User,
        auth_ticket: *const (),
        auth_ticket_len: i32,
        steam_id: i32,
    ) -> i32,

    end_auth_session: extern "C" fn(this: *const User, steam_id: i32),

    cancel_auth_ticket: extern "C" fn(this: *const User, auth_ticket_handle: i32),

    user_has_license_for_app: extern "C" fn(this: *const User, steam_id: i32, app_id: i32) -> i32,

    is_behind_nat: extern "C" fn(this: *const User) -> bool,

    advertise_game:
        extern "C" fn(this: *const User, game_server_id: u32, ip: u32, port: u16) -> bool,

    request_encrypted_app_ticket: extern "C" fn(
        this: *const User,
        data_to_include: *const (),
        data_to_include_len: i32,
    ) -> i32,

    get_encrypted_app_ticket: extern "C" fn(
        this: *const User,
        ticket: *const (),
        max_ticket: i32,
        pcb_ticket: *const u32,
    ) -> bool,

    get_game_badge_level: extern "C" fn(this: *const User, series: i32, foil: i32) -> i32,

    get_player_steam_level: extern "C" fn(this: *const User) -> i32,

    request_store_url: extern "C" fn(this: *const User, pch_redirect_url: *const u8) -> i32,
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetHSteamUser(this: *const User) -> UserHandle {
    debug!();

    69
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_BLoggedOn(this: *const User) -> bool {
    debug!();

    true
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetSteamID(this: *const User) -> u64 {
    debug!();

    69
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_InitiateGameConnection(
    this: *const User,
    auth_blob: *const (),
    auth_blob_max: i32,
    steam_id_game_server: u64,
    ip: u32,
    port: u16,
    secure: bool,
) -> i32 {
    debug!();

    69
}

#[no_mangle]
extern "C" fn SteamAPI_ISteamUser_TerminateGameConnection(this: *const User, ip: u32, port: u16) {
    debug!();
}

#[no_mangle]
extern "C" fn SteamAPI_ISteamUser_TrackAppUsageEvent(
    this: *const User,
    game_id: u64,
    app_usage_event: i32,
    pch_extra_info: *const u8,
) {
    debug!();
}

#[no_mangle]
extern "C" fn SteamAPI_ISteamUser_GetUserDataFolder(
    this: *const User,
    pch_buffer: *mut u8,
    cub_buffer: i32,
) -> bool {
    debug!();
    let path = "/tmp/steamapi";

    unsafe {
        ptr::copy_nonoverlapping(path.as_ptr(), pch_buffer, path.len());
    }

    true
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_StartVoiceRecording(this: *const User) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_StopVoiceRecording(this: *const User) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetAvailableVoice(
    this: *const User,
    pcb_compressed: *const u32,
    pcb_uncompressed: *const u32,
    uncompressed_voice_desired_sample_rate: u32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetVoice(
    this: *const User,
    want_compressed: bool,
    dst_buf: *mut (),
    dst_buf_len: u32,
    bytes_written: u32,
    want_uncompress: bool,
    uncompressed_dst_buf: *mut (),
    uncompressed_dst_buf_len: u32,
    uncompressed_bytes_written: u32,
    uncompressed_voice_desired_sample_rate: u32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_DecompressVoice(
    this: *const User,
    compressed_buf: *const (),
    compressed: u32,
    dst_buf: *mut (),
    dst_buf_len: u32,
    bytes_written: u32,
    desired_sample_rate: u32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetVoiceOptimalSampleRate(this: *const User) -> u32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetAuthSessionTicket(
    this: *const User,
    ticket: *const (),
    auth_ticket: *const (),
    pcb_ticket: *const u32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_BeginAuthSession(
    this: *const User,
    auth_ticket: *const (),
    auth_ticket_len: i32,
    steam_id: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_EndAuthSession(this: *const User, steam_id: i32) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_CancelAuthTicket(this: *const User, auth_ticket_handle: i32) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_UserHasLicenseForApp(
    this: *const User,
    steam_id: i32,
    app_id: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_BIsBehindNAT(this: *const User) -> bool {
    debug!();

    true
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_AdvertiseGame(
    this: *const User,
    game_server_id: u32,
    ip: u32,
    port: u16,
) -> bool {
    debug!();

    false
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_RequestEncryptedAppTicket(
    this: *const User,
    data_to_include: *const (),
    data_to_include_len: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetEncryptedAppTicket(
    this: *const User,
    ticket: *const (),
    max_ticket: i32,
    pcb_ticket: *const u32,
) -> bool {
    debug!();

    false
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetGameBadgeLevel(
    this: *const User,
    series: i32,
    foil: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_GetPlayerSteamLevel(this: *const User) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUser_RequestStoreAuthURL(
    this: *const User,
    pch_redirect_url: *const u8,
) -> i32 {
    debug!();

    0
}
