use config::{ConfigError, Config as ConfigLoader, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_database_url")]
    pub database_url: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_database_url() -> String {
    "sqlite://db.sqlite3".to_string()
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = ConfigLoader::builder()
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}