use axum::extract::State;

use axum::routing::post;
use axum::{Json, Router};
use serde_json::Value;

use super::AppState;

pub fn app() -> Router<AppState> {
    Router::new().route("/", post(adjustendpoints_post))
}

// TODO: Work out how to return error codes
// TODO: Understand why he's wrapped core::result::Result to remove the second Type
// It looks like a partial apply to inject our custom Error enum
// Which could be really good in terms of OPNsense error categories, but seems overkill for now?
// Ref: https://github.com/jeremychone-channel/rust-axum-course/blob/4c9ac43bf5c220d79994be18b637089f5ffbf5dd/src/error.rs#L5
pub async fn adjustendpoints_post(
    State(_state): State<AppState>,
    _body: Json<Value>,
) -> Result<Json<Value>, String> {
    // Need to return 200 on success
    // Ok(Json::from(json!({"500": "bzzzzt"})))
    Err("500".into())
}
