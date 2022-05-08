const INSTALL_PATH: &str = "/usr/lib/esteem\0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum LauncherKind {
    // default
    Default = 0,
    // pw_dota2
    Dota = 2,
    // nexon_dota2
    NexonDota = 3,
    // steamcmd
    SteamCmd = 4,
    // pw_csgo
    CSGO = 5,
    // clientui
    ClientUI = 6,
    // steamhdl
    SteamHDL = 7,
    // steamchina
    SteamChina = 8,
    // singleapp
    SingleApp = 9,
}

#[allow(non_snake_case)]
#[no_mangle]
#[used(linker)]
pub static GetClientLauncherType: unsafe extern "C" fn() -> LauncherKind = {
    #[inline(always)]
    pub unsafe extern "C" fn GetClientLauncherType() -> LauncherKind {
        LauncherKind::Default
    }

    GetClientLauncherType
};

#[allow(non_snake_case)]
#[no_mangle]
#[used(linker)]
pub static SteamBootstrapper_GetInstallDir: unsafe extern "C" fn() -> *const i8 = {
    #[inline(always)]
    pub unsafe extern "C" fn SteamBootstrapper_GetInstallDir() -> *const i8 {
        INSTALL_PATH.as_ptr().cast()
    }

    SteamBootstrapper_GetInstallDir
};

#[allow(non_snake_case)]
#[no_mangle]
#[used(linker)]
pub static SteamBootstrapper_GetBaseUserDir: unsafe extern "C" fn() -> *const i8 = {
    #[inline(always)]
    pub unsafe extern "C" fn SteamBootstrapper_GetBaseUserDir() -> *const i8 {
        INSTALL_PATH.as_ptr().cast()
    }

    SteamBootstrapper_GetBaseUserDir
};

#[allow(non_snake_case)]
#[no_mangle]
#[used(linker)]
pub static SteamBootstrapper_GetLoggingDir: unsafe extern "C" fn() -> *const i8 = {
    #[inline(always)]
    pub unsafe extern "C" fn SteamBootstrapper_GetLoggingDir() -> *const i8 {
        INSTALL_PATH.as_ptr().cast()
    }

    SteamBootstrapper_GetLoggingDir
};
