use axum::extract::State;
use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde_json::{json, Value};

use super::AppState;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(records_get).post(records_post))
}

pub async fn records_get(State(state): State<AppState>) -> impl IntoResponse {}

pub async fn records_post(
    State(state): State<AppState>,
    body: Json<Value>,
) -> Result<Json<Value>, String> {
    // Need to return 204 on success
    Ok(Json::from(json!({"key": 5})))
}
