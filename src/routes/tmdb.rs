use dotenv::dotenv;
use std::env;

use axum::{extract::Query, response::Json, routing::get, Router};
use serde::Deserialize;
use serde_json::{from_str, Value};

pub fn tmdb_routes() -> Router {
    Router::new().route("/searchMovie", get(search_movie))
}

#[derive(Debug, Deserialize)]
struct SearchMovie {
    search_word: String,
    page: u32,
}

fn get_tmdb_api_key() -> Result<String, String> {
    dotenv().ok();
    env::var("TMDB_API_KEY").map_err(|e| e.to_string())
}

async fn search_movie(Query(params): Query<SearchMovie>) -> Result<Json<Value>, String> {
    let client = reqwest::Client::new();
    let tmdb_api_key = get_tmdb_api_key()?;

    let search_word = params.search_word;
    let page = params.page;

    let tmdb_request_url = format!("https://api.themoviedb.org/3/search/movie?query={search_word}&include_adult=true&language=en-US&page={page}");

    let res = client
        .get(tmdb_request_url)
        .header("Authorization", format!("Bearer {}", tmdb_api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;
    let json: Value = from_str(&body).map_err(|e| e.to_string())?;
    Ok(Json(json))
}
