use axum::response::{IntoResponse, Response};
use hyper::{StatusCode, header::CONTENT_TYPE};
use prost::Message;

pub mod auth;

fn into_proto_response<T: Message>(message: T, status_code: StatusCode) -> Response {
    (
        status_code,
        [(CONTENT_TYPE, "application/x-protobuf")],
        message.encode_to_vec(),
    )
        .into_response()
}
