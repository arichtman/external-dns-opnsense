use std::collections::HashMap;

use crate::appstate::DynStateTrait;
use crate::data_structs::{Changes, EDNSEndpoints};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use log::{debug, info};
use serde::Deserialize;
use serde_json::{to_value, Value};

pub fn app() -> Router<DynStateTrait> {
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

#[debug_handler(state = DynStateTrait)]
pub async fn records_get(
    // headers: HeaderMap,
    State(state): State<DynStateTrait>,
) -> impl IntoResponse {
    // TODO: Work out how to match requested content-type. Middleware would be nice
    let override_list = state.get_all_host_overrides().await;
    // Bail out early if error
    if override_list.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            // TODO: Should we translate or modify this message?
            // TODO: This smells. Double-json function calls and unwraps all over?
            Json::from(serde_json::to_value(override_list.unwrap_err()).unwrap()),
        );
    }
    let override_list = override_list.unwrap();
    if override_list.is_empty() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json::from(
                serde_json::to_value(
                    "Unable to locate total record count, aborting...".to_string(),
                )
                .unwrap(),
            ),
        );
    }
    let managed_domains = state.get_domains();
    let managed_overrides: Vec<_> = override_list
        .into_iter()
        .filter(|x| state.get_domains().contains(&x.domain))
        .collect();
    let ol: EDNSEndpoints = managed_overrides.into();
    debug!("{ol:?}");
    (StatusCode::OK, Json(serde_json::to_value(&ol).unwrap()))
}

#[debug_handler(state = DynStateTrait)]
pub async fn records_post(
    State(_state): State<DynStateTrait>,
    Json(_body): Json<Changes>,
) -> impl IntoResponse {
    // TODO: Should we put any response body?
    // Need to return 204 on success, according to the docs
    (StatusCode::NO_CONTENT, Json("Accepted"))
    // StatusCode::NO_CONTENT
}
