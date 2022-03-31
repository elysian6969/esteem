use super::{api_fn, debug, virtual_struct};
use core::ptr;

pub type ScreenshotHandle = u32;

virtual_struct! { Screenshots {
    fn write_screenshot(&self, pub_rgb: *const (), cub_rgb: u32, width: i32, height: i32) -> ScreenshotHandle,
    fn add_screenshot_to_library(&self, filename: *const u8, thumbnail_filename: *const u8, width: i32, height: i32) -> ScreenshotHandle,
    fn trigger_screenshot(&self) -> (),
    fn hook_screenshot(&self, hook: bool) -> (),
} }

impl Screenshots {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                write_screenshot: SteamAPI_ISteamScreenshots_WriteScreenshot,
                add_screenshot_to_library: SteamAPI_ISteamScreenshots_AddScreenshotToLibrary,
                trigger_screenshot: SteamAPI_ISteamScreenshots_TriggerScreenshot,
                hook_screenshot: SteamAPI_ISteamScreenshots_HookScreenshot,
            },
        }
    }
}

api_fn! { WriteScreenshot(
    &Screenshots,
    pub_rgb: *const (),
    cub_rgb: u32,
    width: i32,
    height: i32
) -> ScreenshotHandle {
    debug!();

    69
} }

api_fn! { AddScreenshotToLibrary(
    &Screenshots,
    filename: *const u8,
    thumbnail_filename: *const u8,
    width: i32,
    height: i32
) -> ScreenshotHandle {
    debug!();

    69
} }

api_fn! { TriggerScreenshot(&Screenshots) -> () {
    debug!();
} }

api_fn! { HookScreenshot(&Screenshots, hook: bool) -> () {
    debug!();
} }
