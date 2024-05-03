use axum::extract::State;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Router};
use tracing::debug;

use crate::appstate::DynStateTrait;

pub fn app() -> Router<DynStateTrait> {
    Router::new().route("/", get(healthz_get))
}
// Q: Possible to bubble up errors like this state.api_client.get("").await.unwrap()?
#[debug_handler(state = DynStateTrait)]
pub async fn healthz_get(State(state): State<DynStateTrait>) -> impl IntoResponse {
    let response = state.api_get("get").await;
    debug!("Health check response: {response:?}");
    // TODO: Look into returning more information on failure by returning custom errors
    match response {
        Ok(_) => (StatusCode::OK, "Genki".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
