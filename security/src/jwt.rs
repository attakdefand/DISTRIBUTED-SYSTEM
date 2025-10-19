use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::SecurityError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
    pub service: String, // Service name
    pub scopes: Vec<String>, // Permissions
}

pub struct JwtToken {
    pub token: String,
    pub claims: Claims,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(
        &self,
        user_id: &str,
        service: &str,
        scopes: Vec<String>,
        expiration_seconds: i64,
    ) -> Result<JwtToken, SecurityError> {
        let now = OffsetDateTime::now_utc();
        let exp = now + time::Duration::seconds(expiration_seconds);
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
            service: service.to_string(),
            scopes,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        
        Ok(JwtToken {
            token,
            claims,
        })
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, SecurityError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}