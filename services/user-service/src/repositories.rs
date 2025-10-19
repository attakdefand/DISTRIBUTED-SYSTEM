use crate::{models::User, error::AppError};
use sqlx::PgPool;
use uuid::Uuid;
use time::OffsetDateTime;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, username: &str, email: &str) -> Result<User, AppError> {
        // For now, return a mock user since we don't have a real database
        Ok(User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        })
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
        // For now, return a mock user since we don't have a real database
        Ok(User {
            id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        })
    }
}