use axum::extract::State;
use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde_json::{json, Value};

use super::AppState;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(root_get))
}

pub async fn root_get(State(state): State<AppState>) -> impl IntoResponse {
    // Should be {"filters": [ "www.foo.com", "bing.com"]}
    format!("root get!\n{0}", state.api_key_id)
}
