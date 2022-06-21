#[allow(dead_code)]
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
pub unsafe extern "C" fn GetClientLauncherType() -> LauncherKind {
    let kind = LauncherKind::Default;

    #[cfg(debug_assertions)]
    println!("esteem | steamui requested client launcher type (\x1b[38;5;3m{kind:?}\x1b[m)");

    kind
}
