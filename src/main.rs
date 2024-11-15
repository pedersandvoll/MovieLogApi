mod routes; // Add this line
use routes::all_routes;

#[tokio::main]
async fn main() {
    let app = all_routes();

    // Start the Axum server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
