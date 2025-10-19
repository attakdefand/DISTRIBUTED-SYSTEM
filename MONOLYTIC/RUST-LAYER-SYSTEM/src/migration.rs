//! Database migration runner
//! This module handles running database migrations using SQLx

use sqlx::{SqlitePool, migrate::MigrateDatabase};
use tracing::info;

pub struct MigrationRunner;

impl MigrationRunner {
    pub async fn run_migrations(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Checking if database exists: {}", database_url);
        
        // Create database if it doesn't exist
        if !sqlx::Sqlite::database_exists(database_url).await.unwrap_or(false) {
            info!("Creating database: {}", database_url);
            sqlx::Sqlite::create_database(database_url).await?;
        }

        info!("Connecting to database: {}", database_url);
        let pool = SqlitePool::connect(database_url).await?;

        info!("Running migrations");
        // This will run all migrations in the migrations directory
        sqlx::migrate!("./migrations").run(&pool).await?;

        info!("Migrations completed successfully");
        Ok(())
    }
}