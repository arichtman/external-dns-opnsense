use axum::routing::{get, post};
use axum::Router;

use self::webhook::Endpoints;

pub mod adjustendpoints;
mod error;
pub mod healthz;
pub mod records;
pub mod root;
pub mod webhook;

// TODO: I _think_ we want ownership here cause we don't want lifetime issues when other stuff drops out of scope
#[derive(Clone, Default, Debug)]
pub struct AppState {
    api_key_id: String,
    api_key_secret: String,
    api_url: String,
    api_domains: Vec<String>,
    endpoints: Option<Endpoints>,
}

pub fn app(
    api_key_id: String,
    api_key_secret: String,
    api_url: String,
    api_domains: Vec<String>,
) -> Router {
    let state = AppState {
        api_key_id,
        api_key_secret,
        api_url,
        api_domains,
        ..Default::default()
    };
    // TODO: it feels weird nesting them here but it's marginally less boilerplatey
    Router::new()
        .merge(root::app())
        .nest("/healthz", healthz::app())
        .nest("/adjustendpoints", adjustendpoints::app())
        .nest("/records", records::app())
        .with_state(state)
}
