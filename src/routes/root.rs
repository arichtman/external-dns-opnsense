use crate::routes::error::Result;
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

// TODO: Allow runtime configuration of domains
// TODO: Do we add wildcards? Are subdomains obviated by TLDs?
pub async fn root_get(State(state): State<AppState>) -> Result<Json<Vec<String>>> {
    Ok(Json::from(state.api_domains))
    // Ok(Json::from(
    //     json!({"filters": [ "local", "cluster.local", "svc.cluster.local"]}),
    // ))
}
