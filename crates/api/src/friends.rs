use super::{api_fn, debug, virtual_struct};
use core::ptr;

virtual_struct! { Friends {
    fn get_persona_name(&self) -> *const u8,
    fn set_persona_name(&self, name: *const u8) -> i32,
    fn get_persona_state(&self) -> i32,
    fn get_friend_count(&self, flags: i32) -> i32,
    fn get_friend_by_index(&self, index: i32, flags: i32) -> u64,
    fn get_friend_relationship(&self, friend_id: u64) -> i32,
    fn get_friend_persona_state(&self, friend_id: u64) -> i32,
    fn get_friend_persona_name(&self, friend_id: u64) -> *const u8,
    fn get_friend_game_played(&self, friend_id: u64, game_info: *mut ()) -> bool,
    fn get_friend_persona_name_history(&self, friend_id: u64, persona_name: i32) -> *const u8,
    fn get_friend_steam_level(&self, friend_id: u64) -> i32,
    fn get_player_nickname(&self, user_id: u64) -> *const u8,
    fn get_friends_group_count(&self) -> i32,
    fn get_friends_group_id_by_index(&self, index: i32) -> u64,
} }

impl Friends {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                get_persona_name: SteamAPI_ISteamFriends_GetPersonaName,
                set_persona_name: SteamAPI_ISteamFriends_SetPersonaName,
                get_persona_state: SteamAPI_ISteamFriends_GetPersonaState,
                get_friend_count: SteamAPI_ISteamFriends_GetFriendCount,
                get_friend_by_index: SteamAPI_ISteamFriends_GetFriendByIndex,
                get_friend_relationship: SteamAPI_ISteamFriends_GetFriendRelationship,
                get_friend_persona_state: SteamAPI_ISteamFriends_GetFriendPersonaState,
                get_friend_persona_name: SteamAPI_ISteamFriends_GetFriendPersonaName,
                get_friend_game_played: SteamAPI_ISteamFriends_GetFriendGamePlayed,
                get_friend_persona_name_history: SteamAPI_ISteamFriends_GetFriendPersonaNameHistory,
                get_friend_steam_level: SteamAPI_ISteamFriends_GetFriendSteamLevel,
                get_player_nickname: SteamAPI_ISteamFriends_GetPlayerNickname,
                get_friends_group_count: SteamAPI_ISteamFriends_GetFriendsGroupCount,
                get_friends_group_id_by_index: SteamAPI_ISteamFriends_GetFriendsGroupIDByIndex,
            },
        }
    }
}

api_fn! { GetPersonaName(&Friends) -> *const u8 {
    debug!();

    const NAME: &str = "elysian\0";

    NAME.as_ptr()
} }

api_fn! { SetPersonaName(
    &Friends,
    name: *const u8,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetPersonaState(&Friends) -> i32 {
    debug!();

    0
} }

api_fn! { GetFriendCount(&Friends, flags: i32) -> i32 {
    debug!();

    0
} }

api_fn! { GetFriendByIndex(
    &Friends,
    index: i32,
    flags: i32,
) -> u64 {
    debug!();

    0
} }

api_fn! { GetFriendRelationship(
    &Friends,
    friend: u64,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetFriendPersonaState(
    &Friends,
    friend: u64,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetFriendPersonaName(
    &Friends,
    friend: u64,
) -> *const u8 {
    debug!();

    let name = "rust\0";

    name.as_ptr()
} }

api_fn! { GetFriendGamePlayed(
    &Friends,
    friend: u64,
    game_info: *mut (),
) -> bool {
    debug!();

    false
} }

api_fn! { GetFriendPersonaNameHistory(
    &Friends,
    friend: u64,
    index: i32,
) -> *const u8 {
    debug!();

    ptr::null()
} }

api_fn! { GetFriendSteamLevel(
    &Friends,
    friend: u64,
) -> i32 {
    debug!();

    9
} }

api_fn! { GetPlayerNickname(
    &Friends,
    friend: u64,
) -> *const u8 {
    debug!();

    ptr::null()
} }

api_fn! { GetFriendsGroupCount(&Friends) -> i32 {
    debug!();

    0
} }

api_fn! { GetFriendsGroupIDByIndex(
    &Friends,
    group: i32,
) -> u64 {
    debug!();

    0
} }

api_fn! { GetFriendsGroupName(
    &Friends,
    group: i32,
) -> *const u8 {
    debug!();

    ptr::null()
} }

api_fn! { GetFriendsGroupMembersCount(
    &Friends,
    group: i32,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetFriendsGroupMembersList(
    &Friends,
    group: i32,
    members: *mut i32,
    members_len: i32,
) -> i32 {
    debug!();

    0
} }

api_fn! { HasFriend(
    &Friends,
    index: i32,
    flags: i32,
) -> bool {
    debug!();

    false
} }

api_fn! { GetClanCount(&Friends) -> i32 {
    debug!();

    0
} }

api_fn! { GetClanByIndex(&Friends, clan: i32) -> u64 {
    debug!();

    0
} }

api_fn! { GetClanName(
    &Friends,
    clan_id: u32,
) -> *const u8 {
    debug!();

    ptr::null()
} }

api_fn! { GetClanTag(
    &Friends,
    clan_id: u32,
) -> *const u8 {
    debug!();

    let tag = "eleutheria\0";

    tag.as_ptr()
} }

api_fn! { GetClanActivityCounts(
    &Friends,
    clan_id: u64,
    online: *const i32,
    in_game: *const i32,
    chatting: *const i32,
) -> u64 {
    debug!();

    0
} }

api_fn! { DownloadClanActivityCounts(
    &Friends,
    clan_id: u64,
    ids: *const i64,
    len: i32,
) -> u64 {
    debug!();

    0
} }

api_fn! { GetFriendCountFromSource(
    &Friends,
    id: u64,
) -> i32 {
    debug!();

    0
} }

api_fn! { IsUserInSource(
    &Friends,
    id: u64,
    id_source: u64,
) -> bool {
    debug!();

    false
} }

api_fn! { SetInGameVoiceSpeaking(
    &Friends,
    id: u64,
    speaking: bool,
) -> () {
    debug!();
} }

api_fn! { ActivateGameOverlay(
    &Friends,
    pch_dialog: *const u8,
) -> () {
    debug!();
} }

api_fn! { ActivateGameOverlayToUser(
    &Friends,
    pch_dialog: *const u8,
    user_id: u64,
) -> () {
    debug!();
} }

api_fn! { ActivateGameOverlayToWebPage(
    &Friends,
    pch_url: *const u8,
) -> () {
    debug!();
} }

api_fn! { ActivateGameOverlayToStore(
    &Friends,
    app_id: i32,
    flag: i32,
) -> () {
    debug!();
} }

api_fn! { SetPlayedWith(&Friends, user_id: i32) -> () {
    debug!();
} }

api_fn! { ActivateGameOverlayInviteDialog(
    &Friends,
    lobby_id: i32,
) -> () {
    debug!();
} }

api_fn! { GetSmallFriendAvatar(
    &Friends,
    user_id: i32,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetMediumFriendAvatar(
    &Friends,
    user_id: i32,
) -> i32 {
    debug!();

    0
} }

api_fn! { GetLargeFriendAvatar(
    &Friends,
    user_id: i32,
) -> i32 {
    debug!();

    0
} }

api_fn! { RequestUserInformation(
    &Friends,
    user_id: i32,
    require_name_only: bool,
) -> bool {
    debug!();

    false
} }

api_fn! { RequestClanOfficerList(
    &Friends,
    clan_id: i32,
) -> i32 {
    debug!();

    0
} }
