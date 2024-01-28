use super::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(healthz_get))
}
pub async fn healthz_get(State(state): State<AppState>) -> impl IntoResponse {
    "Genki"
}
