use super::{api_fn, debug, virtual_struct};
use std::ffi::CStr;

pub type BrowserHandle = u32;

virtual_struct! { HTMLSurface {
    fn drop(&self) -> (),
    fn init(&self) -> bool,
    fn shutdown(&self) -> bool,
    fn create_browser(&self, user_agent: *const u8, user_css: *const u8) -> BrowserHandle,
    fn remove_browser(&self, handle: BrowserHandle) -> (),
    fn load_url(&self, handle: BrowserHandle, url: *const u8, post_data: *const u8) -> (),
} }

impl HTMLSurface {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                drop,
                init: SteamAPI_ISteamHTMLSurface_Init,
                shutdown: SteamAPI_ISteamHTMLSurface_Shutdown,
                create_browser: SteamAPI_ISteamHTMLSurface_CreateBrowser,
                remove_browser: SteamAPI_ISteamHTMLSurface_RemoveBrowser,
                load_url: SteamAPI_ISteamHTMLSurface_LoadURL,
            },
        }
    }
}

extern "C" fn drop(this: *const HTMLSurface) {
    debug!();
}

api_fn! { Init(&HTMLSurface) -> bool {
    debug!();

    true
} }

api_fn! { Shutdown(&HTMLSurface) -> bool {
    debug!();

    true
} }

api_fn! { CreateBrowser(
    &HTMLSurface,
    user_agent: *const u8,
    user_css: *const u8
) -> BrowserHandle {
    debug!();

    println!("user_agent = {:?}", unsafe {
        CStr::from_ptr(user_agent.cast())
    });

    println!("user_css = {:?}", unsafe {
        CStr::from_ptr(user_css.cast())
    });

    69
} }

api_fn! { RemoveBrowser(&HTMLSurface, handle: BrowserHandle) -> () {
    debug!();
} }

api_fn! { LoadURL(
    &HTMLSurface,
    handle: BrowserHandle,
    url: *const u8,
    post_data: *const u8
) -> () {
    debug!();

    println!("url = {:?}", unsafe {
        CStr::from_ptr(url.cast())
    });

    println!("post_data = {:?}", unsafe {
        CStr::from_ptr(post_data.cast())
    });
} }
