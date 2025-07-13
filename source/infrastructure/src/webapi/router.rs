use std::sync::Arc;

use axum::{Router, routing::post};
use nimbus_vault_server_application::use_cases::ApplicationUseCases;

use crate::webapi::handlers::auth::register_user;

pub fn create_router(use_cases: Arc<ApplicationUseCases>) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .with_state(use_cases)
}
