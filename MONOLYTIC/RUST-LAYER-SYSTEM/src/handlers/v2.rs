//! API v2 handlers
//! This module contains handlers for API version 2 with enhanced features

pub mod users {
    use axum::{
        extract::{Path, State},
        Json,
    };
    use serde::{Deserialize, Serialize};
    use crate::{models::User, repositories::UserRepository, config::Config, error::AppError};
    use uuid::Uuid;
    use time::OffsetDateTime;

    #[derive(Serialize, Deserialize)]
    pub struct CreateUserRequest {
        pub name: String,
        pub email: String,
        #[serde(default)]
        pub preferences: UserPreferences,
    }
    
    #[derive(Serialize, Deserialize, Default)]
    pub struct UserPreferences {
        #[serde(default)]
        pub newsletter: bool,
        #[serde(default)]
        pub notifications: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UserResponse {
        pub id: String,
        pub name: String,
        pub email: String,
        pub preferences: UserPreferences,
        pub created_at: Option<OffsetDateTime>,
        pub updated_at: Option<OffsetDateTime>,
    }

    impl From<User> for UserResponse {
        fn from(user: User) -> Self {
            Self {
                id: user.id,
                name: user.name,
                email: user.email,
                preferences: UserPreferences::default(),
                created_at: user.created_at,
                updated_at: Some(OffsetDateTime::now_utc()),
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
        
        let response = UserResponse {
            id: saved_user.id,
            name: saved_user.name,
            email: saved_user.email,
            preferences: payload.preferences,
            created_at: saved_user.created_at,
            updated_at: Some(OffsetDateTime::now_utc()),
        };
        
        Ok(Json(response))
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
    
    pub async fn update(
        State(config): State<Config>,
        Path(id): Path<String>,
        Json(payload): Json<CreateUserRequest>,
    ) -> Result<Json<UserResponse>, AppError> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&config.database_url)
            .await
            .map_err(AppError::DatabaseError)?;
        
        let repo = UserRepository::new(pool);
        let existing_user = repo.find_by_id(&id).await?;
        
        match existing_user {
            Some(mut user) => {
                user.name = payload.name;
                user.email = payload.email;
                let updated_user = repo.update(&user).await?;
                
                let response = UserResponse {
                    id: updated_user.id,
                    name: updated_user.name,
                    email: updated_user.email,
                    preferences: payload.preferences,
                    created_at: updated_user.created_at,
                    updated_at: Some(OffsetDateTime::now_utc()),
                };
                
                Ok(Json(response))
            },
            None => Err(AppError::NotFound),
        }
    }
}