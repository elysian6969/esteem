use super::{api_fn, debug, virtual_struct};

virtual_struct! { Apps {
    fn is_subscribed(&self) -> bool,
    fn is_low_violence(&self) -> bool,
    fn is_cybercafe(&self) -> bool,
    fn is_vac_banned(&self) -> bool,
    fn get_current_game_language(&self) -> *const u8,
} }

impl Apps {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                is_subscribed: SteamAPI_ISteamApps_BIsSubscribed,
                is_low_violence: SteamAPI_ISteamApps_BIsLowViolence,
                is_cybercafe: SteamAPI_ISteamApps_BIsCybercafe,
                is_vac_banned: SteamAPI_ISteamApps_BIsVACBanned,
                get_current_game_language: SteamAPI_ISteamApps_GetCurrentGameLanguage,
            },
        }
    }
}

api_fn! { BIsSubscribed(&Apps) -> bool {
    debug!();

    false
} }

api_fn! { BIsLowViolence(&Apps) -> bool {
    debug!();

    false
} }

api_fn! { BIsCybercafe(&Apps) -> bool {
    debug!();

    false
} }

api_fn! { BIsVACBanned(&Apps) -> bool {
    debug!();

    false
} }

api_fn! { GetCurrentGameLanguage(&Apps) -> *const u8 {
    debug!();

    let lang = "en_US\0";

    lang.as_ptr()
} }
