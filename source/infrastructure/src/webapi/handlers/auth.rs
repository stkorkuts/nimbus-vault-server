use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    response::{IntoResponse, Response},
};
use hyper::{StatusCode, header::CONTENT_TYPE};
use nimbus_vault_server_application::use_cases::{
    ApplicationUseCases, user::register::RegisterUserRequestSchema,
};
use prost::Message;

use crate::proto::{RegisterUserRequest, RegisterUserResponse, User};

pub async fn register_user(
    State(use_cases): State<Arc<ApplicationUseCases>>,
    body: Bytes,
) -> Response {
    let RegisterUserRequest {
        username,
        password,
        e2e_key_hash,
        encrypted_master_key,
    } = match RegisterUserRequest::decode(body.as_ref()) {
        Ok(req) => req,
        Err(err) => {
            eprintln!("{err}");
            return (
                StatusCode::BAD_REQUEST,
                "Invalid protobuf message".to_owned(),
            )
                .into_response();
        }
    };

    let request = RegisterUserRequestSchema {
        username,
        password,
        e2e_key_hash,
        encrypted_master_key,
    };

    let result = match use_cases.register_user(request).await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while processing user registration".to_owned(),
            )
                .into_response();
        }
    };

    let response = RegisterUserResponse {
        user: Some(User {
            id: result.user.id,
            username: result.user.username,
            encrypted_master_key: result.user.encrypted_master_key,
        }),
    };

    (
        StatusCode::CREATED,
        [(CONTENT_TYPE, "application/x-protobuf")],
        response.encode_to_vec(),
    )
        .into_response()
}
