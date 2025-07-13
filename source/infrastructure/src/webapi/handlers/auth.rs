use std::sync::Arc;

use axum::{body::Bytes, extract::State, response::IntoResponse};
use hyper::StatusCode;
use nimbus_vault_server_application::use_cases::{self, ApplicationUseCases};
use prost::Message;

use crate::proto::RegisterUserRequest;

pub async fn register_user(
    State(use_cases): State<Arc<ApplicationUseCases>>,
    body: Bytes,
) -> impl IntoResponse {
    let request = match RegisterUserRequest::decode(body.as_ref()) {
        Ok(req) => req,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid protobuf message".to_owned())
        } 
    }

    let result = match use_cases.register_user().await {
        Ok(res) => res,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error while processing user registration".to_owned())
        } 
    };

    
}
