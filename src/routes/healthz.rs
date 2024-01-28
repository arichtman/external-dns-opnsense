use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(healthz_get))
}
pub async fn healthz_get() -> impl IntoResponse {
    "Genki"
}
