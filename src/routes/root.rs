use axum::response::{ErrorResponse, IntoResponse};
use axum::Json;
use serde_json::{json, Value};

pub async fn root_get() -> impl IntoResponse {
    "root get!"
}

struct RootBody {
    // TODO: Work out what the request needs
    thing1: String,
    thing2: Option<u32>,
}

// TODO: That Err still returns HTTP code 200
pub async fn root_post(body: Json<Value>) -> axum::response::Result<Json<Value>, String> {
    // Ok(Json::from(json!({"key": 5})))
    match body {
        _ => Err("foo".into()),
    }
}
