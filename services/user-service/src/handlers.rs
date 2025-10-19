use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::{User, CreateUserRequest},
    services::UserService,
    config::Config,
    error::AppError,
};

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "User service is healthy")
}

pub async fn create_user(
    State(config): State<Config>,
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // In a real implementation, we would initialize the repository and service here
    // For now, we'll just return a mock response
    let user = User {
        id: Uuid::new_v4(),
        username: request.username,
        email: request.email,
        created_at: time::OffsetDateTime::now_utc(),
        updated_at: time::OffsetDateTime::now_utc(),
    };
    
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user_by_id(
    State(config): State<Config>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // In a real implementation, we would fetch from the database
    // For now, we'll just return a mock response
    let user = User {
        id,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        created_at: time::OffsetDateTime::now_utc(),
        updated_at: time::OffsetDateTime::now_utc(),
    };
    
    Ok(Json(user))
}