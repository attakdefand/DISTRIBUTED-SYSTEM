//! Background job scheduler
//! This module provides a simple in-process job scheduler

use tokio::time::{interval, Duration};
use tracing::info;

pub struct JobScheduler;

impl JobScheduler {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) {
        // Spawn a background task for periodic jobs
        tokio::spawn(async {
            let mut interval = interval(Duration::from_secs(60)); // Run every minute
            loop {
                interval.tick().await;
                info!("Running periodic background job");
                // Add your background job logic here
            }
        });
    }
}

impl Default for JobScheduler {
    fn default() -> Self {
        Self::new()
    }
}