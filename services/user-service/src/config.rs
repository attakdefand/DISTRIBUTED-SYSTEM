use serde::Deserialize;
use microservice_config::ConfigError;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // For now, we'll create a simple config without using the config crate
        // In a real implementation, you would use the config crate properly
        Ok(Config {
            database_url: "sqlite:user_service.db".to_string(),
            host: "0.0.0.0".to_string(),
            port: 3001,
        })
    }
}