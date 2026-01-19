mod health;
use axum::{Router, routing::get};

pub fn get_routes() -> Router {
    Router::new().route("/health", get(health::handler))
}
