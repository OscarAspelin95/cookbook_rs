use crate::extractors::{User, ValidatedJson};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::info;

pub async fn post_handler(ValidatedJson(user): ValidatedJson<User>) -> Response {
    info!("Successfully parsed valid user: {:?}", user);
    (StatusCode::OK, "OK").into_response()
}
