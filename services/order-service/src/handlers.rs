use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::{Order, CreateOrderRequest},
    services::OrderService,
    config::Config,
    error::AppError,
};

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Order service is healthy")
}

pub async fn create_order(
    State(config): State<Config>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<impl IntoResponse, AppError> {
    // In a real implementation, we would initialize the repository and service here
    // For now, we'll just return a mock response
    let order = Order {
        id: Uuid::new_v4(),
        user_id: request.user_id,
        product_name: request.product_name,
        quantity: request.quantity,
        total_price: request.total_price,
        created_at: time::OffsetDateTime::now_utc(),
        updated_at: time::OffsetDateTime::now_utc(),
    };
    
    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn get_order_by_id(
    State(config): State<Config>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // In a real implementation, we would fetch from the database
    // For now, we'll just return a mock response
    let order = Order {
        id,
        user_id: Uuid::new_v4(),
        product_name: "Test Product".to_string(),
        quantity: 1,
        total_price: 99.99,
        created_at: time::OffsetDateTime::now_utc(),
        updated_at: time::OffsetDateTime::now_utc(),
    };
    
    Ok(Json(order))
}