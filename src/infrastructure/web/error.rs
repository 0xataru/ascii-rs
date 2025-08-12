use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Payload too large")]
    PayloadTooLarge,
    #[error("Unsupported media type")]
    UnsupportedMediaType,
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            WebError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            WebError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            WebError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            WebError::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "Payload too large".to_string(),
            ),
            WebError::UnsupportedMediaType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Unsupported media type".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}
