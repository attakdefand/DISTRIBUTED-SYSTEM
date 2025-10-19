use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
}

#[derive(Clone, Debug)]
pub struct ServiceInfo {
    pub name: String,
    pub url: String,
    pub healthy: bool,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_service(&self, name: String, url: String) {
        let mut services = self.services.write().await;
        services.insert(name.clone(), ServiceInfo {
            name,
            url,
            healthy: true,
        });
    }

    pub async fn get_service(&self, name: &str) -> Option<ServiceInfo> {
        let services = self.services.read().await;
        services.get(name).cloned()
    }

    pub async fn get_all_services(&self) -> HashMap<String, ServiceInfo> {
        let services = self.services.read().await;
        services.clone()
    }
}