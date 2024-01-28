use axum::routing::{get, post};
use axum::Router;

use crate::AppState;

pub mod adjustendpoints;
mod error;
pub mod healthz;
pub mod records;
pub mod root;

pub fn app(state: AppState) -> Router {
    // TODO: it feels weird nesting them here but it's marginally less boilerplatey
    Router::new()
        .merge(root::app())
        .nest("/healthz", healthz::app())
        .nest("/adjustendpoints", adjustendpoints::app())
        .nest("/records", records::app())
        .with_state(state)
}
