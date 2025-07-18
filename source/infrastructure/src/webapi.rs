use std::{error::Error, sync::Arc};

use axum::{Router, routing::post, serve};
use nimbus_vault_server_application::use_cases::ApplicationUseCases;
use tokio::net::TcpListener;

use crate::webapi::handlers::auth::register_user;

mod handlers;

#[derive(Debug)]
pub struct WebApiSettings {
    base_addr: String,
}

pub struct WebApi {
    settings: WebApiSettings,
    router: Router,
}

impl WebApiSettings {
    pub fn new(base_addr: String) -> Self {
        WebApiSettings { base_addr }
    }
}

impl WebApi {
    pub fn new(settings: WebApiSettings, use_cases: Arc<ApplicationUseCases>) -> Self {
        let router = Self::create_router(use_cases);
        WebApi { settings, router }
    }

    fn create_router(use_cases: Arc<ApplicationUseCases>) -> Router {
        Router::new()
            .route("/register", post(register_user))
            .with_state(use_cases)
    }

    pub async fn serve(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.settings.base_addr.as_str()).await?;
        serve(listener, self.router.clone()).await?;
        Ok(())
    }
}
