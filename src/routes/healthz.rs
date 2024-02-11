use axum::extract::State;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Router};
use log::debug;

use crate::appstate::DynStateTrait;

pub fn app() -> Router<DynStateTrait> {
    Router::new().route("/", get(healthz_get))
}
// TODO: Look into either implementing From between reqwest errors and our custom ones
// Possible to bubble up errors like this state.api_client.get("").await.unwrap()?
// TODO: Look into returning more information on failure.
// Presently only able to return 500 no body
// TODO: Think about tuple matching or something fancier than nested match
#[debug_handler(state = DynStateTrait)]
pub async fn healthz_get(State(state): State<DynStateTrait>) -> impl IntoResponse {
    // TODO: There feels like a cleverer way to simply return the outcome of the api client call
    //   but the ? doesn't seem to play nice with an async function cause it returns unit
    //   and if we don't put any other handling then ? on the golden path returns a reqwest::Response, which doesn't impl IntoResponse
    // state.api_client.get("get").await?
    // TODO: Not sure about this unwrap
    let response = state.api_get("get").await;
    debug!("{response:?}");
    match response {
        Ok(_) => (StatusCode::OK, "Genki".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
