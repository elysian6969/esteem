#![allow(dead_code)]
#![allow(non_camel_case_types)]

use clap::Parser;
use command::Command;

pub use auth_session_response::AuthSessionResponse;
pub use begin_auth_session_result::BeginAuthSessionResult;
pub use broadcast_upload_result::BroadcastUploadResult;
pub use chat_entry_kind::ChatEntryKind;
pub use chat_room_enter_response::ChatRoomEnterResponse;
pub use deny_reason::DenyReason;
pub use duration_control_notification::DurationControlNotification;
pub use duration_control_online_state::DurationControlOnlineState;
pub use duration_control_progress::DurationControlProgress;
pub use game_search_error_code::GameSearchErrorCode;
pub use instance::Instance;
pub use license_result::LicenseResult;
pub use market_not_allowed_reason_flags::MarketNotAllowedReasonFlags;
pub use message::Message;
pub use notification_position::NotificationPosition;
pub use player_result::PlayerResult;
pub use vanity_url_kind::VanityUrlKind;

mod auth_session_response;
mod begin_auth_session_result;
mod broadcast_upload_result;
mod chat_entry_kind;
mod chat_room_enter_response;
mod command;
mod deny_reason;
mod duration_control_notification;
mod duration_control_online_state;
mod duration_control_progress;
mod game_search_error_code;
mod instance;
mod kind;
mod license_result;
mod market_not_allowed_reason_flags;
mod message;
mod notification_position;
mod player_result;
mod vanity_url_kind;

fn main() {
    let _command = Command::parse();
}
