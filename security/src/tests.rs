#[cfg(test)]
mod tests {
    use crate::{jwt::JwtService, auth::AuthService};

    #[test]
    fn test_jwt_token_generation() {
        let jwt_service = JwtService::new("test_secret");
        let scopes = vec!["read".to_string(), "write".to_string()];
        
        let token = jwt_service.generate_token(
            "user123",
            "test_service",
            scopes.clone(),
            3600, // 1 hour
        ).expect("Failed to generate token");

        assert_eq!(token.claims.sub, "user123");
        assert_eq!(token.claims.service, "test_service");
        assert_eq!(token.claims.scopes, scopes);
    }

    #[test]
    fn test_jwt_token_validation() {
        let jwt_service = JwtService::new("test_secret");
        let scopes = vec!["read".to_string(), "write".to_string()];
        
        let token = jwt_service.generate_token(
            "user123",
            "test_service",
            scopes.clone(),
            3600, // 1 hour
        ).expect("Failed to generate token");

        let validated_claims = jwt_service.validate_token(&token.token)
            .expect("Failed to validate token");

        assert_eq!(validated_claims.sub, "user123");
        assert_eq!(validated_claims.service, "test_service");
        assert_eq!(validated_claims.scopes, scopes);
    }

    #[test]
    fn test_auth_service_authentication() {
        let jwt_service = JwtService::new("test_secret");
        let allowed_services = vec!["test_service".to_string()];
        let auth_service = AuthService::new(jwt_service, allowed_services);
        
        let scopes = vec!["read".to_string()];
        let test_jwt_service = JwtService::new("test_secret");
        let token = test_jwt_service.generate_token(
            "user123",
            "test_service",
            scopes,
            3600, // 1 hour
        ).expect("Failed to generate token");

        let claims = auth_service.authenticate(&token.token)
            .expect("Failed to authenticate");

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.service, "test_service");
    }

    #[test]
    fn test_auth_service_authorization() {
        let jwt_service = JwtService::new("test_secret");
        let allowed_services = vec!["test_service".to_string()];
        let auth_service = AuthService::new(jwt_service, allowed_services);
        
        let scopes = vec!["read".to_string(), "write".to_string()];
        let claims = crate::jwt::Claims {
            sub: "user123".to_string(),
            exp: (time::OffsetDateTime::now_utc() + time::Duration::hours(1)).unix_timestamp() as usize,
            iat: time::OffsetDateTime::now_utc().unix_timestamp() as usize,
            service: "test_service".to_string(),
            scopes: scopes.clone(),
        };

        // Test successful authorization
        assert!(auth_service.authorize(&claims, "read").is_ok());
        assert!(auth_service.authorize(&claims, "write").is_ok());

        // Test failed authorization
        assert!(auth_service.authorize(&claims, "delete").is_err());
    }
}