use std::str::ParseBoolError;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use log::{debug, info};
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

pub async fn records_get(State(state): State<AppState>) -> impl IntoResponse {
    let result = state.api_client.get_all_host_overrides().await;
    // Bail out early if error
    // let result: Result<_, &str> = Err::<u32, &str>("foobies");
    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            // TODO: Should we translate or modify this message?
            result.unwrap_err().to_string(),
        );
    }
    let returned_response = Json::from(result.unwrap().json::<Value>().await.unwrap());
    debug!("{returned_response:?}");
    // TODO: revisit the if-statements here and see about nicer pattern matching
    //  though I'm not sure you're supposed to introduce side effects in matches
    let total_records = returned_response.get("total");
    if total_records.is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to locate total record count, aborting...".to_string(),
        );
    }
    let total_records = total_records.unwrap();
    info!("Found {total_records} total host overrides, filtering to domain list...");
    let override_list = returned_response.get("rows");
    if override_list.is_none() {
        return (
            StatusCode::INSUFFICIENT_STORAGE,
            "Unable to locate records in response, aborting...".to_string(),
        );
    }
    // let override_list = override_list.unwrap();
    let override_list = returned_response["rows"].as_array().unwrap();
    // override_list.iter().map(|x| x.get("uuid".into())).collect();
    debug!("{override_list:?}");
    let ol: Vec<_> = override_list
        .into_iter()
        .map(|x| x.get::<&str>("uuid".into()))
        .collect();
    debug!("{ol:?}");
    todo!()
}

pub async fn records_post(State(state): State<AppState>, body: Json<Value>) -> impl IntoResponse {
    // Need to return 204 on success
    (StatusCode::NO_CONTENT, "accepted".to_string())
}
