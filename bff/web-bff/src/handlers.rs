use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tracing::info;

use crate::{
    config::Config,
    error::AppError,
};

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Web BFF is healthy")
}

pub async fn get_dashboard(
    State(_config): State<Config>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching dashboard data");
    
    // In a real implementation, we would fetch data from multiple services
    // and aggregate them for the frontend
    let dashboard_data = json!({
        "user_count": 1250,
        "order_count": 3420,
        "revenue": 125000.00,
        "recent_orders": [
            {"id": "1", "customer": "John Doe", "amount": 120.00},
            {"id": "2", "customer": "Jane Smith", "amount": 85.50},
            {"id": "3", "customer": "Bob Johnson", "amount": 210.75}
        ]
    });
    
    Ok(Json(dashboard_data))
}

pub async fn get_profile(
    State(_config): State<Config>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching user profile");
    
    // In a real implementation, we would fetch user data from the user service
    let profile_data = json!({
        "id": "user-123",
        "username": "johndoe",
        "email": "john.doe@example.com",
        "first_name": "John",
        "last_name": "Doe",
        "avatar": "https://example.com/avatar/johndoe.png",
        "preferences": {
            "theme": "dark",
            "notifications": true
        }
    });
    
    Ok(Json(profile_data))
}