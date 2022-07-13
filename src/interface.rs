        let config = dirs::config_dir().expect("why");

        let data = dirs::data_dir().expect("why");

        let esteem = format!("/usr/lib/{NAME}");
        let esteem_32 = esteem.join("i686");
        let esteem_64 = esteem.join("x86_64");

        let esteem_config = config.join(NAME);
        let esteem_data = data.join(NAME);

        let options = options::Options::parse();
        let Options {
            cef,
            cef_sandbox,
            game_id,
            developer,
            verbose,
        } = &options;

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
