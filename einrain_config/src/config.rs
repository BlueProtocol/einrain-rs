use crate::BOT_CONFIG;
use serde::{Serialize, Deserialize};
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct BotConfig {
    token: String,
    owner_id: u64,
    embed_role_id: u64,
    log_file: String,
    colour: u32,
}

impl BotConfig {
    pub fn set(config_path: &str) {
        let config: BotConfig =
            toml::from_str(&fs::read_to_string(config_path).unwrap_or_else(|err| {
                if err.kind() == io::ErrorKind::NotFound {
                    let default_cfg = BotConfig {
                        token: "AAAAAAAAAAAAAAAAAAAA.AAAAAAA.AAAAAAAAAAAAAA".to_string(),
                        owner_id: 123456789012345678,
                        embed_role_id: 123456789012345678,
                        log_file: "log.txt".to_string(),
                        colour: 12345678,
                    };

                    let default_cfg_str = toml::to_string_pretty(&default_cfg).expect("failed to serialize config");
                    
                    fs::write(config_path, &default_cfg_str).unwrap_or_else(|_| {
                        panic!(
                            "Couldn't write the default config, write it manually please:\n{}",
                            default_cfg_str
                        )
                    });

                    panic!("Created the default config, edit it and restart please");
                } else {
                    panic!("{}", err)
                }
            }))
            .expect("Looks like something is wrong with your config");

        BOT_CONFIG
            .set(config)
            .unwrap_or_else(|_| panic!("Couldn't set the config to BOT_CONFIG"));
    }

    pub fn get() -> Option<&'static BotConfig> {
        BOT_CONFIG.get()
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn owner_id(&self) -> u64 {
        self.owner_id
    }

    pub fn embed_role_id(&self) -> u64 {
        self.embed_role_id
    }

    pub fn log_file(&self) -> &String {
        &self.log_file
    }

    pub fn colour(&self) -> u32 {
        self.colour
    }
}