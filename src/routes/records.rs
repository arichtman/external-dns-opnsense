use std::sync::Arc;

use crate::data_structs::{Changes, Endpoints};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use log::{debug, info};
use serde::Deserialize;
use serde_json::Value;

use super::AppState;

pub fn app() -> Router<Arc<AppState>> {
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

#[debug_handler(state = Arc<AppState>)]
pub async fn records_get(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    debug!("{:#?}", headers);
    headers.contains_key("Content-Type");
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
    debug!("Initial get: {override_list:#?}");
    if override_list.is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to locate records in response, aborting...".to_string(),
        );
    }
    // TODO: Domain check is passing, find out why the filter isn't
    debug!("api domains: {:?}", state.api_domains);
    debug!(
        "domain check: {:?}",
        state.api_domains.contains(&"com".to_string())
    );
    // TODO: do we need to grab this twice? Does it matter since there's no additional API call?
    debug!("{:#?}", returned_response["rows"]);
    let override_list: Vec<&Value> = returned_response["rows"]
        .as_array()
        .unwrap()
        .into_iter()
        .filter(|x| {
            // TODO: This quotation replace is jank. Should be happening much earlier, ideally in Clap parsing or config construction
            // debug!(
            //     "{:#?}",
            //     &x.get("domain").unwrap().to_string().replace('"', "")
            // );
            state
                .api_domains
                .contains(&x.get("domain").unwrap().to_string().replace('"', ""))
        })
        .collect();
    let ol: Endpoints = override_list.into();
    (StatusCode::OK, serde_json::to_string(&ol).unwrap())
}

#[debug_handler(state = Arc<AppState>)]
pub async fn records_post(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Changes>,
) -> impl IntoResponse {
    // TODO: Should we put any response body?
    // Need to return 204 on success, according to the docs
    (StatusCode::NO_CONTENT, Json("Accepted"))
    // StatusCode::NO_CONTENT
}
