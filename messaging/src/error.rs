use thiserror::Error;
use serde_json::Error as SerdeError;

#[derive(Error, Debug)]
pub enum MessagingError {
    #[error("NATS error: {0}")]
    NatsError(#[from] async_nats::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeError),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Publish error: {0}")]
    PublishError(String),
    
    #[error("Subscribe error: {0}")]
    SubscribeError(String),
}