use super::debug;
use core::ptr;

#[repr(C)]
pub struct Utils {
    vtable: &'static VTable,
}

impl Utils {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                get_seconds_since_app_active: SteamAPI_ISteamUtils_GetSecondsSinceAppActive,
                get_seconds_since_computer_active:
                    SteamAPI_ISteamUtils_GetSecondsSinceComputerActive,
            },
        }
    }
}

#[repr(C)]
struct VTable {
    get_seconds_since_app_active: extern "C" fn(this: *const Utils) -> u32,
    get_seconds_since_computer_active: extern "C" fn(this: *const Utils) -> u32,
    get_connected_universe: extern "C" fn(this: *const Utils) -> u32,
    get_server_real_time: extern "C" fn(this: *const Utils) -> u32,
    get_ip_country: extern "C" fn(this: *const Utils) -> *const u8,
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUtils_GetSecondsSinceAppActive(this: *const Utils) -> u32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUtils_GetSecondsSinceComputerActive(this: *const Utils) -> u32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUtils_GetConnectedUniverse(this: *const Utils) -> i32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUtils_GetServerRealTime(this: *const Utils) -> u32 {
    debug!();

    0
}

#[no_mangle]
pub extern "C" fn SteamAPI_ISteamUtils_GetIPCountry(this: *const Utils) -> *const u8 {
    debug!();

    let country = "US\0";

    country.as_ptr()
}
