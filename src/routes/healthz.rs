use super::error::Result;
use super::AppState;
use axum::extract::State;

use axum::routing::get;
use axum::Router;
use log::debug;
use reqwest::StatusCode;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(healthz_get))
}
// TODO: Look into either implementing From between reqwest errors and our custom ones
// Possible to bubble up errors like this state.api_client.get("").await.unwrap()?
// TODO: Look into returning more information on failure.
// Presently only able to return 500 no body
// TODO: Think about tuple matching or something fancier than nested match
pub async fn healthz_get(State(state): State<AppState>) -> Result<String> {
    // TODO: Not sure about this unwrap
    let response = state.api_client.get("get").await.unwrap();
    debug!("{response:?}");
    match response.error_for_status() {
        Ok(_) => Ok("Genki".into()),
        Err(e) => match e.status() {
            Some(StatusCode::UNAUTHORIZED) => Err(super::error::Error::LoginFail),
            _ => Err(super::error::Error::GenericFail),
        },
    }
}
