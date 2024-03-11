use axum::extract::State;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use log::debug;

use crate::appstate::DynStateTrait;

pub fn app() -> Router<DynStateTrait> {
    Router::new().route("/", get(healthz_get))
}
// TODO: Look into either implementing From between reqwest errors and our custom ones
// Possible to bubble up errors like this state.api_client.get("").await.unwrap()?
// TODO: Look into returning more information on failure.
#[debug_handler(state = DynStateTrait)]
pub async fn healthz_get(State(state): State<DynStateTrait>) -> impl IntoResponse {
    // TODO: There feels like a cleverer way to simply return the outcome of the api client call
    //   but the ? doesn't seem to play nice with an async function cause it returns unit
    //   and if we don't put any other handling then ? on the golden path returns a reqwest::Response, which doesn't impl IntoResponse
    let response = state.api_get("get").await;
    debug!("Health check response: {response:?}");
    // TODO: This isn't actually putting JSON in the body of the response.
    // TODO: Return custom internal error to be mapped to server one
    match response {
        Ok(_) => (StatusCode::OK, Json("Genki".to_string())),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
    }
}
