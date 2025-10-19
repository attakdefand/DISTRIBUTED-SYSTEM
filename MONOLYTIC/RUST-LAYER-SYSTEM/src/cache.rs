//! Caching service implementation
//! This module provides a simple caching layer using moka

use moka::future::Cache;
use std::time::Duration;

pub struct CacheService {
    cache: Cache<String, String>,
}

impl CacheService {
    pub fn new() -> Self {
        let cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(3600)) // 1 hour
            .build();
        
        Self { cache }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.cache.get(key).await
    }

    pub async fn set(&self, key: String, value: String) {
        self.cache.insert(key, value).await
    }

    pub async fn remove(&self, key: &str) {
        self.cache.invalidate(key).await
    }
}

impl Default for CacheService {
    fn default() -> Self {
        Self::new()
    }
}