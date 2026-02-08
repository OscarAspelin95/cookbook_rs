use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid socket address")]
    InvalidSocketAddress(#[from] std::net::AddrParseError),

    #[error("IO error")]
    IOError(#[from] std::io::Error),

    #[error("Serialization error")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid payload")]
    InvalidPayloadError(String),

    #[error("Validation error")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Extraction error")]
    ExtractionError(#[from] axum::extract::rejection::BytesRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidSocketAddress(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::IOError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::SerializationError(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            Self::ValidationError(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            Self::InvalidPayloadError(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            Self::ExtractionError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
