use super::{debug, virtual_struct};
use core::ptr;

virtual_struct! { Friends {
    fn get_persona_name(&self) -> *const u8,
    fn set_persona_name(&self, name: *const u8) -> i32,
    fn get_persona_state(&self) -> i32,
    fn get_friend_count(&self, flags: i32) -> i32,
    fn get_friend_by_index(&self, index: i32, flags: i32) -> u64,
    fn get_friend_relationship(&self, friend_id: u64) -> i32,
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
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetPersonaName(this: *const Friends) -> *const u8 {
    debug!();

    const NAME: &str = "elysian\0";

    NAME.as_ptr()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_SetPersonaName(
    this: *const Friends,
    name: *const u8,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetPersonaState(this: *const Friends) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendCount(this: *const Friends, flags: i32) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendByIndex(
    this: *const Friends,
    index: i32,
    flags: i32,
) -> u64 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendRelationship(
    this: *const Friends,
    friend: u64,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendPersonaState(
    this: *const Friends,
    friend: u64,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendPersonaName(
    this: *const Friends,
    friend: u64,
) -> *const u8 {
    debug!();

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendGamePlayed(
    this: *const Friends,
    friend: u64,
    game_info: *mut (),
) -> bool {
    debug!();

    false
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendPersonaNameHistory(
    this: *const Friends,
    friend: u64,
    index: i32,
) -> *const u8 {
    debug!();

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendSteamLevel(
    this: *const Friends,
    friend: u64,
) -> i32 {
    debug!();

    9
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetPlayerNickname(
    this: *const Friends,
    friend: u64,
) -> *const u8 {
    debug!();

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendsGroupCount(this: *const Friends) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendsGroupIDByIndex(
    this: *const Friends,
    group: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendsGroupName(
    this: *const Friends,
    group: i32,
) -> *const u8 {
    debug!();

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendsGroupMembersCount(
    this: *const Friends,
    group: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendsGroupMembersList(
    this: *const Friends,
    group: i32,
    members: *mut i32,
    members_len: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_HasFriend(
    this: *const Friends,
    index: i32,
    flags: i32,
) -> bool {
    debug!();

    false
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetClanCount(this: *const Friends) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetClanByIndex(this: *const Friends, clan: i32) -> u64 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetClanName(
    this: *const Friends,
    clan_id: u32,
) -> *const u8 {
    debug!();

    ptr::null()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetClanTag(
    this: *const Friends,
    clan_id: u32,
) -> *const u8 {
    debug!();

    let tag = "eleutheria\0";

    tag.as_ptr()
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetClanActivityCounts(
    this: *const Friends,
    clan_id: u64,
    online: *const i32,
    in_game: *const i32,
    chatting: *const i32,
) -> u64 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_DownloadClanActivityCounts(
    this: *const Friends,
    clan_id: u64,
    ids: *const i64,
    len: i32,
) -> u64 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetFriendCountFromSource(
    this: *const Friends,
    id: u64,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_IsUserInSource(
    this: *const Friends,
    id: u64,
    id_source: u64,
) -> bool {
    debug!();

    false
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(
    this: *const Friends,
    id: u64,
    speaking: bool,
) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_ActivateGameOverlay(
    this: *const Friends,
    pch_dialog: *const u8,
) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_ActivateGameOverlayToUser(
    this: *const Friends,
    pch_dialog: *const u8,
    user_id: u64,
) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_ActivateGameOverlayToWebPage(
    this: *const Friends,
    pch_url: *const u8,
) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_ActivateGameOverlayToStore(
    this: *const Friends,
    app_id: i32,
    flag: i32,
) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_SetPlayedWith(this: *const Friends, user_id: i32) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_ActivateGameOverlayInviteDialog(
    this: *const Friends,
    lobby_id: i32,
) {
    debug!();
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetSmallFriendAvatar(
    this: *const Friends,
    user_id: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetMediumFriendAvatar(
    this: *const Friends,
    user_id: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_GetLargeFriendAvatar(
    this: *const Friends,
    user_id: i32,
) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_RequestUserInformation(
    this: *const Friends,
    user_id: i32,
    require_name_only: bool,
) -> bool {
    debug!();

    false
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamFriends_RequestClanOfficerList(
    this: *const Friends,
    clan_id: i32,
) -> i32 {
    debug!();

    0
}

/*
    // returns the steamID of the clan owner
    virtual CSteamID GetClanOwner( CSteamID steamIDClan ) = 0;
    // returns the number of officers in a clan (including the owner)
    virtual int GetClanOfficerCount( CSteamID steamIDClan ) = 0;
    // returns the steamID of a clan officer, by index, of range [0,GetClanOfficerCount)
    virtual CSteamID GetClanOfficerByIndex( CSteamID steamIDClan, int iOfficer ) = 0;
    // if current user is chat restricted, he can't send or receive any text/voice chat messages.
    // the user can't see custom avatars. But the user can be online and send/recv game invites.
    // a chat restricted user can't add friends or join any groups.
    virtual uint32 GetUserRestrictions() = 0;

    // Rich Presence data is automatically shared between friends who are in the same game
    // Each user has a set of Key/Value pairs
    // Up to 20 different keys can be set
    // There are two magic keys:
    //		"status"  - a UTF-8 string that will show up in the 'view game info' dialog in the Steam friends list
    //		"connect" - a UTF-8 string that contains the command-line for how a friend can connect to a game
    // GetFriendRichPresence() returns an empty string "" if no value is set
    // SetRichPresence() to a NULL or an empty string deletes the key
    // You can iterate the current set of keys for a friend with GetFriendRichPresenceKeyCount()
    // and GetFriendRichPresenceKeyByIndex() (typically only used for debugging)
    virtual bool SetRichPresence( const char *pchKey, const char *pchValue ) = 0;
    virtual void ClearRichPresence() = 0;
    virtual const char *GetFriendRichPresence( CSteamID steamIDFriend, const char *pchKey ) = 0;
    virtual int GetFriendRichPresenceKeyCount( CSteamID steamIDFriend ) = 0;
    virtual const char *GetFriendRichPresenceKeyByIndex( CSteamID steamIDFriend, int iKey ) = 0;
    // Requests rich presence for a specific user.
    virtual void RequestFriendRichPresence( CSteamID steamIDFriend ) = 0;

    // rich invite support
    // if the target accepts the invite, the pchConnectString gets added to the command-line for launching the game
    // if the game is already running, a GameRichPresenceJoinRequested_t callback is posted containing the connect string
    // invites can only be sent to friends
    virtual bool InviteUserToGame( CSteamID steamIDFriend, const char *pchConnectString ) = 0;

    // recently-played-with friends iteration
    // this iterates the entire list of users recently played with, across games
    // GetFriendCoplayTime() returns as a unix time
    virtual int GetCoplayFriendCount() = 0;
    virtual CSteamID GetCoplayFriend( int iCoplayFriend ) = 0;
    virtual int GetFriendCoplayTime( CSteamID steamIDFriend ) = 0;
    virtual AppId_t GetFriendCoplayGame( CSteamID steamIDFriend ) = 0;

    // chat interface for games
    // this allows in-game access to group (clan) chats from in the game
    // the behavior is somewhat sophisticated, because the user may or may not be already in the group chat from outside the game or in the overlay
    // use ActivateGameOverlayToUser( "chat", steamIDClan ) to open the in-game overlay version of the chat
    CALL_RESULT( JoinClanChatRoomCompletionResult_t )
    virtual SteamAPICall_t JoinClanChatRoom( CSteamID steamIDClan ) = 0;
    virtual bool LeaveClanChatRoom( CSteamID steamIDClan ) = 0;
    virtual int GetClanChatMemberCount( CSteamID steamIDClan ) = 0;
    virtual CSteamID GetChatMemberByIndex( CSteamID steamIDClan, int iUser ) = 0;
    virtual bool SendClanChatMessage( CSteamID steamIDClanChat, const char *pchText ) = 0;
    virtual int GetClanChatMessage( CSteamID steamIDClanChat, int iMessage, void *prgchText, int cchTextMax, EChatEntryType *peChatEntryType, OUT_STRUCT() CSteamID *psteamidChatter ) = 0;
    virtual bool IsClanChatAdmin( CSteamID steamIDClanChat, CSteamID steamIDUser ) = 0;

    // interact with the Steam (game overlay / desktop)
    virtual bool IsClanChatWindowOpenInSteam( CSteamID steamIDClanChat ) = 0;
    virtual bool OpenClanChatWindowInSteam( CSteamID steamIDClanChat ) = 0;
    virtual bool CloseClanChatWindowInSteam( CSteamID steamIDClanChat ) = 0;

    // peer-to-peer chat interception
    // this is so you can show P2P chats inline in the game
    virtual bool SetListenForFriendsMessages( bool bInterceptEnabled ) = 0;
    virtual bool ReplyToFriendMessage( CSteamID steamIDFriend, const char *pchMsgToSend ) = 0;
    virtual int GetFriendMessage( CSteamID steamIDFriend, int iMessageID, void *pvData, int cubData, EChatEntryType *peChatEntryType ) = 0;

    // following apis
    CALL_RESULT( FriendsGetFollowerCount_t )
    virtual SteamAPICall_t GetFollowerCount( CSteamID steamID ) = 0;
    CALL_RESULT( FriendsIsFollowing_t )
    virtual SteamAPICall_t IsFollowing( CSteamID steamID ) = 0;
    CALL_RESULT( FriendsEnumerateFollowingList_t )
    virtual SteamAPICall_t EnumerateFollowingList( uint32 unStartIndex ) = 0;
*/
