use crate::appstate::DynStateTrait;
use crate::data_structs::{Changes, Endpoints};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use log::{debug, info};
use serde::Deserialize;
use serde_json::{Value};

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
    headers: HeaderMap,
    State(state): State<DynStateTrait>,
) -> impl IntoResponse {
    debug!("{:#?}", headers);
    headers.contains_key("Content-Type");
    // TODO: Work out how to match requested content-type. Middleware would be nice
    // fn match_content_type(resp: impl Into<String>) {
    //     match headers.get("Content-Type") {
    //         None => resp,
    //         Some("application/json") => Json(resp),
    //         Some("text/plain") => resp,
    //         _ => resp,
    //     }
    // }
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
    // TODO: revisit the if-statements here and see about nicer pattern matching
    //  though I'm not sure you're supposed to introduce side effects in matches
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
    let override_list: Vec<&Value> = returned_response["rows"]
        .as_array()
        .unwrap()
        .into_iter()
        .filter(|x| {
            // TODO: This quotation replace is jank. Should be happening much earlier, ideally in Clap parsing or config construction
            state
                .get_domains()
                .contains(&x.get("domain").unwrap().to_string().replace('"', ""))
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
