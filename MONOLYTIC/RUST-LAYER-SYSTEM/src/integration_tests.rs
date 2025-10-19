//! Integration tests for the application
//! These tests verify that all components work together correctly

#[cfg(test)]
mod tests {
    use crate::models::User;
    use crate::repositories::UserRepository;
    use sqlx::SqlitePool;

    #[tokio::test]
    async fn test_user_repository() -> Result<(), Box<dyn std::error::Error>> {
        // Create an in-memory database for testing
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        
        // Run migrations
        // For testing, we'll create the table directly since the path is tricky in tests
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (id TEXT PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)"
        )
        .execute(&pool)
        .await?;
        
        let repo = UserRepository::new(pool);
        
        // Test creating a user
        let user = User::new("1".to_string(), "Test User".to_string(), "test@example.com".to_string());
        let saved_user = repo.create(&user).await?;
        
        assert_eq!(saved_user.id, "1");
        assert_eq!(saved_user.name, "Test User");
        assert_eq!(saved_user.email, "test@example.com");
        
        // Test finding a user by ID
        let found_user = repo.find_by_id("1").await?;
        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, "1");
        assert_eq!(found_user.name, "Test User");
        assert_eq!(found_user.email, "test@example.com");
        
        // Test finding a user by email
        let found_user = repo.find_by_email("test@example.com").await?;
        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.email, "test@example.com");
        
        // Test finding all users
        let users = repo.find_all().await?;
        assert_eq!(users.len(), 1);
        
        // Test updating a user
        let mut updated_user = found_user;
        updated_user.name = "Updated User".to_string();
        let saved_user = repo.update(&updated_user).await?;
        assert_eq!(saved_user.name, "Updated User");
        
        // Test deleting a user
        repo.delete("1").await?;
        let found_user = repo.find_by_id("1").await?;
        assert!(found_user.is_none());
        
        Ok(())
    }
}