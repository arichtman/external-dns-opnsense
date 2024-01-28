use self::healthz::healthz_get;
use self::root::{root_get, root_post};
use axum::routing::{get, post};
use axum::Router;

pub mod healthz;
pub mod root;

pub fn app() -> Router {
    Router::new()
        .route("/", get(root_get))
        .route("/", post(root_post))
        .route("/healthz", get(healthz_get))
}
