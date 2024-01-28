use axum::extract::State;
use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde_json::{json, Value};

use super::AppState;

pub fn app() -> Router<AppState> {
    Router::new()
        .route("/", get(root_get))
        .route("/", post(root_post))
    // .with_state::<AppState>(())
    // .with_state(AppState)
}

pub async fn root_get(State(state): State<AppState>) -> impl IntoResponse {
    format!("root get!\n{0}", state.api_key_id)
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
