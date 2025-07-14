use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait TimeService: Send + Sync {
    async fn get_current_time(&self) -> Result<DateTime<Utc>, Box<dyn Error>>;
}
