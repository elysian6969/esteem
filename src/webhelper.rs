use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::{fmt, io};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlinkFeature {
    AudioWorklet,
    Badging,
    ResizeObserver,
    Worklet,
}

impl BlinkFeature {
    pub const fn as_str(&self) -> &'static str {
        match self {
            BlinkFeature::AudioWorklet => "AudioWorklet",
            BlinkFeature::Badging => "Badging",
            BlinkFeature::ResizeObserver => "ResizeObserver",
            BlinkFeature::Worklet => "Worklet",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Realm {
    Global,
}

impl Realm {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Realm::Global => "global",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Universe {
    Dev,
}

impl Universe {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Universe::Dev => "Dev",
        }
    }
}

pub struct WebHelper {
    build_id: Option<u32>,
    cache_dir: Option<PathBuf>,
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
    program: PathBuf,
    realm: Option<Realm>,
    quick_menu: bool,
    sandbox: bool,
    seccomp_filter: bool,
    smooth_scrolling: bool,
    steam_pid: Option<u32>,
    universe: Option<Universe>,
}

impl WebHelper {
    #[inline]
    pub fn new<S>(program: S) -> Self
    where
        S: AsRef<Path>,
    {
        let program = program.as_ref().to_path_buf();

        Self {
            build_id: None,
            cache_dir: None,
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
            program: program,
            realm: None,
            quick_menu: true,
            sandbox: true,
            seccomp_filter: true,
            smooth_scrolling: true,
            steam_pid: None,
            universe: None,
        }
    }

    #[inline]
    pub fn build_id(&mut self, id: u32) -> &mut Self {
        self.build_id = Some(id);
        self
    }

    #[inline]
    pub fn cache_dir<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.cache_dir = to_path_buf(path);
        self
    }

    #[inline]
    pub fn current_dir<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.current_dir = to_path_buf(path);
        self
    }

    #[inline]
    pub fn disable_features<I>(&mut self, features: I) -> &mut Self
    where
        I: IntoIterator<Item = BlinkFeature>,
    {
        self.disabled_features = features.into_iter().collect();
        self
    }

    #[inline]
    pub fn enable_features<I>(&mut self, features: I) -> &mut Self
    where
        I: IntoIterator<Item = BlinkFeature>,
    {
        self.enabled_features = features.into_iter().collect();
        self
    }

    #[inline]
    pub fn gpu_watchdog(&mut self, enable: bool) -> &mut Self {
        self.gpu_watchdog = enable;
        self
    }

    #[inline]
    pub fn hang_monitor(&mut self, enable: bool) -> &mut Self {
        self.hang_monitor = enable;
        self
    }

    #[inline]
    pub fn index<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.index = to_path_buf(path);
        self
    }

    #[inline]
    pub fn lang<S>(&mut self, lang: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.lang = to_string(lang);
        self
    }

    #[inline]
    pub fn library_path<S>(&mut self, path: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        self.library_path = to_os_string(path);
        self
    }

    #[inline]
    pub fn log_dir<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.log_dir = to_path_buf(path);
        self
    }

    #[inline]
    pub fn log_file<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.log_file = to_path_buf(path);
        self
    }

    #[inline]
    pub fn media_stream(&mut self, enable: bool) -> &mut Self {
        self.media_stream = enable;
        self
    }

    #[inline]
    pub fn realm(&mut self, realm: Realm) -> &mut Self {
        self.realm = Some(realm);
        self
    }

    #[inline]
    pub fn quick_menu(&mut self, enable: bool) -> &mut Self {
        self.quick_menu = enable;
        self
    }

    #[inline]
    pub fn sandbox(&mut self, enable: bool) -> &mut Self {
        self.sandbox = enable;
        self
    }

    #[inline]
    pub fn seccomp_filter(&mut self, enable: bool) -> &mut Self {
        self.seccomp_filter = enable;
        self
    }

    #[inline]
    pub fn smooth_scrolling(&mut self, enable: bool) -> &mut Self {
        self.smooth_scrolling = enable;
        self
    }

    #[inline]
    pub fn steam_pid(&mut self, pid: u32) -> &mut Self {
        self.steam_pid = Some(pid);
        self
    }

    #[inline]
    pub fn universe(&mut self, universe: Universe) -> &mut Self {
        self.universe = Some(universe);
        self
    }

    #[inline]
    fn command(&self) -> Command {
        let mut command = Command::new(&self.program);

        if let Some(build_id) = self.build_id {
            let mut string = OsString::from("-buildid=");

            string.push(build_id.to_string());
            command.arg(string);
        }

        if let Some(cache_dir) = &self.cache_dir {
            let mut string = OsString::from("-cachedir=");

            string.push(cache_dir);
            command.arg(string);
        }

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

            command.arg(format!("--disable-blink-features={features}"));
        }

        if !self.enabled_features.is_empty() {
            let features = self
                .enabled_features
                .iter()
                .map(|feature| feature.as_str())
                .collect::<Vec<_>>();

            let features = features.join(",");

            command.arg(format!("--enable-blink-features={features}"));
        }

        if !self.gpu_watchdog {
            command.arg("--disable-gpu-watchdog");
        }

        if !self.hang_monitor {
            command.arg("--disable-hang-monitor");
        }

        if let Some(index) = &self.index {
            command.arg(index);
        }

        if let Some(lang) = &self.lang {
            command.arg(format!("-lang={lang}"));
        }

        if let Some(library_path) = &self.library_path {
            command.env("LD_LIBRARY_PATH", library_path);
        }

        if let Some(log_dir) = &self.log_dir {
            let mut string = OsString::from("-logdir=");

            string.push(log_dir);
            command.arg(string);
        }

        if let Some(log_file) = &self.log_file {
            let mut string = OsString::from("--log-file=");

            string.push(log_file);
            command.arg(string);
        }

        if self.media_stream {
            command.arg("--enable-media-stream");
        }

        if let Some(realm) = &self.realm {
            let mut string = OsString::from("-realm=");

            string.push(realm.as_str());
            command.arg(string);
        }

        if !self.quick_menu {
            command.arg("--disable-quick-menu");
        }

        if !self.sandbox {
            command.arg("--no-sandbox");
        }

        if !self.seccomp_filter {
            command.arg("--disable-seccomp-filter-sandbox");
        }

        if !self.smooth_scrolling {
            command.arg("--disable-smooth-scrolling");
        }

        if let Some(pid) = &self.steam_pid {
            let mut string = OsString::from("-steampid=");

            string.push(pid.to_string());
            command.arg(string);
        }

        if let Some(universe) = &self.universe {
            let mut string = OsString::from("-steamuniverse=");

            string.push(universe.as_str());
            command.arg(string);
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
