use self::healthz::healthz_get;
use axum::routing::{get, post};
use axum::Router;

pub mod healthz;
pub mod root;

pub fn app() -> Router {
    Router::new().merge(root::app()).merge(healthz::app())
}
