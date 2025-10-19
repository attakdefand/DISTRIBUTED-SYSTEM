#[cfg(test)]
mod tests {
    use crate::{ServiceConfig, Config};

    #[test]
    fn test_service_config_creation() {
        let user_service = ServiceConfig {
            name: "user-service".to_string(),
            host: "localhost".to_string(),
            port: 3001,
        };
        
        assert_eq!(user_service.name, "user-service");
        assert_eq!(user_service.host, "localhost");
        assert_eq!(user_service.port, 3001);
    }

    #[test]
    fn test_config_creation() {
        let services = std::collections::HashMap::new();
        let config = Config {
            services,
            host: "0.0.0.0".to_string(),
            port: 3000,
        };
        
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
    }
}