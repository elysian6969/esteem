use super::{api_fn, debug, virtual_struct, UserHandle};
use core::ptr;

virtual_struct! { User {
    fn get_handle(&self) -> UserHandle,
    fn logged_on(&self) -> bool,
    fn get_steam_id(&self) -> u64,
    fn initiate_game_connection(&self, auth_blob: *const (), auth_blob_max: i32, steam_id_game_server: u64, ip: u32, port: u16, secure: bool) -> i32,
    fn terminate_game_connection(&self, ip: u32, port: u16) -> (),
    fn track_app_usage_event(&self, game_id: u64, app_usage_event: i32, pch_extra_info: *const u8) -> (),
    fn get_user_data_folder(&self, pch_buffer: *mut u8, cub_buffer: i32) -> bool,
    fn start_voice_recording(&self) -> (),
    fn stop_voice_recording(&self) -> (),
    fn get_available_voice(&self, pcb_compressed: *const u32, pcb_uncompressed: *const u32, uncompressed_voice_desired_sample_rate: u32) -> i32,
    fn get_voice(&self, want_compressed: bool, dst_buf: *mut (), dst_buf_len: u32, bytes_written: u32, want_uncompress: bool, uncompressed_dst_buf: *mut (), uncompressed_dst_buf_len: u32, uncompressed_bytes_written: u32, uncompressed_voice_desired_sample_rate: u32) -> i32,
    fn decompress_voice(&self, compressed_buf: *const (), compressed: u32, dst_buf: *mut (), dst_buf_len: u32, bytes_written: u32, desired_sample_rate: u32) -> i32,
    fn get_voice_optimal_sample_rate(&self) -> u32,
    fn get_auth_session_ticket(&self, ticket: *const (), auth_ticket: *const (), pcb_ticket: *const u32) -> i32,
    fn begin_auth_session(&self, auth_ticket: *const (), auth_ticket_len: i32, steam_id: i32) -> i32,
    fn end_auth_session(&self, steam_id: i32) -> (),
    fn cancel_auth_ticket(&self, auth_ticket_handle: i32) -> (),
    fn user_has_license_for_app(&self, steam_id: i32, app_id: i32) -> i32,
    fn is_behind_nat(&self) -> bool,
    fn advertise_game(&self, game_server_id: u32, ip: u32, port: u16) -> bool,
    fn request_encrypted_app_ticket(&self, data_to_include: *const (), data_to_include_len: i32) -> i32,
    fn get_encrypted_app_ticket(&self, ticket: *const (), max_ticket: i32, pcb_ticket: *const u32) -> bool,
    fn get_game_badge_level(&self, series: i32, foil: i32) -> i32,
    fn get_player_steam_level(&self) -> i32,
    fn request_store_url(&self, pch_redirect_url: *const u8) -> i32,
} }

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

api_fn! { GetHSteamUser(&User) -> UserHandle {
    debug!();

    69
} }

api_fn! { BLoggedOn(&User) -> bool {
    debug!();

    true
} }

api_fn! { GetSteamID(&User) -> u64 {
    debug!();

    let id = 76561199254102667;

    println!("id = {id:?}");

    id
} }

api_fn! { InitiateGameConnection(
    &User,
    auth_blob: *const (),
    auth_blob_max: i32,
    steam_id_game_server: u64,
    ip: u32,
    port: u16,
    secure: bool,
) -> i32 {
    debug!();

    69
} }

api_fn! { TerminateGameConnection(&User, ip: u32, port: u16) -> () {
    debug!();
} }

api_fn! { TrackAppUsageEvent(
    &User,
    game_id: u64,
    app_usage_event: i32,
    pch_extra_info: *const u8,
) -> () {
    debug!();
} }

api_fn! { GetUserDataFolder(
    &User,
    pch_buffer: *mut u8,
    cub_buffer: i32,
) -> bool {
    debug!();

    let path = "/tmp/steamapi";

    unsafe {
        ptr::copy_nonoverlapping(path.as_ptr(), pch_buffer, path.len());
    }

    true
} }

api_fn! { StartVoiceRecording(&User) -> () {
    debug!();
} }

api_fn! { StopVoiceRecording(&User) -> () {
    debug!();
} }

api_fn! { GetAvailableVoice(
    &User,
    pcb_compressed: *const u32,
    pcb_uncompressed: *const u32,
    uncompressed_voice_desired_sample_rate: u32,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetVoice(
    &User,
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
} }

api_fn! { DecompressVoice(
    &User,
    compressed_buf: *const (),
    compressed: u32,
    dst_buf: *mut (),
    dst_buf_len: u32,
    bytes_written: u32,
    desired_sample_rate: u32,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetVoiceOptimalSampleRate(&User) -> u32 {
    debug!();

    0
} }

api_fn! { GetAuthSessionTicket(
    &User,
    ticket: *const (),
    auth_ticket: *const (),
    pcb_ticket: *const u32,
) -> i32 {
    debug!();

    0
} }

api_fn! { BeginAuthSession(
    &User,
    auth_ticket: *const (),
    auth_ticket_len: i32,
    steam_id: i32,
) -> i32 {
    debug!();

        println!("auth_ticket = {auth_ticket:?}");
        println!("auth_ticket_len = {auth_ticket_len:?}");
        println!("steam_id = {steam_id:?}");

    0
} }

api_fn! { EndAuthSession(&User, steam_id: i32) -> () {
    debug!();

        println!("steam_id = {steam_id:?}");
} }

api_fn! { CancelAuthTicket(&User, auth_ticket_handle: i32) -> () {
    debug!();

        println!("auth_ticket_handle = {auth_ticket_handle:?}");
} }

api_fn! { UserHasLicenseForApp(
    &User,
    steam_id: i32,
    app_id: i32,
) -> i32 {
    debug!();

    0
} }

api_fn! { BIsBehindNAT(&User) -> bool {
    debug!();

    true
} }

api_fn! { AdvertiseGame(
    &User,
    game_server_id: u32,
    ip: u32,
    port: u16,
) -> bool {
    debug!();

    false
} }

api_fn! { RequestEncryptedAppTicket(
    &User,
    data_to_include: *const (),
    data_to_include_len: i32,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetEncryptedAppTicket(
    &User,
    ticket: *const (),
    max_ticket: i32,
    pcb_ticket: *const u32,
) -> bool {
    debug!();

    false
} }

api_fn! { GetGameBadgeLevel(
    &User,
    series: i32,
    foil: i32,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetPlayerSteamLevel(&User) -> i32 {
    debug!();

    0
} }

api_fn! { RequestStoreAuthURL(
    &User,
    pch_redirect_url: *const u8,
) -> i32 {
    debug!();

    0
} }
