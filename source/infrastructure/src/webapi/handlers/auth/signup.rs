use std::{collections::HashMap, sync::Arc};

use axum::{body::Bytes, extract::State, response::Response};
use hyper::StatusCode;
use nimbus_vault_server_application::use_cases::{
    ApplicationUseCases,
    user::{
        UserSchema,
        register::{RegisterUserResponseSchema, SignUpRequestSchema},
    },
};
use prost::Message;

use crate::{
    proto::auth::{
        AuthErrorCodeProto, AuthErrorProto, SignUpRequestProto, SignUpResponseProto,
        SignUpSuccessResponseProto, sign_up_response_proto::Result,
    },
    webapi::handlers::{auth::into_auth_error_response, into_proto_response},
};

impl From<SignUpSuccessResponseProto> for SignUpResponseProto {
    fn from(value: SignUpSuccessResponseProto) -> Self {
        SignUpResponseProto {
            result: Some(Result::Success(value)),
        }
    }
}

impl From<AuthErrorProto> for SignUpResponseProto {
    fn from(value: AuthErrorProto) -> Self {
        SignUpResponseProto {
            result: Some(Result::Error(value)),
        }
    }
}

fn into_signup_error_response(
    auth_error_code: AuthErrorCodeProto,
    message: &str,
    metadata: Option<HashMap<String, String>>,
) -> Response {
    into_proto_response(
        SignUpResponseProto {
            result: Some(Result::Error(AuthErrorProto {
                code: auth_error_code.into(),
                message: message.to_owned(),
                metadata: metadata.unwrap_or(HashMap::default()),
            })),
        },
        StatusCode::BAD_REQUEST,
    )
}

pub async fn signup(State(use_cases): State<Arc<ApplicationUseCases>>, body: Bytes) -> Response {
    let SignUpRequestProto {
        username,
        password,
        e2e_key_hash,
        encrypted_master_key,
    } = match SignUpRequestProto::decode(body.as_ref()) {
        Ok(req) => req,
        Err(err) => {
            eprintln!("{err}");
            return into_proto_response(
                AuthErrorProto {
                    code: AuthErrorCodeProto::WrongBodyFormat.into(),
                    message: "Message is not in protobuf SignUpRequestProto format".to_owned(),
                    metadata: HashMap::default(),
                },
                StatusCode::BAD_REQUEST,
            );
        }
    };

    let request = SignUpRequestSchema {
        username,
        password,
        e2e_key_hash,
        encrypted_master_key,
    };

    let RegisterUserResponseSchema { user } = match use_cases.register_user(request).await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return into_proto_response(
                AuthErrorProto {
                    code: AuthErrorCodeProto::Undefined.into(),
                    message: "Error while processing user signing up".to_owned(),
                    metadata: HashMap::default(),
                },
                StatusCode::BAD_REQUEST,
            );
        }
    };

    let response = SignUpSuccessResponseProto {
        user: Some(user.into()),
        refresh_token: Some(()),
        access_token: Some(()),
    };

    into_proto_response(response, StatusCode::CREATED)
}
