use axum::{routing::get, Router};

pub fn auth_routes() -> Router {
    Router::new().route("/auth", get(|| async { "Auth Route" }))
}
