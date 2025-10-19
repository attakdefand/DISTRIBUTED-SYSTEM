use crate::{
    models::{Order, CreateOrderRequest},
    repositories::OrderRepository,
    error::AppError,
};

pub struct OrderService {
    repository: OrderRepository,
}

impl OrderService {
    pub fn new(repository: OrderRepository) -> Self {
        Self { repository }
    }

    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<Order, AppError> {
        // Validate input
        if request.product_name.is_empty() {
            return Err(AppError::ValidationError("Product name cannot be empty".to_string()));
        }

        if request.quantity <= 0 {
            return Err(AppError::ValidationError("Quantity must be greater than zero".to_string()));
        }

        if request.total_price <= 0.0 {
            return Err(AppError::ValidationError("Total price must be greater than zero".to_string()));
        }

        // Create order
        let order = self.repository.create(
            request.user_id,
            &request.product_name,
            request.quantity,
            request.total_price
        ).await?;
        
        Ok(order)
    }

    pub async fn get_order_by_id(&self, id: uuid::Uuid) -> Result<Order, AppError> {
        let order = self.repository.find_by_id(id).await?;
        Ok(order)
    }
}