use axum::{response::IntoResponse, Json};
use serde_json::{json, Value};

pub async fn root_get() -> impl IntoResponse {
    "root get!"
}

pub async fn root_post(body: Json<Value>) -> axum::response::Result<Json<Value>> {
    Ok(Json::from(json!({"key": 5})))
}
