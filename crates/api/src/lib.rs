#![allow(unused_variables)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_type_name)]
#![feature(type_name_of_val)]

use std::ffi::CStr;

pub use account_kind::AccountKind;
pub use client::Client;
pub use error::Error;
pub use friends::Friends;
pub use user::User;

mod account_kind;
mod error;

pub mod client;
pub mod friends;
pub mod user;

pub(crate) mod macros;

pub type PipeHandle = i32;
pub type UserHandle = i32;

pub static FAKE_CLIENT: Client = Client::new();
pub static FAKE_FRIENDS: Friends = Friends::new();
pub static FAKE_USER: User = User::new();

#[repr(C)]
pub struct GameServer {}

#[repr(C)]
pub struct GameServerStats {}

#[repr(C)]
pub struct Utils {}

#[repr(C)]
pub struct Matchmaking {}

#[repr(C)]
pub struct MatchmakingServers {}

#[repr(C)]
pub struct UserStats {}

#[repr(C)]
pub struct Apps {}

#[repr(C)]
pub struct Networking {}

#[repr(C)]
pub struct RemoteStorage {}

#[repr(C)]
pub struct Screenshots {}

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
pub extern "C" fn SteamAPI_InitSafe(this: *const Client) {
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
