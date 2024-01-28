use self::healthz::healthz_get;
use axum::routing::{get, post};
use axum::Router;

pub mod healthz;
pub mod root;
pub mod webhook;

#[derive(Clone)]
pub struct AppState {
    api_key_id: String,
    api_key_secret: String,
    api_url: String,
}

pub fn app(api_key_id: String, api_key_secret: String, api_url: String) -> Router {
    let state = AppState {
        api_key_id,
        api_key_secret,
        api_url,
    };
    Router::new()
        .merge(root::app())
        .merge(healthz::app())
        .with_state(state)
}
