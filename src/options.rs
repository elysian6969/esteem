use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Options {
    /// Enable devtools in CEF.
    #[clap(long)]
    pub devtools: bool,

    #[clap(long)]
    pub dev: bool,

    /// Enable some more steam logs.
    #[clap(long)]
    pub more_log: bool,

    /// Disable CEF entirely.
    #[clap(long)]
    pub no_browser: bool,

    /// Disable CEF sandbox.
    #[clap(long)]
    pub no_sandbox: bool,

    /// Disable SteamVR initialization.
    #[clap(long)]
    pub no_vr: bool,

    /// Set the `STEAM_ZENITY` environment variable.
    #[clap(default_value = "/usr/bin/zenity", long)]
    pub zenity: PathBuf,

    #[clap(long)]
    pub game: Option<u16>,
}

impl Options {
    pub fn parse() -> Self {
        <Options as Parser>::parse()
    }
}
