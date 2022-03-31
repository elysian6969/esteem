use super::{api_fn, debug, virtual_struct};
use core::ptr;

virtual_struct! { Networking {
    fn send_p2p_packet(&self, remote_id: u64, pub_data: *const (), cub_data: u32, p2p_send_kind: u8, channel: i32) -> bool,
    fn is_p2p_packet_available(&self, msg_size: *mut u32, channel: i32) -> bool,
    fn read_p2p_packet(&self, dst: *const (), cub_dst: u32, cub_msg_size: *mut u32, remote_id: u64, channel: i32) -> bool,
    fn accept_p2p_session_with_user(&self, remote_id: u64) -> bool,
    fn close_p2p_session_with_user(&self, remote_id: u64) -> bool,
    fn close_p2p_channel_with_user(&self, remote_id: u64) -> bool,
    fn get_p2p_session_state(&self, remote_id: u64, connection_state: *mut ()) -> bool,
    fn allow_p2p_packet_relay(&self, allow: bool) -> bool,
} }

impl Networking {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                send_p2p_packet: SteamAPI_ISteamNetworking_SendP2PPacket,
                is_p2p_packet_available: SteamAPI_ISteamNetworking_IsP2PPacketAvailable,
                read_p2p_packet: SteamAPI_ISteamNetworking_ReadP2PPacket,
                accept_p2p_session_with_user: SteamAPI_ISteamNetworking_AcceptP2PSessionWithUser,
                close_p2p_session_with_user: SteamAPI_ISteamNetworking_CloseP2PSessionWithUser,
                close_p2p_channel_with_user: SteamAPI_ISteamNetworking_CloseP2PChannelWithUser,
                get_p2p_session_state: SteamAPI_ISteamNetworking_GetP2PSessionState,
                allow_p2p_packet_relay: SteamAPI_ISteamNetworking_AllowP2PPacketRelay,
            },
        }
    }
}

api_fn! { SendP2PPacket(
    &Networking,
    remote_id: u64,
    pub_data: *const (),
    cub_data: u32,
    p2p_send_kind: u8,
    channel: i32
) -> bool {
    debug!();

    false
} }

api_fn! { IsP2PPacketAvailable(
    &Networking,
    msg_size: *mut u32,
    channel: i32
) -> bool {
    debug!();

    false
} }

api_fn! { ReadP2PPacket(
    &Networking,
    dst: *const (),
    cub_dst: u32,
    cub_msg_size: *mut u32,
    remote_id: u64,
    channel: i32
) -> bool {
    debug!();

    false
} }

api_fn! { AcceptP2PSessionWithUser(&Networking, remote_id: u64) -> bool {
    debug!();

    false
} }

api_fn! { CloseP2PSessionWithUser(&Networking, remote_id: u64) -> bool {
    debug!();

    false
} }

api_fn! { CloseP2PChannelWithUser(&Networking, remote_id: u64) -> bool {
    debug!();

    false
} }

api_fn! { GetP2PSessionState(&Networking, remote_id: u64, connection_state: *mut ()) -> bool {
    debug!();

    false
} }

api_fn! { AllowP2PPacketRelay(&Networking, allow: bool) -> bool {
    debug!();

    println!("allow = {allow:?}");

    false
} }
