#![allow(unused_variables)]
#![feature(const_fn_fn_ptr_basics)]

use std::ffi::CStr;

pub use app_id::AppId;
pub use apps::Apps;
pub use client::Client;
pub use controller::Controller;
pub use error::Error;
pub use friends::Friends;
pub use game_server::GameServer;
pub use game_server_stats::GameServerStats;
pub use html_surface::HTMLSurface;
pub use matchmaking::Matchmaking;
pub use matchmaking_servers::MatchmakingServers;
pub use networking::Networking;
pub use remote_storage::RemoteStorage;
pub use screenshots::Screenshots;
pub use user::User;
pub use user_stats::UserStats;
pub use utils::Utils;

mod app_id;
mod error;

pub mod apps;
pub mod client;
pub mod controller;
pub mod friends;
pub mod game_server;
pub mod game_server_stats;
pub mod html_surface;
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

// im sick of doing this
pub static P100_FAKE: usize = 69420;

pub static FAKE_APPS: Apps = Apps::new();
pub static FAKE_CLIENT: Client = Client::new();
pub static FAKE_CONTROLLER: Controller = Controller::new();
pub static FAKE_FRIENDS: Friends = Friends::new();
pub static FAKE_GAME_SERVER: GameServer = GameServer::new();
pub static FAKE_GAME_SERVER_STATS: GameServerStats = GameServerStats::new();
pub static FAKE_HTML_SURFACE: HTMLSurface = HTMLSurface::new();
pub static FAKE_MATCHMAKING: Matchmaking = Matchmaking::new();
pub static FAKE_MATCHMAKING_SERVERS: MatchmakingServers = MatchmakingServers::new();
pub static FAKE_NETWORKING: Networking = Networking::new();
pub static FAKE_REMOTE_STORAGE: RemoteStorage = RemoteStorage::new();
pub static FAKE_SCREENSHOTS: Screenshots = Screenshots::new();
pub static FAKE_USER: User = User::new();
pub static FAKE_USER_STATS: UserStats = UserStats::new();
pub static FAKE_UTILS: Utils = Utils::new();

pub const USER_ID: steam_id::SteamId =
    unsafe { steam_id::SteamId::new_unchecked(76561199254102667) };

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
pub extern "C" fn SteamAPI_RegisterCallback(callbacks: *const (), len: i32) {
    debug!();

    println!("callbacks = {callbacks:?}");
    println!("len = {len}");
}

#[no_mangle]
pub extern "C" fn SteamAPI_RunCallbacks() {
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
pub extern "C" fn SteamGameServer_GetHSteamPipe(this: *const Client) -> PipeHandle {
    debug!();

    69
}

#[no_mangle]
pub extern "C" fn GetHSteamPipe(this: *const Client) -> PipeHandle {
    debug!();

    69
}

#[no_mangle]
pub extern "C" fn SteamGameServer_GetHSteamUser(this: *const Client) -> UserHandle {
    debug!();

    69
}

#[no_mangle]
pub extern "C" fn SteamGameServer_GetIPCCallCount(this: *const Client) -> i32 {
    debug!();

    0
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
pub extern "C" fn SteamGameServerInternal_CreateInterface(ver: *const u8) -> *const () {
    debug!();

    &P100_FAKE as *const usize as *const ()
}

#[no_mangle]
pub extern "C" fn SteamInternal_GameServer_Init(
    ip: u32,
    port: u32,
    game_port: u16,
    query_port: u16,
    server_mode: u8,
    version_string: *const u8,
) -> bool {
    debug!();

    println!("ip = {ip}");
    println!("port = {port}");
    println!("game_port = {game_port}");
    println!("query_port = {query_port}");
    println!("server_mode = {server_mode}");
    println!("version_string = {:?}", unsafe {
        CStr::from_ptr(version_string.cast())
    });

    false
}

#[no_mangle]
pub extern "C" fn SteamAPI_SetMiniDumpComment(message: *const u8) {
    let message = unsafe { CStr::from_ptr(message.cast()).to_string_lossy() };
    let lines = message.lines();

    for line in lines {
        println!("{}: {}", frosting::function!(), line);
    }
}

#[no_mangle]
pub extern "C" fn SteamAPI_RestartAppIfNecessary(app_id: AppId) {
    println!("{}: app_id = {app_id}", frosting::function!());
}

#[no_mangle]
pub extern "C" fn SteamAPI_Shutdown() {
    debug!();
}
