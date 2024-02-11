use std::sync::Arc;

use axum::async_trait;
use log::debug;

// TODO: Why is Endpoint under data_structs but EndpointS isn't...
// I think it's to do with where things are public or exported
use crate::cli::Cli;
use crate::data_structs::Endpoint;
use crate::errors::InternalDataError;
use crate::opnsense::OPNsenseClient;
use crate::Endpoints;
#[cfg(test)]
use mockall::automock;

pub type AppState = Arc<State>;
// TODO: I _think_ we want ownership here cause we don't want lifetime issues when other stuff drops out of scope
// TODO: Can/should we make this private?
#[derive(Clone, Default, Debug)]
pub struct State {
    pub api_client: OPNsenseClient,
    pub api_domains: Vec<String>,
    pub endpoints: Endpoints,
}

pub fn build(cli: Cli) -> DynStateTrait {
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
    // How to let users configure it in the simplest way, I've seen some rely on RUST_LOG
    debug!("{:?}", cli);
    let client = OPNsenseClient::new(cli.key, cli.secret, cli.fqdn);
    debug!("{client:#?}");
    Arc::new(State {
        api_client: client,
        api_domains: cli.domain.into_iter().map(|d| d.replace('"', "")).collect(),
        ..Default::default()
    }) as DynStateTrait
}
// TODO: Is wrapping all the client logic really the right way?
// Perhaps we should pull it out of the OPNsense client.
// But then it's not as cohesive and nice... It doesn't make sense for the state trait to have that interface at all
// Should we just give a generic get_client() and use that?
// Use a trait to decouple data access layer
#[cfg_attr(test, automock)]
#[async_trait]
pub trait StateTrait {
    fn get_by_name(&self, name: &str) -> Result<Endpoint, InternalDataError>;
    fn get_by_address(&self, address: &str) -> Result<Endpoint, InternalDataError>;
    async fn api_get(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error>;
    // fn get_domains(&self) -> Vec<String>;
    fn get_domains(&self) -> &Vec<String>;
    async fn get_all_host_overrides(&self) -> Result<reqwest::Response, reqwest::Error>;
}

#[async_trait]
impl StateTrait for State {
    fn get_by_name(&self, _name: &str) -> Result<Endpoint, InternalDataError> {
        todo!()
    }
    fn get_by_address(&self, _address: &str) -> Result<Endpoint, InternalDataError> {
        todo!()
    }
    async fn api_get(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.api_client.get(resource).await
    }
    fn get_domains(&self) -> &Vec<String> {
        &self.api_domains
    }
    async fn get_all_host_overrides(&self) -> Result<reqwest::Response, reqwest::Error> {
        self.api_client.get_all_host_overrides().await
    }
}

// Trait object. idk, it's cursed arcane incantation
pub type DynStateTrait = Arc<dyn StateTrait + Send + Sync>;
