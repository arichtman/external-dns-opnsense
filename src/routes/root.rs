use axum::extract::State;

use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use serde::Serialize;

use crate::appstate::DynStateTrait;

use super::REPLY_HEADERS;

pub fn app() -> Router<DynStateTrait> {
    Router::new().route("/", get(root_get))
}

// TODO: Do we add wildcards? Are subdomains obviated by TLDs?
#[debug_handler(state = DynStateTrait)]
pub async fn root_get(State(state): State<DynStateTrait>) -> impl IntoResponse {
    // TODO: Should the struct be in the function, outside, or another file?
    #[derive(Serialize)]
    struct DomainFilter {
        filters: Vec<String>,
    }
    let reply = DomainFilter {
        filters: state.get_domains(),
    };
    // TODO: think about the arc and whether static stuff like domains list should be arc
    (REPLY_HEADERS, Json::from(reply))
}
