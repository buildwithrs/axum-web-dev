use axum::{Json, Router, routing::get};
use serde::Serialize;

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
}

async fn root() -> &'static str {
    "axum-web-dev"
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
    version: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        version: domain::VERSION,
    })
}
