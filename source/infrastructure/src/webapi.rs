use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use prost::Message;
use tower_http::cors::CorsLayer;

use crate::proto::{RegisterUserRequest, RegisterUserResponse, User};

// Custom extractor for protobuf messages
pub struct ProtobufMessage<T>(pub T);

impl<T> axum::extract::FromRequest<AppState> for ProtobufMessage<T>
where
    T: Message + Default,
{
    type Rejection = (StatusCode, String);

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let bytes = axum::body::Bytes::from_request(req, _state)
            .await
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Failed to extract bytes".to_string(),
                )
            })?;

        let message = T::decode(bytes.as_ref()).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Failed to decode protobuf".to_string(),
            )
        })?;

        Ok(ProtobufMessage(message))
    }
}

// Custom response for protobuf messages
pub struct ProtobufResponse<T>(pub T);

impl<T> IntoResponse for ProtobufResponse<T>
where
    T: Message,
{
    fn into_response(self) -> Response {
        let bytes = self.0.encode_to_vec();
        (
            StatusCode::OK,
            [("content-type", "application/x-protobuf")],
            bytes,
        )
            .into_response()
    }
}

// Handler functions
async fn register_user(
    State(_state): State<AppState>,
    ProtobufMessage(request): ProtobufMessage<RegisterUserRequest>,
) -> Result<ProtobufResponse<RegisterUserResponse>, (StatusCode, String)> {
    // Validate request
    if request.username.is_empty() || request.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username and password are required".to_string(),
        ));
    }

    // TODO: Implement your business logic here
    // let user_service = &_state.user_service;
    // let result = user_service.register_user(request).await?;

    // For now, return a mock response
    let user = User {
        id: "user_123".to_string(),
        username: request.username,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let response = RegisterUserResponse {
        user: Some(user),
        signed_certificate: request.certificate_signing_request, // Mock: return the CSR as signed
    };

    Ok(ProtobufResponse(response))
}

// Create the router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/users/register", post(register_user))
        .layer(CorsLayer::permissive()) // Allow CORS for development
        .with_state(state)
}

// Server setup function
pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        // Initialize your services here
    };

    let app = create_router(state);

    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
