use reqwest::Client;
use crate::{config::Config, error::AppError};

pub struct ServiceClient {
    client: Client,
    config: Config,
}

impl ServiceClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn get_user(&self, user_id: &str) -> Result<serde_json::Value, AppError> {
        let service_url = self.config.get_service_url("user-service")
            .ok_or_else(|| AppError::ServiceUnavailable("User service not configured".to_string()))?;

        let url = format!("{}/users/{}", service_url, user_id);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let body = response.text().await?;
            let user: serde_json::Value = serde_json::from_str(&body)?;
            Ok(user)
        } else {
            Err(AppError::ClientError(reqwest::Error::from(response.error_for_status().unwrap_err())))
        }
    }

    pub async fn get_order(&self, order_id: &str) -> Result<serde_json::Value, AppError> {
        let service_url = self.config.get_service_url("order-service")
            .ok_or_else(|| AppError::ServiceUnavailable("Order service not configured".to_string()))?;

        let url = format!("{}/orders/{}", service_url, order_id);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let body = response.text().await?;
            let order: serde_json::Value = serde_json::from_str(&body)?;
            Ok(order)
        } else {
            Err(AppError::ClientError(reqwest::Error::from(response.error_for_status().unwrap_err())))
        }
    }
}