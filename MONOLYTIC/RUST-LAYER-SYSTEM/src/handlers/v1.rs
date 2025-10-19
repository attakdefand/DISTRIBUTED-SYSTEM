//! API v1 handlers
//! This module contains handlers for API version 1

pub mod users {
    use axum::{
        extract::{Path, State},
        Json,
    };
    use serde::{Deserialize, Serialize};
    use crate::{models::User, repositories::UserRepository, config::Config, error::AppError};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct CreateUserRequest {
        pub name: String,
        pub email: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UserResponse {
        pub id: String,
        pub name: String,
        pub email: String,
    }

    impl From<User> for UserResponse {
        fn from(user: User) -> Self {
            Self {
                id: user.id,
                name: user.name,
                email: user.email,
            }
        }
    }

    pub async fn create(
        State(config): State<Config>,
        Json(payload): Json<CreateUserRequest>,
    ) -> Result<Json<UserResponse>, AppError> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&config.database_url)
            .await
            .map_err(AppError::DatabaseError)?;
        
        let repo = UserRepository::new(pool);
        let user = User::new(Uuid::new_v4().to_string(), payload.name, payload.email);
        let saved_user = repo.create(&user).await?;
        
        Ok(Json(saved_user.into()))
    }

    pub async fn get_by_id(
        State(config): State<Config>,
        Path(id): Path<String>,
    ) -> Result<Json<UserResponse>, AppError> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&config.database_url)
            .await
            .map_err(AppError::DatabaseError)?;
        
        let repo = UserRepository::new(pool);
        let user = repo.find_by_id(&id).await?;
        
        match user {
            Some(user) => Ok(Json(user.into())),
            None => Err(AppError::NotFound),
        }
    }
}