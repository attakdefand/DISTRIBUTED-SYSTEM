use thiserror::Error;
use jsonwebtoken::errors::Error as JwtError;

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("JWT error: {0}")]
    JwtError(#[from] JwtError),
    
    #[error("TLS error: {0}")]
    TlsError(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
}