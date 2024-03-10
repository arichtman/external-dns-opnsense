use crate::appstate::DynStateTrait;
use crate::data_structs::{Changes, Endpoints};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use log::{debug, info};
use serde::Deserialize;
use serde_json::Value;

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
    let result = state.get_all_host_overrides().await;
    // Bail out early if error
    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            // TODO: Should we translate or modify this message?
            // TODO: This smells. Double-json function calls and unwraps all over?
            Json::from(serde_json::to_value(result.unwrap_err().to_string()).unwrap()),
        );
    }
    let returned_response = Json::from(result.unwrap().json::<Value>().await.unwrap());
    debug!("{returned_response:?}");
    let total_records = returned_response.get("total");
    if total_records.is_none() {
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
    let total_records = total_records.unwrap();
    info!("Found {total_records} total host overrides, filtering to domain list...");
    let override_list = returned_response.get("rows");
    debug!("Initial get: {override_list:#?}");
    if override_list.is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json::from(
                serde_json::to_value(
                    "Unable to locate records in response, aborting...".to_string(),
                )
                .unwrap(),
            ),
        );
    }
    debug!("{:#?}", returned_response["rows"]);
    // TODO: do we need to grab this twice? Does it matter since there's no additional API call?
    let override_set = returned_response["rows"].as_array().unwrap();
    let controlled_domains = state.get_domains();
    // use crate::data_structs::Endpoint;
    // let endpoint_set: Vec<_> = override_set
    //     .into_iter()
    //     .filter_map(|x| {
    //         let normalized_domain_name = &x.get("domain").unwrap().to_string().replace('"', "");
    //         match controlled_domains.contains(normalized_domain_name) {
    //             true => Some(normalized_domain_name),
    //             false => None,
    //         }
    //     })
    //     .collect();
    // let ol: Endpoints = endpoint_set.into();
    let override_list: Vec<&Value> = returned_response["rows"]
        .as_array()
        .unwrap()
        .into_iter()
        .filter(|x| {
            let raw_domain_name = &x.get("domain").unwrap().to_string().replace('"', "");
            // TODO: This quotation replace is jank. Should be happening much earlier, ideally in Clap parsing or config construction
            state.get_domains().contains(raw_domain_name)
        })
        .collect();
    let ol: Endpoints = override_list.into();
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
