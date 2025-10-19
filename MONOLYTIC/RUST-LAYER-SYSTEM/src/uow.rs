//! Unit of Work pattern implementation
//! This module provides a simple Unit of Work pattern for database transactions

use sqlx::{Sqlite, Transaction};
use std::ops::{Deref, DerefMut};

pub struct UnitOfWork {
    transaction: Transaction<'static, Sqlite>,
}

impl UnitOfWork {
    pub async fn new(pool: &sqlx::SqlitePool) -> Result<Self, sqlx::Error> {
        let transaction = pool.begin().await?;
        Ok(Self { transaction })
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.transaction.commit().await
    }

    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        self.transaction.rollback().await
    }
}

impl Deref for UnitOfWork {
    type Target = Transaction<'static, Sqlite>;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl DerefMut for UnitOfWork {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction
    }
}