use crate::routes::root::root_get;
use axum::routing::{get, post};
use axum::Router;

use self::root::root_post;
pub mod root;

pub fn app() -> Router {
    Router::new()
        .route("/", get(root_get))
        .route("/", post(root_post))
}
