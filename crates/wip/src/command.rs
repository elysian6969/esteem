use clap::{Parser, Subcommand};

#[derive(Parser)]
pub enum Command {
    /// Get information about apps
    #[clap(subcommand)]
    Apps(Apps),
    /// Helpful automation
    #[clap(subcommand)]
    Assistant(Assistant),
    /// Manage Steam athenicators
    #[clap(subcommand)]
    Authenticator(Authenticator),
    /// Remove data stored on disk
    #[clap(subcommand)]
    Clear(Clear),
    /// Manage Steam Cloud files (e.g. save files, settings, etc)
    #[clap(subcommand)]
    Cloud(Cloud),
    /// List and download from Steam depots
    #[clap(subcommand)]
    Depot(Depot),
    /// Query master server and server information
    #[clap(subcommand)]
    Hlmaster(Hlmaster),
    /// Parse SteamID representations
    SteamId,
    /// Info and download of user generated content
    #[clap(subcommand)]
    Ugc(Ugc),
    /// Access to WebAPI
    #[clap(subcommand)]
    WebApi(WebApi),
    /// Search and download workshop items
    #[clap(subcommand)]
    Workshop(Workshop),
}

#[derive(Subcommand)]
pub enum Apps {
    /// Activate key(s) on account
    ActivateKey,
    /// Manage licenses
    Licenses,
    /// List owned or all apps
    List,
    /// Show product info for app
    ProductInfo,
    /// Get item definitions for app
    ItemDef,
}

#[derive(Subcommand)]
pub enum Licenses {
    /// List all licenses on account
    List,
    /// Add free package license(s)
    Add,
    /// Remove free package license(s)
    Remove,
}

#[derive(Subcommand)]
pub enum Assistant {
    /// Idle up to 32 games for game time
    IdleGames,
    /// Automatic idling for game cards
    IdleCards,
    /// Explore a single discovery queue
    DiscoveryQueue,
}

#[derive(Subcommand)]
pub enum Authenticator {
    /// Add authentictor to a Steam account
    Add,
    /// Remove an authenticator
    Rwmove,
    /// List all authenticators
    List,
    /// Query Steam Guard status for account
    Status,
    /// Generate auth code
    Code,
    /// Generate QR code
    QrCode,
}

#[derive(Subcommand)]
pub enum Clear {
    /// Remove all cache and data files
    Cache,
    /// Remove all credentials and saved logins
    Credentials,
    /// Remove all cache files
    All,
}

#[derive(Subcommand)]
pub enum Cloud {
    /// List files for app
    List,
    /// List all apps with cloud files
    ListApps,
    /// Download files for app
    Download,
}

#[derive(Subcommand)]
pub enum Depot {
    /// View info about a depot(s)
    Info,
    /// List files from depot(s)
    List,
    /// Download depot files
    Download,
    /// Compare files between manifest(s) and filesystem
    Diff,
    /// Decrypt manifest gid
    DecryptGid,
}

#[derive(Subcommand)]
pub enum Hlmaster {
    /// Query HL Master for servers
    Query,
    /// Query info from a goldsrc or source server
    Info,
}

#[derive(Subcommand)]
pub enum Ugc {
    /// Get details for UGC
    Info,
    /// Download UGC
    Download,
}

#[derive(Subcommand)]
pub enum WebApi {
    /// Set WebAPI key
    SetKey,
    /// Remove saved key
    ClearKey,
    /// List all available WebAPI endpoints
    List,
    /// Call WebAPI endpoint
    Call,
}

#[derive(Subcommand)]
pub enum Workshop {
    /// Search the workshop
    Search,
    /// Get all details for a workshop item
    Info,
    /// Download a workshop item
    Download,
    /// Subscribe to workshop items
    Subscribe,
    /// Unsubscribe to workshop items
    Unsubscribe,
    /// Favourite workshop items
    Favourite,
    /// Unfavourite workshop items
    Unfavourite,
}
