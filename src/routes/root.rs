use crate::routes::error::Result;
use axum::extract::State;

use axum::routing::{get};
use axum::Json;
use axum::Router;


use super::AppState;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(root_get))
}

// TODO: Do we add wildcards? Are subdomains obviated by TLDs?
pub async fn root_get(State(state): State<AppState>) -> Result<Json<Vec<String>>> {
    Ok(Json::from(state.api_domains))
}
