use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde_json::{json, Value};

pub fn app() -> Router {
    Router::new()
        .route("/", get(root_get))
        .route("/", post(root_post))
}

pub async fn root_get() -> impl IntoResponse {
    "root get!"
}

// TODO: Work out what the structure DomainFilter is and how it's presented
struct RootBody {
    thing1: String,
    thing2: Option<u32>,
}

// TODO: That Err still returns HTTP code 200
// TODO: Understand why he's wrapped core::result::Result to remove the second Type
// It looks like a partial apply to inject our custom Error enum
// Which could be really good in terms of OPNsense error categories, but seems overkill for now?
// Ref: https://github.com/jeremychone-channel/rust-axum-course/blob/4c9ac43bf5c220d79994be18b637089f5ffbf5dd/src/error.rs#L5
pub async fn root_post(body: Json<Value>) -> Result<Json<Value>, String> {
    // Ok(Json::from(json!({"key": 5})))
    match body {
        _ => Err("foo".into()),
    }
}
