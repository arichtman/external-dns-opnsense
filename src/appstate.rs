use std::sync::Arc;

use axum::async_trait;
use tracing::debug;

// Q: Why is Endpoint under data_structs but EndpointS isn't...
// I think it's to do with where things are public or exported
use crate::cli::Cli;
use crate::data_structs::{EDNSEndpoint, OPNSenseEndpoint};
use crate::errors::{InternalDataError, OPNSenseError};
use crate::opnsense::OPNsenseClient;
use crate::EDNSEndpoints;
#[cfg(test)]
use mockall::automock;

pub type AppState = Arc<State>;
// Q: I _think_ we want ownership here cause we don't want lifetime issues when other stuff drops out of scope
// Q: Can/should we make this private?
#[derive(Clone, Default, Debug)]
pub struct State {
    pub api_client: OPNsenseClient,
    pub api_domains: Vec<String>,
    pub endpoints: EDNSEndpoints,
}

pub fn build(cli: Cli) -> DynStateTrait {
    let client = OPNsenseClient::new(cli.key, cli.secret, cli.fqdn);
    Arc::new(State {
        api_client: client,
        api_domains: cli.domain.into_iter().map(|d| d.replace('"', "")).collect(),
        ..Default::default()
    }) as DynStateTrait
}
// Q: Is wrapping all the client logic really the right way?
// Perhaps we should pull it out of the OPNsense client.
// But then it's not as cohesive and nice... It doesn't make sense for the state trait to have that interface at all
// Should we just give a generic get_client() and use that?

// Use a trait to decouple data access layer
#[cfg_attr(test, automock)]
#[async_trait]
pub trait StateTrait {
    async fn get_by_name(&self, name: &str) -> Result<EDNSEndpoint, InternalDataError>;
    async fn get_by_address(&self, address: &str) -> Result<EDNSEndpoint, InternalDataError>;
    async fn api_get(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error>;
    fn get_domains(&self) -> Vec<String>;
    // async fn get_domains(&self) -> &Vec<String>;
    async fn get_all_host_overrides(&self) -> Result<Vec<OPNSenseEndpoint>, OPNSenseError>;
}

#[async_trait]
impl StateTrait for State {
    async fn get_by_name(&self, _name: &str) -> Result<EDNSEndpoint, InternalDataError> {
        let api_results = &self.api_client.get_all_host_overrides().await;
        if api_results.is_err() {
            return Err(InternalDataError::GenericError);
        };
        debug!("API results: {api_results:?}");
        todo!()
    }
    async fn get_by_address(&self, _address: &str) -> Result<EDNSEndpoint, InternalDataError> {
        todo!()
    }
    async fn api_get(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.api_client.get(resource).await
    }
    fn get_domains(&self) -> Vec<String> {
        // Q: this smells, we keep one copy of state and the api domains
        //  don't change during runtime and aren't mutated either.
        //  Maybe it's just easier than passing references around everywhere?
        self.api_domains.clone()
    }
    async fn get_all_host_overrides(&self) -> Result<Vec<OPNSenseEndpoint>, OPNSenseError> {
        self.api_client.get_all_host_overrides().await
    }
}

// Trait object. idk, it's cursed arcane incantation
pub type DynStateTrait = Arc<dyn StateTrait + Send + Sync>;
