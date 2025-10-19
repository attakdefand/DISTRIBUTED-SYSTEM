use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

// Common models that can be shared across services

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub status: String,
}