use axum::routing::{get, post};
use axum::Router;

pub mod adjustendpoints;
mod error;
pub mod healthz;
pub mod records;
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
        .nest("/adjustendpoints", adjustendpoints::app())
        .nest("/records", records::app())
        .with_state(state)
}
