use std::collections::HashMap;

use axum::response::Response;
use hyper::StatusCode;

use crate::{
    proto::auth::{
        AuthErrorCodeProto, AuthErrorProto, SignUpResponseProto, sign_up_response_proto::Result,
    },
    webapi::handlers::into_proto_response,
};

pub mod refresh;
pub mod signin;
pub mod signup;

fn into_auth_error_response(
    auth_error_code: AuthErrorCodeProto,
    message: Option<&str>,
    metadata: Option<HashMap<String, String>>,
) -> Response {
    let message: &str = message.unwrap_or(match auth_error_code {
        AuthErrorCodeProto::Undefined => "Unknown auth error",
        AuthErrorCodeProto::WrongBodyFormat => "Wrong request body format",
        AuthErrorCodeProto::ValidationError => "Some fields have invalid values",
        AuthErrorCodeProto::UserAlreadyExists => "User with this username already exists",
        AuthErrorCodeProto::UserNotFound => "User with this username is not found",
        AuthErrorCodeProto::InvalidE2eKeyHash => "Invalid E2E key provided",
        AuthErrorCodeProto::InvalidPassword => "Invalid password provided",
        AuthErrorCodeProto::InvalidRefreshToken => {
            "Invalid (Revoked/Expired) refresh token provided"
        }
    });
    let metadata = metadata.unwrap_or(HashMap::default());

    into_proto_response(
        SignUpResponseProto {
            result: Some(Result::Error(AuthErrorProto {
                code: AuthErrorCodeProto::WrongBodyFormat.into(),
                message: message.to_owned(),
                metadata: metadata,
            })),
        },
        StatusCode::BAD_REQUEST,
    )
}
