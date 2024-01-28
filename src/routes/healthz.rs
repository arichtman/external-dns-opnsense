use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use super::AppState;

pub fn app() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(healthz_get))
}
pub async fn healthz_get() -> impl IntoResponse {
    "Genki"
}
