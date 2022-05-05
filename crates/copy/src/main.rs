use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

mod env;
mod fs;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

struct Extract {
    lib_dir: PathBuf,
    i686_dir: PathBuf,
    x86_64_dir: PathBuf,

    ubuntu32_dir: PathBuf,
    //ubuntu64_dir: PathBuf,
    steam_runtime_i686_dir: PathBuf,
    steam_runtime_usr_i686_dir: PathBuf,
}

impl Extract {
    pub fn new() -> Self {
        let root_dir = Path::new(ROOT).join("../..");
        let lib_dir = root_dir.join("lib");
        let i686_dir = lib_dir.join("i686");
        let x86_64_dir = lib_dir.join("x86_64");

        let home_dir = env::home_dir();
        let data_dir = env::data_dir(home_dir);

        let ubuntu32_dir = data_dir.join("ubuntu12_32");
        //let ubuntu64_dir = data_dir.join("ubuntu12_64");

        let steam_runtime_dir = ubuntu32_dir.join("steam-runtime");
        let steam_runtime_i686_dir = steam_runtime_dir.join("lib/i386-linux-gnu");
        let steam_runtime_usr_i686_dir = steam_runtime_dir.join("usr/lib/i386-linux-gnu");

        Self {
            lib_dir,
            i686_dir,
            x86_64_dir,

            ubuntu32_dir,
            //ubuntu64_dir,
            steam_runtime_i686_dir,
            steam_runtime_usr_i686_dir,
        }
    }

    fn prepare(&self) -> io::Result<()> {
        let _ = fs::create_dir_all(&self.i686_dir);
        let _ = fs::create_dir_all(&self.x86_64_dir);

        //fs::symlink(self.lib_dir.join("ubuntu12_32"), "i686")?;
        //fs::symlink(self.lib_dir.join("ubuntu12_64"), "i686")?;

        Ok(())
    }

    fn copy<N>(&self, name: N) -> io::Result<()>
    where
        N: AsRef<OsStr>,
    {
        let name = name.as_ref();

        println!("   - \x1b[38;5;2m{name:?}\x1b[m");

        fs::copy_from(&self.ubuntu32_dir, &self.i686_dir, name)
            .or_else(|_| fs::copy_from(&self.steam_runtime_i686_dir, &self.i686_dir, name))
            .or_else(|_| fs::copy_from(&self.steam_runtime_usr_i686_dir, &self.i686_dir, name))
    }

    fn symlink<F, T>(&self, from: F, to: T) -> io::Result<()>
    where
        F: AsRef<OsStr>,
        T: AsRef<OsStr>,
    {
        let from = from.as_ref();
        let to = to.as_ref();

        println!("   - \x1b[38;5;2m{from:?}\x1b[m -> \x1b[38;5;2m{to:?}\x1b[m");

        fs::symlink(self.i686_dir.join(from), to)
    }
}

fn main() -> io::Result<()> {
    let extract = Extract::new();

    extract.prepare()?;

    println!(" > copy ubuntu12_32 libraries");

    // cef
    extract.copy("chromehtml.so")?;
    extract.copy("crashhandler.so")?;

    // valve
    extract.copy("filesystem_stdio.so")?;
    extract.copy("libtier0_s.so")?;
    extract.copy("libvideo.so")?;
    extract.copy("libvstdlib_s.so")?;
    extract.copy("serverbrowser.so")?;
    extract.copy("vgui2_s.so")?;

    // steam
    extract.copy("friendsui.so")?;
    extract.copy("steamclient.so")?;
    extract.copy("steamservice.so")?;
    extract.copy("steamui.so")?;

    // icu
    extract.copy("libicui18n.so")?;
    extract.copy("libicuuc.so")?;

    // ffmpeg
    extract.copy("libavcodec.so.58")?;
    extract.copy("libavfilter.so.7")?;
    extract.copy("libavformat.so.58")?;
    extract.copy("libavresample.so.4")?;
    extract.copy("libavutil.so.56")?;

    // misc libs
    extract.copy("libswscale.so.5")?;
    extract.copy("libv8.so")?;
    extract.copy("libvpx.so.6")?;
    extract.copy("libopenvr_api.so")?;

    println!(" > copy ubuntu12_32 binaries");

    extract.copy("gldriverquery")?;
    extract.copy("vulkandriverquery")?;

    println!(" > copy steam-runtime libraries");

    extract.copy("libudev.so.0.13.0")?;
    extract.symlink("libudev.so.0", "libudev.so.0.13.0")?;

    extract.copy("libappindicator.so.1.0.0")?;
    extract.symlink("libappindicator.so.1", "libappindicator.so.1.0.0")?;

    extract.copy("libdbusmenu-glib.so.4.0.13")?;
    extract.symlink("libdbusmenu-glib.so.4", "libdbusmenu-glib.so.4.0.13")?;

    extract.copy("libdbusmenu-gtk.so.4.0.13")?;
    extract.symlink("libdbusmenu-gtk.so.4", "libdbusmenu-gtk.so.4.0.13")?;

    //extract.copy("libgtk-x11-2.0.so.0.2400.10")?;
    //extract.symlink("libgtk-x11-2.0.so.0", "libgtk-x11-2.0.so.0.2400.10")?;

    extract.copy("libgudev-1.0.so.0.1.1")?;
    extract.symlink("libgudev-1.0.so.0", "libgudev-1.0.so.0.1.1")?;

    extract.copy("libindicator.so.7.0.0")?;
    extract.symlink("libindicator.so.7", "libindicator.so.7.0.0")?;

    extract.copy("libnm.so.0.1.0")?;
    extract.symlink("libnm.so.0", "libnm.so.0.1.0")?;

    extract.copy("libSM.so.6.0.1")?;
    extract.symlink("libSM.so.6", "libSM.so.6.0.1")?;

    extract.copy("libICE.so.6.3.0")?;
    extract.symlink("libICE.so.6", "libICE.so.6.3.0")?;

    Ok(())
}
