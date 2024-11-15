pub mod auth;
pub mod tmdb;

use crate::routes::auth::auth_routes;
use crate::routes::tmdb::tmdb_routes;
use axum::Router;

pub fn all_routes() -> Router {
    tmdb_routes().merge(auth_routes()) // Merging the routes here
}
