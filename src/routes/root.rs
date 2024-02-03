use crate::routes::error::Result;
use axum::extract::State;

use axum::routing::get;
use axum::{debug_handler, Json, Router};

use super::AppState;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(root_get))
}

// TODO: Do we add wildcards? Are subdomains obviated by TLDs?
#[debug_handler(state = AppState)]
pub async fn root_get(State(state): State<AppState>) -> Result<Json<Vec<String>>> {
    Ok(Json::from(state.api_domains))
}
