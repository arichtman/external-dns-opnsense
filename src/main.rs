// TODO: Remove for production
#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use std::io;
use std::net::Ipv6Addr;

use axum_otel_metrics::HttpMetricsLayerBuilder;
use data_structs::EDNSEndpoints;

use serde_json::Value;
use tokio::net::TcpListener;

use tracing::debug;

// Q: This seems tedious mod-ing everything. Is this correct?
mod appstate;
mod cli;
mod data_structs;
mod errors;
mod opnsense;
mod routes;

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli_opts = cli::get();
    // Initialize telemetry/logging
    let max_log_level = match cli_opts.verbose {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        4.. => tracing::Level::TRACE,
    };
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(max_log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // immediately output some debugging information
    debug!("Options: {cli_opts:?}");
    debug!("Max log level: {max_log_level}");

    let listener = TcpListener::bind((Ipv6Addr::UNSPECIFIED, cli_opts.port)).await?;
    // Q: I'm not sure about how we've separated cli and appstate building, mostly by the amount of imports they all have to do which feels like a lot of coupling/shared knowledge?
    let state = crate::appstate::build(cli_opts);
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
}

// Ref: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use mockall::predicate::*;
    use tower::util::ServiceExt;
    // Q: Should these have prefix crate:: or is this fine?
    // use appstate::AppState;
    use crate::appstate::{DynStateTrait, MockStateTrait};
    use routes::app;
    use rstest::rstest;

    #[rstest]
    #[case("/")]
    // #[case("/healthz")]
    // #[case("/records")]
    #[tokio::test]
    async fn get_requests(#[case] resource: &'static str) {
        let mut appstate_mock = MockStateTrait::new();
        let fixed_domains = vec!["local".to_string()];
        appstate_mock
            .expect_get_domains()
            .return_const(fixed_domains);
        let state = Arc::new(appstate_mock) as DynStateTrait;
        let app = app(state);
        let response = app
            .oneshot(
                Request::builder()
                    .uri(resource)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // let body: Value = serde_json::from_slice(&body).unwrap();
        // assert_eq!(body, json!(&fixed_domains));
    }
}
