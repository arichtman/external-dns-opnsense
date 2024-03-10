use axum::extract::State;

use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};

use crate::appstate::DynStateTrait;

pub fn app() -> Router<DynStateTrait> {
    Router::new().route("/", get(root_get))
}

// TODO: Do we add wildcards? Are subdomains obviated by TLDs?
#[debug_handler(state = DynStateTrait)]
pub async fn root_get(State(state): State<DynStateTrait>) -> impl IntoResponse {
    // TODO: think about the arc and whether static stuff like domains list should be arc
    Json::from(state.get_domains())
}
