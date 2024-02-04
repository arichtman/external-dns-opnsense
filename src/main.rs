// TODO: Remove for production
#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use std::sync::Arc;

use crate::data_structs::Endpoints;

use serde_json::{json, Value};
use tokio::net::TcpListener;
mod data_structs;
use axum_otel_metrics::HttpMetricsLayerBuilder;
use log::debug;

mod cli;
use crate::cli::Cli;
mod appstate;

mod opnsense;
mod routes;
#[tokio::main]
async fn main() {
    // TODO: I'm not sure about how we've separated cli and appstate building, mostly by the amount of imports they all have to do which feels like a lot of coupling/shared knowledge?
    let state = crate::appstate::build(cli::get());
    debug!("{:#?}", state);
    let listener = TcpListener::bind("[::]:8888").await.unwrap();
    let metrics = HttpMetricsLayerBuilder::new()
        .with_service_name(env!("CARGO_PKG_NAME").into())
        .with_service_version(env!("CARGO_PKG_VERSION").into())
        .with_prefix(env!("CARGO_PKG_NAME").into())
        .build();
    axum::serve(
        listener,
        routes::app(state)
            .merge(metrics.routes())
            .layer(metrics)
            .into_make_service(),
    )
    .await
    .unwrap();
}

// Ref: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::app;
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn get_root() {
        let app = app(Arc::new(AppState {
            ..Default::default()
        }));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
