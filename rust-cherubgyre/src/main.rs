use axum::{Router, Server};
use dotenv::dotenv;
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    let app = Router::new()
        .merge(routes::create_health_router());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
