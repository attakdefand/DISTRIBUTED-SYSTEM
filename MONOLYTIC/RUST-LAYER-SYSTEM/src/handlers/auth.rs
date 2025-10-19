//! Authentication handlers
//! This module contains handlers for authentication endpoints

use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::{auth::AuthService, config::Config, error::AppError};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    State(config): State<Config>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // In a real implementation, you would verify the user credentials
    // against a database and hash the password
    
    // For demo purposes, we'll just check if the user exists
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await
        .map_err(AppError::DatabaseError)?;
    
    let user = sqlx::query!(r#"SELECT id FROM users WHERE email = ?"#, payload.email)
        .fetch_optional(&pool)
        .await
        .map_err(AppError::DatabaseError)?;
    
    match user {
        Some(user_record) => {
            // Create auth service and generate token
            let auth_service = AuthService::new("my_secret_key");
            let token = auth_service.generate_token(&user_record.id)
                .map_err(|_| AppError::ValidationError("Failed to generate token".to_string()))?;
            
            Ok(Json(LoginResponse { token }))
        },
        None => Err(AppError::ValidationError("Invalid credentials".to_string()))
    }
}