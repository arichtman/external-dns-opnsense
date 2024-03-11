use axum::extract::State;

use super::REPLY_HEADERS;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{debug_handler, Json, Router};
use serde_json::Value;

use crate::appstate::DynStateTrait;

pub fn app() -> Router<DynStateTrait> {
    Router::new().route("/", post(adjustendpoints_post))
}

// TODO: Work out how to return error codes
// TODO: Understand why he's wrapped core::result::Result to remove the second Type
// It looks like a partial apply to inject our custom Error enum
// Which could be really good in terms of OPNsense error categories, but seems overkill for now?
// Ref: https://github.com/jeremychone-channel/rust-axum-course/blob/4c9ac43bf5c220d79994be18b637089f5ffbf5dd/src/error.rs#L5
#[debug_handler(state = DynStateTrait)]
pub async fn adjustendpoints_post(
    State(_state): State<DynStateTrait>,
    _body: Json<Value>,
) -> impl IntoResponse {
    todo!();
    (StatusCode::INTERNAL_SERVER_ERROR, REPLY_HEADERS)
}
