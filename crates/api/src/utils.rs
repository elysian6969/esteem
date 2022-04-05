use super::{api_fn, debug, virtual_struct, AppId};
use std::net::Ipv4Addr;

virtual_struct! { Utils {
    fn get_seconds_since_app_active(&self) -> u32,
    fn get_seconds_since_computer_active(&self) -> u32,
    fn get_connected_universe(&self) -> steam_id::Universe,
    fn get_server_real_time(&self) -> u32,
    fn get_ip_country(&self) -> *const u8,
    fn get_image_size(&self, image: i32, width: *mut u32, height: *mut u32) -> bool,
    fn get_image_rgba(&self, image: i32, buf: *mut u8, len: i32) -> bool,
    fn get_server_ip_port(&self, ip: *mut u32, port: *mut u16) -> bool,
    fn get_current_battery_power(&self) -> u8,
    fn get_app_id(&self) -> AppId,
} }

impl Utils {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                get_seconds_since_app_active: SteamAPI_ISteamUtils_GetSecondsSinceAppActive,
                get_seconds_since_computer_active:
                    SteamAPI_ISteamUtils_GetSecondsSinceComputerActive,
                get_connected_universe: SteamAPI_ISteamUtils_GetConnectedUniverse,
                get_server_real_time: SteamAPI_ISteamUtils_GetServerRealTime,
                get_ip_country: SteamAPI_ISteamUtils_GetIPCountry,
                get_image_size: SteamAPI_ISteamUtils_GetImageSize,
                get_image_rgba: SteamAPI_ISteamUtils_GetImageRGBA,
                get_server_ip_port: SteamAPI_ISteamUtils_GetCSERIPPort,
                get_current_battery_power: SteamAPI_ISteamUtils_GetCurrentBatteryPower,
                get_app_id: SteamAPI_ISteamUtils_GetAppId,
            },
        }
    }
}

api_fn! { GetSecondsSinceAppActive(&Utils) -> u32 {
    debug!();

    0
} }

api_fn! { GetSecondsSinceComputerActive(&Utils) -> u32 {
    debug!();

    0
} }

api_fn! { GetConnectedUniverse(&Utils) -> steam_id::Universe {
    debug!();

    steam_id::Universe::Public
} }

api_fn! { GetServerRealTime(&Utils) -> u32 {
    debug!();

    0
} }

api_fn! { GetIPCountry(&Utils) -> *const u8 {
    debug!();

    let country = "US\0";

    country.as_ptr()
} }

api_fn! { GetImageSize(&Utils, image: i32, width: *mut u32, height: *mut u32) -> bool {
    debug!();

    false
} }

api_fn! { GetImageRGBA(&Utils, image: i32, buf: *mut u8, len: i32) -> bool {
    debug!();

    false
} }

api_fn! { GetCSERIPPort(&Utils, ip: *mut u32, port: *mut u16) -> bool {
    debug!();

    unsafe {
        ip.write(u32::from_be_bytes(Ipv4Addr::new(192, 168, 20, 69).octets()));
        port.write(27015);
    }

    true
} }

api_fn! { GetCurrentBatteryPower(&Utils) -> u8 {
    debug!();

    10
} }

api_fn! { GetAppId(&Utils) -> AppId {
    debug!();

    AppId(730)
} }
