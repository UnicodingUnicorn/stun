extern crate serde;
extern crate toml;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{ Deserialize, Serialize };

pub struct Config {
    pub port: String,
}

#[derive(Deserialize, Serialize)]
struct LoadableConfig {
    port: Option<String>,
}

pub fn get_config(filename: &str) -> Config {
    let default = Config {
        port: String::from("3478"),
    };

    let mut config_file = match File::open(Path::new(filename)) {
        Ok(config_file) => config_file,
        Err(_) => return default,
    };
    let mut config_string = String::new();
    match config_file.read_to_string(&mut config_string) {
        Ok(_) => (),
        Err(_) => return default,
    }

    let loaded_config:LoadableConfig = match toml::from_str(&config_string) {
        Ok(loaded_config) => loaded_config,
        Err(_) => return default,
    };

    let config = Config {
        port: match &loaded_config.port {
            Some(port) => port.clone(),
            None => String::from("3478"),
        },
    };

    config
}
