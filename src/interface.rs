        let config = dirs::config_dir().expect("why");

        let data = dirs::data_dir().expect("why");

        let esteem = format!("/usr/lib/{NAME}");
        let esteem_32 = esteem.join("i686");
        let esteem_64 = esteem.join("x86_64");

        let esteem_config = config.join(NAME);
        let esteem_data = data.join(NAME);

        let Options {
            cef,
            cef_sandbox,
            game_id,
            developer,
            verbose,
        } = = options::Options::parse();

        let mut interface = Interface::new();

        interface.cef(cef);
        interface.cef_sandbox(cef_sandbox);
        interface.developer(developer);
        interface.verbose(verbose);

        if game_id {
            interface.open(Open::Game(id));
        } else {
            interface.open(Open::Normal);
        }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Open {
    Normal,
    Game(u32),
}

#[derive(Debug)]
pub struct Interface {
    cef: bool,
    cef_sandbox: bool,
    developer: u8,
    verbose: u8,
}

impl Interface {
    #[inline]
    pub const fn new() -> Self {
        Self {
            cef: true,
            cef_sandbox: true,
            developer: 0,
            verbose: 0,
        }
    }

    #[inline]
    pub fn cef(&mut self, enable: bool) -> &mut Self {
        self.cef = enable;
    }
    
    #[inline]
    pub fn cef_sandbox(&mut self, enable: bool) -> &mut Self {
        self.cef_sandbox = enable;
    }
    
    #[inline]
    pub fn developer(&mut self, enable: bool) -> &mut Self {
        self.developer = enable;
    }

    #[inline]
    pub fn open(self, open: Open) -> Result<(), ()> {
        Ok(())
    }

    #[inline]
    pub fn verbose(&mut self, enable: bool) -> &mut Self {
        self.verbose = enable;
    }
}
