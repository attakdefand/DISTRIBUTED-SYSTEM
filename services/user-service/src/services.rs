use crate::{
    models::{User, CreateUserRequest},
    repositories::UserRepository,
    error::AppError,
};

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, AppError> {
        // Validate input
        if request.username.is_empty() {
            return Err(AppError::ValidationError("Username cannot be empty".to_string()));
        }

        if request.email.is_empty() {
            return Err(AppError::ValidationError("Email cannot be empty".to_string()));
        }

        // Create user
        let user = self.repository.create(&request.username, &request.email).await?;
        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<User, AppError> {
        let user = self.repository.find_by_id(id).await?;
        Ok(user)
    }
}