use super::{api_fn, debug, virtual_struct};
use core::ptr;

virtual_struct! { MatchmakingServers {
    fn request_internet_server_list(&self, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32,
    fn request_lan_server_list(&self, app: i32, response: *mut ()) -> i32,
    fn request_friends_server_list(&self, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32,
    fn request_favourites_server_list(&self, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32,
    fn request_history_server_list(&self, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32,
    fn request_spectator_server_list(&self, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32,
    fn release_request(&self, request: i32) -> (),
    fn get_server_details(&self, request: i32, server: i32) -> *const (),
    fn cancel_query(&self, request: i32) -> (),
    fn refresh_query(&self, request: i32) -> (),
    fn is_refreshing(&self, request: i32) -> bool,
} }

impl MatchmakingServers {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                request_internet_server_list:
                    SteamAPI_ISteamMatchmakingServers_RequestInternetServerList,
                request_lan_server_list: SteamAPI_ISteamMatchmakingServers_RequestLanServerList,
                request_friends_server_list:
                    SteamAPI_ISteamMatchmakingServers_RequestFriendsServerList,
                request_favourites_server_list:
                    SteamAPI_ISteamMatchmakingServers_RequestFavouritesServerList,
                request_history_server_list:
                    SteamAPI_ISteamMatchmakingServers_RequestHistoryServerList,
                request_spectator_server_list:
                    SteamAPI_ISteamMatchmakingServers_RequestSpectatorServerList,
                release_request: SteamAPI_ISteamMatchmakingServers_ReleaseRequest,
                get_server_details: SteamAPI_ISteamMatchmakingServers_GetServerDetails,
                cancel_query: SteamAPI_ISteamMatchmakingServers_CancelQuery,
                refresh_query: SteamAPI_ISteamMatchmakingServers_RefreshQuery,
                is_refreshing: SteamAPI_ISteamMatchmakingServers_IsRefreshing,
            },
        }
    }
}

api_fn! { RequestInternetServerList(&MatchmakingServers, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32 {
    debug!();

    0
} }

api_fn! { RequestLanServerList(&MatchmakingServers, app: i32, response: *mut ()) -> i32 {
    debug!();

    0
} }

api_fn! { RequestFriendsServerList(&MatchmakingServers, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32 {
    debug!();

    0
} }

api_fn! { RequestFavouritesServerList(&MatchmakingServers, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32 {
    debug!();

    0
} }

api_fn! { RequestHistoryServerList(&MatchmakingServers, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32 {
    debug!();

    0
} }

api_fn! { RequestSpectatorServerList(&MatchmakingServers, app: i32, filters: *const *const (), filters_len: u32, response: *mut ()) -> i32 {
    debug!();

    0
} }

api_fn! { ReleaseRequest(&MatchmakingServers, request: i32) -> () {
    debug!();
} }

api_fn! { GetServerDetails(&MatchmakingServers, request: i32, server: i32) -> *const () {
    debug!();

    ptr::null()
} }

api_fn! { CancelQuery(&MatchmakingServers, request: i32) -> () {
    debug!();
} }

api_fn! { RefreshQuery(&MatchmakingServers, request: i32) -> () {
    debug!();
} }

api_fn! { IsRefreshing(&MatchmakingServers, request: i32) -> bool {
    debug!();

    true
} }
