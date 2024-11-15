use axum::{routing::get, Router};

pub fn tmdb_routes() -> Router {
    Router::new().route("/tmdb", get(|| async { "TMDB Route" }))
}
