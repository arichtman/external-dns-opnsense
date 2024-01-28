use crate::routes::root::root_get;
use axum::routing::get;
use axum::Router;
pub mod root;
pub fn app() -> Router {
    Router::new().route("/", get(root_get))
}
