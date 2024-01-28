// TODO: Remove for production
#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use crate::data_structs::Endpoints;
use clap::{arg, command, Parser};

use log::{debug, error, info, trace, warn};

// TODO: Update env use when issue is resolved https://github.com/clap-rs/clap/issues/3221
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, )]
struct Cli {
    #[arg(short = 'i', long, env = "EDNS_API_KEY_ID", default_value = "")]
    api_key_id: String,
    #[arg(short = 's', long, env = "EDNS_API_KEY_SECRET", default_value = "")]
    api_key_secret: String,
    #[arg(short = 'u', long, env = "EDNS_API_URL", default_value = "")]
    api_url: String,
    #[arg(short = 'd', long, action = clap::ArgAction::Append, env = "EDNS_API_DOMAIN", long_help = "May be specified multiple times.", default_values_t = vec!(String::from("local")))]
    api_domain: Vec<String>,
    /// Increments logging verbosity.
    #[arg(short, long, action = clap::ArgAction::Count, env = "EDNS_VERBOSE", long_help = "Optional. May be applied up to 4 times. Environment variable takes integer.")]
    verbose: u8,
}

use tokio::net::TcpListener;

pub mod data_structs;
// TODO: I _think_ we want ownership here cause we don't want lifetime issues when other stuff drops out of scope
#[derive(Clone, Default, Debug)]
pub struct AppState {
    api_key_id: String,
    api_key_secret: String,
    api_url: String,
    api_domains: Vec<String>,
    endpoints: Option<Endpoints>,
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
    debug!("{:?}", cli);
    let state = AppState {
        api_key_id: cli.api_key_id,
        api_key_secret: cli.api_key_secret,
        api_url: cli.api_url,
        api_domains: cli.api_domain,
        ..Default::default()
    };
    let listener = TcpListener::bind("[::]:8888").await.unwrap();
    axum::serve(listener, routes::app(state).into_make_service())
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
        let app = app(AppState {
            ..Default::default()
        });
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
