use std::str::ParseBoolError;

use axum::extract::State;
use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde::Deserialize;
use serde_json::{json, Value};

// use super::error::Result;
// use super::error::Error;
use super::AppState;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(records_get).post(records_post))
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct HostOverrides {
    rows: Vec<HostOverride>,
    rowCount: u64,
    total: u64,
    current: u64,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct HostOverride {
    uuid: String,
    enabled: String,
    hostname: String,
    domain: String,
    rr: String,
    mxprio: String,
    server: String,
    description: String,
}

pub async fn records_get(State(state): State<AppState>) -> Result<Json<Value>, String> {
    let result = state.api_client.get_all_host_overrides().await;
    match result {
        Ok(response) => Ok(Json::from(response.json::<Value>().await.unwrap())),
        Err(e) => Err(e.to_string()),
    }
}

// TODO: Find out why using naked serde_json::Value for body type breaks
pub async fn records_post(
    State(state): State<AppState>,
    body: Json<Value>,
) -> Result<Json<Value>, String> {
    // Need to return 204 on success
    Ok(Json::from(json!({"key": 5})))
}
