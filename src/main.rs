mod handlers;
mod response;
mod routes;
mod utils;

use tokio;

#[tokio::main]
async fn main() {
    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("Listening on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
