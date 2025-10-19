use moka::future::Cache as MokaCache;
use serde_json::Value;
use std::time::Duration;

pub struct Cache {
    cache: MokaCache<String, Value>,
}

impl Cache {
    pub fn new() -> Self {
        let cache = MokaCache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(300)) // 5 minutes
            .build();
            
        Self { cache }
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        self.cache.get(key).await
    }

    pub async fn set(&self, key: String, value: Value) {
        self.cache.insert(key, value).await
    }

    pub async fn remove(&self, key: &str) {
        self.cache.invalidate(key).await
    }
}