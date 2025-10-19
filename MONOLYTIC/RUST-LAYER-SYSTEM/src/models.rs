//! Domain models
//! This module contains the core domain entities with SQLx annotations for database operations

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(with = "time::serde::rfc3339::option")]
    #[sqlx(default)]
    pub created_at: Option<OffsetDateTime>,
}

impl User {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            created_at: Some(OffsetDateTime::now_utc()),
        }
    }
}