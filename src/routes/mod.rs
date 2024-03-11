use axum::Router;

use crate::appstate::DynStateTrait;
use axum::http::header;

mod adjustendpoints;
mod healthz;
mod records;
mod root;

const REPLY_HEADERS: [(axum::http::HeaderName, &str); 1] = [(
    header::CONTENT_TYPE,
    // Q: Should this match the request or will it always be this?
    "application/external.dns.webhook+json;version=1",
)];

pub fn app(state: DynStateTrait) -> Router {
    // Note: it feels weird nesting them here but it's marginally less boilerplatey
    Router::new()
        .merge(root::app())
        .nest("/healthz", healthz::app())
        .nest("/adjustendpoints", adjustendpoints::app())
        .nest("/records", records::app())
        .with_state(state)
}
