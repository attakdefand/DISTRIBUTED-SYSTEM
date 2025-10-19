use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ConfigError(pub String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Deserialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub services: HashMap<String, ServiceConfig>,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // For now, we'll create a simple config without using the config crate
        // In a real implementation, you would use the config crate properly
        Ok(Config {
            services: {
                let mut services = HashMap::new();
                services.insert("user-service".to_string(), ServiceConfig {
                    name: "user-service".to_string(),
                    host: "localhost".to_string(),
                    port: 3001,
                });
                services.insert("order-service".to_string(), ServiceConfig {
                    name: "order-service".to_string(),
                    host: "localhost".to_string(),
                    port: 3002,
                });
                services
            },
            host: "0.0.0.0".to_string(),
            port: 3000,
        })
    }

    pub fn get_service_url(&self, service_name: &str) -> Option<String> {
        self.services.get(service_name).map(|service| {
            format!("http://{}:{}", service.host, service.port)
        })
    }
}

#[cfg(test)]
mod tests;