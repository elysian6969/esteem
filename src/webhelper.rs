use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::{fmt, io};

pub use blink_feature::BlinkFeature;
pub use command_decoder::CommandDecoder;
pub use gl::Gl;
pub use password_store::PasswordStore;
pub use realm::Realm;
pub use universe::Universe;

mod blink_feature;
mod command_decoder;
mod gl;
mod password_store;
mod realm;
mod universe;

#[inline]
fn to_os_string<S>(string: S) -> Option<OsString>
where
    S: AsRef<OsStr>,
{
    Some(string.as_ref().to_os_string())
}

#[inline]
fn to_path_buf<P>(path: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    Some(path.as_ref().to_path_buf())
}

#[inline]
fn to_string<S>(string: S) -> Option<String>
where
    S: AsRef<str>,
{
    Some(string.as_ref().to_string())
}

#[inline]
fn join<L, R>(left: L, right: R) -> OsString
where
    L: AsRef<OsStr>,
    R: AsRef<OsStr>,
{
    let mut string = OsString::from(left.as_ref());

    string.push(right);
    string
}

/// steamwebhelper
pub struct WebHelper {
    build_id: Option<u32>,
    cache_dir: Option<PathBuf>,
    command_decoder: Option<CommandDecoder>,
    composer_mode: bool,
    client_ui: Option<PathBuf>,
    current_dir: Option<PathBuf>,
    disabled_features: Vec<BlinkFeature>,
    enabled_features: Vec<BlinkFeature>,
    gpu_watchdog: bool,
    hang_monitor: bool,
    index: Option<PathBuf>,
    lang: Option<String>,
    library_path: Option<OsString>,
    log_dir: Option<PathBuf>,
    log_file: Option<PathBuf>,
    media_stream: bool,
    password_store: Option<PasswordStore>,
    program: PathBuf,
    realm: Option<Realm>,
    quick_menu: bool,
    sandbox: bool,
    seccomp_filter: bool,
    smooth_scrolling: bool,
    steam_id: Option<u32>,
    steam_pid: Option<u32>,
    universe: Option<Universe>,
    use_angle: Option<Gl>,
}

impl WebHelper {
    /// construct a builder to call the steamwebhelper located at `program`
    #[inline]
    pub fn new<S>(program: S) -> Self
    where
        S: AsRef<Path>,
    {
        let program = program.as_ref().to_path_buf();

        Self {
            build_id: None,
            cache_dir: None,
            client_ui: None,
            command_decoder: None,
            composer_mode: false,
            current_dir: None,
            disabled_features: Vec::new(),
            enabled_features: Vec::new(),
            gpu_watchdog: true,
            hang_monitor: true,
            index: None,
            lang: None,
            library_path: None,
            log_dir: None,
            log_file: None,
            media_stream: true,
            password_store: None,
            program: program,
            realm: None,
            quick_menu: true,
            sandbox: true,
            seccomp_filter: true,
            smooth_scrolling: true,
            steam_id: None,
            steam_pid: None,
            universe: None,
            use_angle: None,
        }
    }

    /// the build id
    #[inline]
    pub fn build_id(&mut self, id: u32) -> &mut Self {
        self.build_id = Some(id);
        self
    }

