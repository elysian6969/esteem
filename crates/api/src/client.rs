use super::{
    debug, AccountKind, Apps, Friends, GameServer, GameServerStats, Matchmaking,
    MatchmakingServers, Networking, PipeHandle, RemoteStorage, Screenshots, User, UserHandle,
    UserStats, Utils,
};
use core::ptr;
use std::ffi::CStr;

#[repr(C)]
pub struct Client {
    vtable: &'static VTable,
}

impl Client {
    pub const fn new() -> Self {
        Client {
            vtable: &VTable {
                create_pipe: SteamAPI_ISteamClient_CreateSteamPipe,
                release_pipe: SteamAPI_ISteamClient_BReleaseSteamPipe,
                connect_to_global_user: SteamAPI_ISteamClient_ConnectToGlobalUser,
                create_local_user: SteamAPI_ISteamClient_CreateLocalUser,
                release_user: SteamAPI_ISteamClient_ReleaseUser,
                get_user: SteamAPI_ISteamClient_GetISteamUser,
                get_game_server: SteamAPI_ISteamClient_GetISteamGameServer,
                set_local_ip_binding: SteamAPI_ISteamClient_SetLocalIPBinding,
                get_friends: SteamAPI_ISteamClient_GetISteamFriends,
                get_utils: SteamAPI_ISteamClient_GetISteamUtils,
                get_matchmaking: SteamAPI_ISteamClient_GetISteamMatchmaking,
                get_matchmaking_servers: SteamAPI_ISteamClient_GetISteamMatchmakingServers,
                get_generic_interface: SteamAPI_ISteamClient_GetISteamGenericInterface,
                get_user_stats: SteamAPI_ISteamClient_GetISteamUserStats,
                get_game_server_stats: SteamAPI_ISteamClient_GetISteamGameServerStats,
                get_apps: SteamAPI_ISteamClient_GetISteamApps,
                get_networking: SteamAPI_ISteamClient_GetISteamNetworking,
                get_remote_storage: SteamAPI_ISteamClient_GetISteamRemoteStorage,
                get_screenshots: SteamAPI_ISteamClient_GetISteamScreenshots,
            },
        }
    }
}

#[repr(C)]
struct VTable {
    create_pipe: extern "C" fn(this: *const Client) -> PipeHandle,

    release_pipe: extern "C" fn(this: *const Client, pipe_handle: PipeHandle) -> bool,

    connect_to_global_user:
        extern "C" fn(this: *const Client, pipe_handle: PipeHandle) -> UserHandle,

    create_local_user: extern "C" fn(
        this: *const Client,
        pipe_handle: *const PipeHandle,
        account_kind: AccountKind,
    ) -> UserHandle,

    release_user:
        extern "C" fn(this: *const Client, pipe_handle: PipeHandle, user_handle: UserHandle),

    get_user: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const User,

    get_game_server: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const GameServer,

    set_local_ip_binding: extern "C" fn(this: *const Client, ip: u32, port: u16),

    get_friends: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const Friends,

    get_utils: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const Utils,

    get_matchmaking: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const Matchmaking,

    get_matchmaking_servers: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const MatchmakingServers,

    get_generic_interface: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const (),

    get_user_stats: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const UserStats,

    get_game_server_stats: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const GameServerStats,

    get_apps: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const Apps,

    get_networking: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const Networking,

    get_remote_storage: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const RemoteStorage,

    get_screenshots: extern "C" fn(
        this: *const Client,
        user_handle: UserHandle,
        pipe_handle: PipeHandle,
        pch_version: *const u8,
    ) -> *const Screenshots,
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_CreateSteamPipe(this: *const Client) -> PipeHandle {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_BReleaseSteamPipe(
    this: *const Client,
    pipe_handle: PipeHandle,
) -> bool {
    debug!();

    println!("pipe_handle = {pipe_handle:?}");

    true
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_ConnectToGlobalUser(
    this: *const Client,
    pipe_handle: PipeHandle,
) -> UserHandle {
    debug!();

    println!("pipe_handle = {pipe_handle:?}");

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_CreateLocalUser(
    this: *const Client,
    pipe_handle: *const PipeHandle,
    account_kind: AccountKind,
) -> UserHandle {
    debug!();

    println!("pipe_handle = {pipe_handle:?}");
    println!("account_kind = {account_kind:?}");

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_ReleaseUser(
    this: *const Client,
    pipe_handle: PipeHandle,
    user_handle: UserHandle,
) {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamUser(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const User {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");

    &super::FAKE_USER
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamGameServer(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const GameServer {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_SetLocalIPBinding(this: *const Client, ip: u32, port: u16) {
    debug!();

    println!("port = {:?}", port);
    println!("ip = {:?}", ip);
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamFriends(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const Friends {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamUtils(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const Utils {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamMatchmaking(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const Matchmaking {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamMatchmakingServers(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const MatchmakingServers {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamGenericInterface(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const () {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamUserStats(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const UserStats {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamGameServerStats(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const GameServerStats {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamApps(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const Apps {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamNetworking(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const Networking {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamRemoteStorage(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const RemoteStorage {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamClient_GetISteamScreenshots(
    this: *const Client,
    user_handle: UserHandle,
    pipe_handle: PipeHandle,
    pch_version: *const u8,
) -> *const Screenshots {
    debug!();

    println!("user_handle = {user_handle:?}");
    println!("pipe_handle = {pipe_handle:?}");
    println!("pch_version = {:?}", unsafe {
        CStr::from_ptr(pch_version.cast())
    });

    ptr::null()
}
