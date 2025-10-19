//! Admin API handlers
//! This module contains handlers for administrative endpoints

use axum::{
    extract::State,
    Json,
};
use serde::Serialize;
use crate::{config::Config, error::AppError};

#[derive(Serialize)]
pub struct AdminStats {
    pub user_count: i64,
    pub uptime: u64,
}

pub async fn get_stats(
    State(config): State<Config>,
) -> Result<Json<AdminStats>, AppError> {
    // In a real implementation, you would connect to the database
    // and fetch actual statistics
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await
        .map_err(AppError::DatabaseError)?;
    
    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .map_err(AppError::DatabaseError)?;
    
    let stats = AdminStats {
        user_count: user_count.0,
        uptime: 3600, // In a real app, this would be actual uptime
    };
    
    Ok(Json(stats))
}

pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "admin_api": true
    }))
}