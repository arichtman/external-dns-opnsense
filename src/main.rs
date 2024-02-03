// TODO: Remove for production
#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use std::sync::Arc;

use crate::data_structs::Endpoints;
use clap::{arg, command, Parser};

use serde_json::{json, Value};
use tokio::net::TcpListener;
mod data_structs;
use axum_otel_metrics::HttpMetricsLayerBuilder;
use log::debug;

// TODO: Update env use when issue is resolved https://github.com/clap-rs/clap/issues/3221
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, )]
struct Cli {
    #[arg(short, long, env = "EDNS_KEY", default_value = "")]
    key: String,
    #[arg(short = 's', long, env = "EDNS_SECRET", default_value = "")]
    secret: String,
    #[arg(short = 'u', long, env = "EDNS_FQDN", default_value = "")]
    fqdn: String,
    #[arg(short = 'd', long, action = clap::ArgAction::Append, env = "EDNS_DOMAIN", long_help = "May be specified multiple times.", default_values_t = vec!(String::from("local")))]
    domain: Vec<String>,
    /// Increments logging verbosity.
    #[arg(short, long, action = clap::ArgAction::Count, env = "EDNS_VERBOSE", long_help = "Optional. May be applied up to 4 times. Environment variable takes integer.")]
    verbose: u8,
}

// TODO: I _think_ we want ownership here cause we don't want lifetime issues when other stuff drops out of scope
// TODO: Can/should we make this private?
#[derive(Clone, Default, Debug)]
pub struct AppState {
    api_client: OPNsenseClient,
    api_domains: Vec<String>,
    endpoints: Endpoints,
}

// TODO: Look at moving the URL parsing maybe earlier in the setup?
// api_url could be Url type but Default isn't implemented for reqwest::Url
#[derive(Clone, Default, Debug)]
struct OPNsenseClient {
    client: reqwest::Client,
    key: String,
    secret: String,
    url: String,
}

// TODO: We _could_ enumerate the REST resources, but honestly it's easier as a String
// TODO: This is getting a bit big, either shift it to a module or break it up
//   On that note, I think another abstraction that holds the "business logic" of our
//   API transactions makes some sense, keep the client very plain
impl OPNsenseClient {
    fn new(key: String, secret: String, fqdn: String) -> OPNsenseClient {
        OPNsenseClient {
            client: reqwest::ClientBuilder::new().build().unwrap(),
            key,
            secret,
            url: format!("https://{fqdn}"),
            ..Default::default()
        }
    }
    fn settings_url(&self, resource: &str) -> String {
        format!("settings/{resource}")
    }
    async fn get(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.act(reqwest::Method::GET, &self.settings_url(resource), None)
            .await
    }
    async fn post(
        &self,
        resource: &str,
        body: Option<Value>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.act(reqwest::Method::POST, &self.settings_url(resource), body)
            .await
    }
    async fn get_raw(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.act(reqwest::Method::GET, resource, None).await
    }
    async fn post_raw(
        &self,
        resource: &str,
        body: Option<Value>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.act(reqwest::Method::POST, resource, body).await
    }
    async fn act(
        &self,
        method: reqwest::Method,
        resource: &str,
        body: Option<Value>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let req_builder = self
            .client
            .request(method, format!("{0}/api/unbound/{1}", self.url, resource));
        let req_builder = req_builder.basic_auth(&self.key, Some(&self.secret));
        let req = match body {
            // TODO: This is a bit convoluted but I'd prefer to take in serde_json::Value over std::String
            Some(s) => req_builder.body(serde_json::to_string(&s).unwrap()),
            None => req_builder,
        }
        .build()
        .unwrap();
        debug!("{req:?}");
        self.client.execute(req).await
    }
    async fn apply_changes(&self) -> Result<reqwest::Response, reqwest::Error> {
        self.post_raw("service/reconfigure", None).await
    }
    async fn get_all_host_overrides(&self) -> Result<reqwest::Response, reqwest::Error> {
        let body = json!({"current":1,"rowCount":-1,"sort":{"hostname":"asc"},"searchPhrase":""});
        self.post("searchHostOverride", Some(body)).await
    }
}

mod routes;
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    // TODO: Is this the idiomatic way to handle it?
    let log_level = match cli.verbose {
        0 => log::Level::Error,
        1 => log::Level::Warn,
        2 => log::Level::Info,
        3 => log::Level::Debug,
        4.. => log::Level::Trace,
    };
    simple_logger::init_with_level(log_level).expect("Error initialising logging, aborting.");
    // TODO: Learn best logging practices.
    // Specifically: The debug here redundifies the info level and should we use "{:?}" or "{:#?}"
    // How to let users configure it in the simplest way
    debug!("{:?}", cli);
    let client = OPNsenseClient::new(cli.key, cli.secret, cli.fqdn);
    debug!("{client:#?}");
    let state = Arc::new(AppState {
        api_client: client,
        api_domains: cli.domain,
        ..Default::default()
    });
    debug!("{:?}", state);
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

    // TODO: Work out a shared app object
    // TODO: Look into mocking the OPNsense API or stubbing functions
    // TODO: Begin adding tests for the other calls
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
