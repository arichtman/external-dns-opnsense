use super::error::Result;
use super::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use log::debug;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(healthz_get))
}
// TODO: Look into either implementing From between reqwest errors and our custom ones
// Possible to bubble up errors like this state.api_client.get("").await.unwrap()?
pub async fn healthz_get(State(state): State<AppState>) -> Result<String> {
    let response = state.api_client.get("").await;
    debug!("{response:?}");
    match response {
        Ok(_) => Ok("Genki".into()),
        Err(_) => Err(super::error::Error::LoginFail),
    }
}
