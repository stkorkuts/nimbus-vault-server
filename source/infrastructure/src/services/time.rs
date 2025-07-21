use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use nimbus_vault_server_application::services::time::TimeService;

pub struct DefaultTimeService;

impl DefaultTimeService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl TimeService for DefaultTimeService {
    async fn get_current_time(&self) -> Result<DateTime<Utc>, ApplicationError> {
        Ok(Utc::now())
    }
}
