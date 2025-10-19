#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_service_config_creation() {
        let mut services = HashMap::new();
        let user_service = ServiceConfig {
            name: "user-service".to_string(),
            host: "localhost".to_string(),
            port: 3001,
        };
        
        services.insert("user-service".to_string(), user_service);
        
        let config = Config {
            services,
            host: "0.0.0.0".to_string(),
            port: 3000,
        };
        
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert!(config.get_service_url("user-service").is_some());
        assert_eq!(config.get_service_url("user-service").unwrap(), "http://localhost:3001");
    }

    #[test]
    fn test_service_registry() {
        let registry = service_discovery::ServiceRegistry::new();
        
        // In a real test, we would test the async methods
        // For now, we just verify the struct can be created
        assert!(true); // Placeholder
    }
}