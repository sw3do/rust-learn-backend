mod routes;

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    let port = std::env::var("PORT").expect("PORT must be set in .env file");

    let app = routes::get_app().layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_owned() + &port)
        .await
        .expect("Failed to bind to port");

    println!("Listening on port {}", port);
    axum::serve(listener, app)
        .await
        .expect("Failed to start server")
}
