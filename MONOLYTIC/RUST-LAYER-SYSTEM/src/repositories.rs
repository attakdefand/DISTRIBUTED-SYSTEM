//! Repository layer for data access
//! This module implements database queries and data access logic using SQLx

use crate::models::User;
use crate::error::AppError;
use sqlx::{SqlitePool, query, query_as};

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<User, AppError> {
        query(
            "INSERT INTO users (id, name, email, created_at) VALUES (?, ?, ?, ?)"
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(user.created_at)
        .execute(&self.pool)
        .await?;

        Ok(user.clone())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let user = query_as::<_, User>(
            "SELECT id, name, email, created_at FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = query_as::<_, User>(
            "SELECT id, name, email, created_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let users = query_as::<_, User>(
            "SELECT id, name, email, created_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn update(&self, user: &User) -> Result<User, AppError> {
        query(
            "UPDATE users SET name = ?, email = ? WHERE id = ?"
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.id)
        .execute(&self.pool)
        .await?;

        Ok(user.clone())
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        query(
            "DELETE FROM users WHERE id = ?"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}