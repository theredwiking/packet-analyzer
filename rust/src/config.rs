use serde::Deserialize;
use std::fs;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub database: DBConfig,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct NetworkConfig {
    pub interface: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct DBConfig {
    pub url: String,
    pub bucket: String,
    pub token: String,
}

pub fn load_config() -> Result<Config, ()> {
    let toml_content = match fs::read_to_string("config.toml") {
        Ok(x) => x,
        Err(error) => {println!("{:?}", error); std::process::exit(0)},
    };

    let config: Config = match toml::from_str(&toml_content) {
        Ok(x) => x,
        Err(error) => {println!("{:?}", error); std::process::exit(0)},
    };

    Ok(config)
}
