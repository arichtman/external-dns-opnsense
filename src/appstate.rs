use std::sync::Arc;

use log::debug;

use crate::cli::Cli;
use crate::opnsense::OPNsenseClient;
use crate::Endpoints;
// TODO: I _think_ we want ownership here cause we don't want lifetime issues when other stuff drops out of scope
// TODO: Can/should we make this private?
#[derive(Clone, Default, Debug)]
pub struct AppState {
    pub api_client: OPNsenseClient,
    pub api_domains: Vec<String>,
    pub endpoints: Endpoints,
}

pub fn build(cli: Cli) -> Arc<AppState> {
    // TODO: Is this the idiomatic way to handle it?
    // TODO: Maybe move this into the cli module?
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
    Arc::new(AppState {
        api_client: client,
        api_domains: cli.domain.into_iter().map(|d| d.replace('"', "")).collect(),
        ..Default::default()
    })
}
