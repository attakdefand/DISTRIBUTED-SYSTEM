#[cfg(test)]
mod tests {
    use crate::config::{Config, ServiceConfig};
    use crate::cache::Cache;
    use std::collections::HashMap;

    #[test]
    fn test_bff_config_creation() {
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
            port: 3003,
            cache_ttl_seconds: 300,
        };
        
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3003);
        assert_eq!(config.cache_ttl_seconds, 300);
    }

    #[test]
    fn test_cache_creation() {
        let cache = Cache::new();
        
        // In a real test, we would test the cache operations
        // For now, we just verify the struct can be created
        assert!(true); // Placeholder
    }
}