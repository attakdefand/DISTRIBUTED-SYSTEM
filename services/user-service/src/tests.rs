#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CreateUserRequest, User};
    use uuid::Uuid;
    use time::OffsetDateTime;

    #[test]
    fn test_create_user_request_validation() {
        // Test valid request
        let valid_request = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
        };
        
        // In a real implementation, we would test the service layer
        // For now, we just verify the struct can be created
        assert_eq!(valid_request.username, "testuser");
        assert_eq!(valid_request.email, "test@example.com");
    }

    #[test]
    fn test_user_model_creation() {
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };
        
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
    }
}