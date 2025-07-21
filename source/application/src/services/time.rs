pub mod errors;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::services::time::errors::TimeServiceError;

#[async_trait]
pub trait TimeService: Send + Sync {
    async fn get_current_time(&self) -> Result<DateTime<Utc>, TimeServiceError>;
}
