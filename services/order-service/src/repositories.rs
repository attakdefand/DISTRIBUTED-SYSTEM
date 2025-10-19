use crate::{models::Order, error::AppError};
use sqlx::PgPool;
use uuid::Uuid;
use time::OffsetDateTime;

pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user_id: Uuid, product_name: &str, quantity: i32, total_price: f64) -> Result<Order, AppError> {
        // For now, return a mock order since we don't have a real database
        Ok(Order {
            id: Uuid::new_v4(),
            user_id,
            product_name: product_name.to_string(),
            quantity,
            total_price,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        })
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Order, AppError> {
        // For now, return a mock order since we don't have a real database
        Ok(Order {
            id,
            user_id: Uuid::new_v4(),
            product_name: "Test Product".to_string(),
            quantity: 1,
            total_price: 99.99,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        })
    }
}