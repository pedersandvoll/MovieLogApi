use std::fmt::format;

use axum::{extract::Query, response::Html, routing::post, Router};
use serde::Deserialize;

pub fn tmdb_routes() -> Router {
    Router::new().route("/searchMovie", post(search_movie))
}

#[derive(Debug, Deserialize)]
struct SearchMovie {
    search_word: String,
}

async fn search_movie(Query(params): Query<SearchMovie>) -> Html<String> {
    let search_word = params.search_word;
    Html(format!("Movie result of <strong>{search_word}</strong>"))
}
