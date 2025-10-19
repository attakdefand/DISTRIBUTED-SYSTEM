use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SharedError {
    #[error("Validation error: {message}")]
    ValidationError { message: String },
    
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
    
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },
}