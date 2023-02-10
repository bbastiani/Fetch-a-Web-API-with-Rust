use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadConfigError {
    #[error("Config file not found")]
    FileNotFound(#[from] std::io::Error),
    #[error("Config file not valid")]
    FileNotValid(#[from] serde_yaml::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub token_url: String,
    pub get_url: String,
    pub username: String,
    pub password: String,
    pub filename: String,
}

pub fn load_config_file() -> Result<ConfigFile, LoadConfigError> {
    let config_file = fs::read_to_string("config.yml")?;
    let config: ConfigFile = serde_yaml::from_str(&config_file)?;
    Ok(config)
}
