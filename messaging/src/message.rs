use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub payload: serde_json::Value,
    pub message_type: String,
    pub source: String,
    pub destination: String,
    pub timestamp: OffsetDateTime,
    pub headers: std::collections::HashMap<String, String>,
}

impl Message {
    pub fn new(
        message_type: String,
        source: String,
        destination: String,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            payload,
            message_type,
            source,
            destination,
            timestamp: OffsetDateTime::now_utc(),
            headers: std::collections::HashMap::new(),
        }
    }

    pub fn with_correlation(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_causation(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
}

// Specific message types for common patterns

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderCreatedEvent {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub product_name: String,
    pub quantity: i32,
    pub total_price: f64,
    pub timestamp: OffsetDateTime,
}