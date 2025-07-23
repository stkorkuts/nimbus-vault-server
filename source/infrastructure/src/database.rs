pub mod db_entities;
pub mod enums;
pub mod errors;

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

use crate::database::errors::DatabaseError;

#[derive(Debug)]
pub struct DatabaseSettings {
    database_url: String,
    max_connections: u32,
}

pub struct Database {
    pool: PgPool,
}

impl DatabaseSettings {
    pub fn new(database_url: String) -> Self {
        DatabaseSettings {
            database_url,
            max_connections: 32,
        }
    }

    pub fn with_max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = max_connections;
        self
    }
}

impl Database {
    pub async fn new(settings: DatabaseSettings) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(settings.max_connections)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(&settings.database_url)
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
