#![allow(unused_variables)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_type_name)]
#![feature(type_name_of_val)]

use std::ffi::CStr;

pub use account_kind::AccountKind;
pub use apps::Apps;
pub use client::Client;
pub use error::Error;
pub use friends::Friends;
pub use matchmaking::Matchmaking;
pub use matchmaking_servers::MatchmakingServers;
pub use networking::Networking;
pub use remote_storage::RemoteStorage;
pub use screenshots::Screenshots;
pub use universe::Universe;
pub use user::User;
pub use user_stats::UserStats;
pub use utils::Utils;

mod account_kind;
mod error;
mod universe;

pub mod apps;
pub mod client;
pub mod friends;
pub mod matchmaking;
pub mod matchmaking_servers;
pub mod networking;
pub mod remote_storage;
pub mod screenshots;
pub mod user;
pub mod user_stats;
pub mod utils;

pub(crate) mod macros;

pub type PipeHandle = i32;
pub type UserHandle = i32;

pub static FAKE_APPS: Apps = Apps::new();
pub static FAKE_CLIENT: Client = Client::new();
pub static FAKE_FRIENDS: Friends = Friends::new();
pub static FAKE_MATCHMAKING: Matchmaking = Matchmaking::new();
pub static FAKE_MATCHMAKING_SERVERS: MatchmakingServers = MatchmakingServers::new();
pub static FAKE_NETWORKING: Networking = Networking::new();
pub static FAKE_REMOTE_STORAGE: RemoteStorage = RemoteStorage::new();
pub static FAKE_SCREENSHOTS: Screenshots = Screenshots::new();
pub static FAKE_USER: User = User::new();
pub static FAKE_USER_STATS: UserStats = UserStats::new();
pub static FAKE_UTILS: Utils = Utils::new();

#[repr(C)]
pub struct GameServer {}

#[repr(C)]
pub struct GameServerStats {}

#[no_mangle]
pub extern "C" fn SteamAPI_GetHSteamPipe(this: *const Client) -> PipeHandle {
    debug!();

    69
}

#[no_mangle]
pub extern "C" fn SteamAPI_GetHSteamUser(this: *const Client) -> UserHandle {
    debug!();

    69
}

#[no_mangle]
pub extern "C" fn SteamAPI_InitSafe() {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_Init(params: *const ()) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_RegisterCallResult(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_RegisterCallback(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_RunCallbacks(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_SetBreakpadAppID(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_SetTryCatchCallbacks(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_UnregisterCallResult(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_UnregisterCallback(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_UseBreakpadCrashHandler(
    pch_version: *const u8,
    pch_date: *const u8,
    pch_time: *const u8,
    full_memory_dumps: bool,
    context: *const (),
    callback: *const (),
) {
    debug!();

    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    println!("pch_date = {:?}", unsafe {
        CStr::from_ptr(pch_date.cast())
    });

    println!("pch_time = {:?}", unsafe {
        CStr::from_ptr(pch_time.cast())
    });

    println!("full_memory_dumps = {full_memory_dumps:?}");
}

#[no_mangle]
pub extern "C" fn SteamAPI_WriteMiniDump(this: *const Client) {
    debug!();
}
#[no_mangle]
pub extern "C" fn SteamGameServer_GetHSteamPipe(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn GetHSteamPipe(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamGameServer_GetHSteamUser(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamGameServer_GetIPCCallCount(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamGameServer_RunCallbacks(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamGameServer_Shutdown(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamInternal_CreateInterface(version: *const u8) -> *const Client {
    debug!();

    println!("version = {:?}", unsafe { CStr::from_ptr(version.cast()) });

    &FAKE_CLIENT
}

#[no_mangle]
pub extern "C" fn SteamInternal_GameServer_Init(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_SetMiniDumpComment(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_RestartAppIfNecessary(this: *const Client) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_Shutdown(this: *const Client) {
    debug!();
}
