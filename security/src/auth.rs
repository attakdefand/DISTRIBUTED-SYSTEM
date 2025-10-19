use crate::{JwtService, Claims, SecurityError};
use std::collections::HashSet;

pub struct AuthService {
    jwt_service: JwtService,
    allowed_services: HashSet<String>,
}

impl AuthService {
    pub fn new(jwt_service: JwtService, allowed_services: Vec<String>) -> Self {
        Self {
            jwt_service,
            allowed_services: allowed_services.into_iter().collect(),
        }
    }

    pub fn authenticate(&self, token: &str) -> Result<Claims, SecurityError> {
        let claims = self.jwt_service.validate_token(token)?;
        
        // Check if the service is allowed
        if !self.allowed_services.contains(&claims.service) {
            return Err(SecurityError::AuthenticationFailed(
                "Service not allowed".to_string()
            ));
        }
        
        Ok(claims)
    }

    pub fn authorize(&self, claims: &Claims, required_scope: &str) -> Result<(), SecurityError> {
        if !claims.scopes.contains(&required_scope.to_string()) {
            return Err(SecurityError::AuthorizationFailed(
                format!("Missing required scope: {}", required_scope)
            ));
        }
        
        Ok(())
    }
}