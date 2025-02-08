use axum::{response::IntoResponse, routing::get, Router};

pub async fn health_check() -> impl IntoResponse {
    (axum::http::StatusCode::OK, "Healthy")
}

pub fn create_health_router() -> Router {
    Router::new().route("/health", get(health_check))
}
