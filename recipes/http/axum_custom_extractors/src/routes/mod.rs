mod health;
mod user;
use axum::{
    Router,
    routing::{get, post},
};

pub fn get_routes() -> Router {
    let health_route = Router::new().route("/", get(health::handler));
    let user_route = Router::new().route("/", post(user::post_handler));

    Router::new()
        .nest("/health", health_route)
        .nest("/user", user_route)
}