    /// html cache dir
    #[inline]
    pub fn cache_dir<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.cache_dir = to_path_buf(path);
        self
    }

    /// client ui path
    #[inline]
    pub fn client_ui<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.client_ui = to_path_buf(path);
        self
    }

    /// specify the command decoder
    #[inline]
    pub fn command_decoder(&mut self, decoder: CommandDecoder) -> &mut Self {
        self.command_decoder = Some(decoder);
        self
    }

    /// composer mode (todo: wtf is this)
    #[inline]
    pub fn composer_mode(&mut self, enable: bool) -> &mut Self {
        self.composer_mode = enable;
        self
    }

    /// the current directory to change to (steamwebhelper may attempt to use relative paths)
    #[inline]
    pub fn current_dir<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.current_dir = to_path_buf(path);
        self
    }

    /// disable blink features
    #[inline]
    pub fn disable_features<I>(&mut self, features: I) -> &mut Self
    where
        I: IntoIterator<Item = BlinkFeature>,
    {
        self.disabled_features = features.into_iter().collect();
        self
    }

    /// enable blink features
    #[inline]
    pub fn enable_features<I>(&mut self, features: I) -> &mut Self
    where
        I: IntoIterator<Item = BlinkFeature>,
    {
        self.enabled_features = features.into_iter().collect();
        self
    }

    /// control whether the gpu watchdog should be enabled or not
    #[inline]
    pub fn gpu_watchdog(&mut self, enable: bool) -> &mut Self {
        self.gpu_watchdog = enable;
        self
    }

    /// control whether the hang monitor should be enabled or not
    #[inline]
    pub fn hang_monitor(&mut self, enable: bool) -> &mut Self {
        self.hang_monitor = enable;
        self
    }

    /// specify `index.html`
    #[inline]
    pub fn index<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.index = to_path_buf(path);
        self
    }

    /// specify the locale
    #[inline]
    pub fn lang<S>(&mut self, lang: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.lang = to_string(lang);
        self
    }

    /// set `LD_LIBRARY_PATH` (to find `libcef.so`, for example)
    #[inline]
    pub fn library_path<S>(&mut self, path: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        self.library_path = to_os_string(path);
        self
    }

    /// specify where to place cef logs
    #[inline]
    pub fn log_dir<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.log_dir = to_path_buf(path);
        self
    }

    /// specify where to place main cef log
    #[inline]
    pub fn log_file<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.log_file = to_path_buf(path);
        self
    }

    /// control whether media stream is enabled or not
    #[inline]
    pub fn media_stream(&mut self, enable: bool) -> &mut Self {
        self.media_stream = enable;
        self
    }

    /// specify the steam realm
    #[inline]
    pub fn realm(&mut self, realm: Realm) -> &mut Self {
        self.realm = Some(realm);
        self
    }

    /// specify the password store
    #[inline]
    pub fn password_store(&mut self, store: PasswordStore) -> &mut Self {
        self.password_store = Some(store);
        self
    }

    /// control whethee quick menu is enabled or not
    #[inline]
    pub fn quick_menu(&mut self, enable: bool) -> &mut Self {
        self.quick_menu = enable;
        self
    }

    /// control whether the cef sandbox is enabled or not
    #[inline]
    pub fn sandbox(&mut self, enable: bool) -> &mut Self {
        self.sandbox = enable;
        self
    }

    /// control whether the cef sandbox uses a seccomp filter
    #[inline]
    pub fn seccomp_filter(&mut self, enable: bool) -> &mut Self {
        self.seccomp_filter = enable;
        self
    }

    /// control whether smooth scrolling is enabled or not
    #[inline]
    pub fn smooth_scrolling(&mut self, enable: bool) -> &mut Self {
        self.smooth_scrolling = enable;
        self
    }

    /// steam id
    #[inline]
    pub fn steam_id(&mut self, id: u32) -> &mut Self {
        self.steam_id = Some(id);
        self
    }

    /// provide the steam pid (to do ipc with) (most of the steam client doesnt work without this)
    #[inline]
    pub fn steam_pid(&mut self, pid: u32) -> &mut Self {
        self.steam_pid = Some(pid);
        self
    }

    /// provide the steam universe to use
    #[inline]
    pub fn universe(&mut self, universe: Universe) -> &mut Self {
        self.universe = Some(universe);
        self
    }

    /// provide the gl variant to use
    #[inline]
    pub fn use_angle(&mut self, gl: Gl) -> &mut Self {
        self.use_angle = Some(gl);
        self
    }

    #[inline]
    pub fn command(&self) -> Command {
        const BUILD_ID: &str = "-buildid=";

        const CACHE_DIR: &str = "-cachedir=";
        const CLIENT_UI: &str = "-clientui=";
        const COMPOSER_MODE: &str = "-composer-mode=";

        const DISABLE_BLINK_FEATURES: &str = "--disable-blink-features=";
        const DISABLE_GPU_MONITOR: &str = "--disable-gpu-monitor";
        const DISABLE_GPU_WATCHDOG: &str = "--disable-gpu-watchdog";
        const DISABLE_QUICK_MENU: &str = "--disable-quick-menu";
        const DISABLE_SECCOMP_FILTER_SANDBOX: &str = "--disable-seccomp-filter-sandbox";
        const DISABLE_SMOOTH_SCROLLING: &str = "--disable-smooth-scrolling";

        const ENABLE_BLINK_FEATURES: &str = "--enable-blink-features=";
        const ENABLE_MEDIA_STREAM: &str = "--enable-media-stream";
        const ENABLE_SMOOTH_SCROLLING: &str = "--enable-smooth-scrolling";

        const LANG: &str = "-lang=";
        const LOG_DIR: &str = "-logdir=";
        const LOG_FILE: &str = "--log-file=";

        const NO_SANDBOX: &str = "--no-sandbox";

        const PASSWORD_STORE: &str = "--password-store=";

        const REALM: &str = "-realm=";

        const STEAM_ID: &str = "-steamid=";
        const STEAM_PID: &str = "-steampid=";
        const STEAM_UNIVERSE: &str = "-steamuniverse=";

        const USE_ANGLE: &str = "--use-angle=";
        const USE_CMD_DECODER: &str = "--use-cmd-decoder=";

        let mut command = Command::new(&self.program);

        if let Some(build_id) = self.build_id {
            command.arg(join(BUILD_ID, build_id.to_string()));
        }

        if let Some(cache_dir) = &self.cache_dir {
            command.arg(join(CACHE_DIR, cache_dir));
        }

        if let Some(decoder) = &self.command_decoder {
            command.arg(join(USE_CMD_DECODER, decoder.as_str()));
        }

        if let Some(client_ui) = &self.client_ui {
            command.arg(join(CLIENT_UI, client_ui));
        }

        command.arg(join(COMPOSER_MODE, (self.composer_mode as u8).to_string()));

        if let Some(current_dir) = &self.current_dir {
            command.current_dir(current_dir);
        }

        if !self.disabled_features.is_empty() {
            let features = self
                .disabled_features
                .iter()
                .map(|feature| feature.as_str())
                .collect::<Vec<_>>();

            let features = features.join(",");

            command.arg(format!("{DISABLE_BLINK_FEATURES}{features}"));
        }

        if !self.enabled_features.is_empty() {
            let features = self
                .enabled_features
                .iter()
                .map(|feature| feature.as_str())
                .collect::<Vec<_>>();

            let features = features.join(",");

            command.arg(format!("{ENABLE_BLINK_FEATURES}{features}"));
        }

        if !self.gpu_watchdog {
            command.arg(DISABLE_GPU_WATCHDOG);
        }

        if !self.hang_monitor {
            command.arg(DISABLE_GPU_MONITOR);
        }

        if let Some(index) = &self.index {
            command.arg(index);
        }

        if let Some(lang) = &self.lang {
            command.arg(join(LANG, lang));
        }

        if let Some(library_path) = &self.library_path {
            command.env("LD_LIBRARY_PATH", library_path);
        }

        if let Some(log_dir) = &self.log_dir {
            command.arg(join(LOG_DIR, log_dir));
        }

        if let Some(log_file) = &self.log_file {
            command.arg(join(LOG_FILE, log_file));
        }

        if self.media_stream {
            command.arg(ENABLE_MEDIA_STREAM);
        }

        if let Some(store) = &self.password_store {
            command.arg(join(PASSWORD_STORE, store.as_str()));
        }

        if let Some(realm) = &self.realm {
            command.arg(join(REALM, realm.as_str()));
        }

        if !self.quick_menu {
            command.arg(DISABLE_QUICK_MENU);
        }

        if !self.sandbox {
            command.arg(NO_SANDBOX);
        }

        if !self.seccomp_filter {
            command.arg(DISABLE_SECCOMP_FILTER_SANDBOX);
        }

        if self.smooth_scrolling {
            command.arg(ENABLE_SMOOTH_SCROLLING);
        } else {
            command.arg(DISABLE_SMOOTH_SCROLLING);
        }

        if let Some(id) = &self.steam_id {
            command.arg(join(STEAM_ID, id.to_string()));
        }

        if let Some(pid) = &self.steam_pid {
            command.arg(join(STEAM_PID, pid.to_string()));
        }

        if let Some(universe) = &self.universe {
            command.arg(join(STEAM_UNIVERSE, universe.as_str()));
        }

        if let Some(gl) = &self.use_angle {
            command.arg(join(USE_ANGLE, gl.as_str()));
        }

        command
    }

    #[inline]
    pub fn spawn(&mut self) -> io::Result<Child> {
        let mut command = self.command();

        command.spawn()
    }
}

impl fmt::Debug for WebHelper {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.command(), fmt)
    }
}
