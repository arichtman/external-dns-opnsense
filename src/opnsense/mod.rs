use std::net::{IpAddr, Ipv6Addr};

use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::data_structs::OPNSenseEndpoint;
use crate::errors::OPNSenseError;
use crate::Value;
// TODO: Look at moving the URL parsing maybe earlier in the setup?
// api_url could be Url type but Default isn't implemented for reqwest::Url
#[derive(Clone, Default, Debug)]
pub struct OPNsenseClient {
    client: reqwest::Client,
    key: String,
    secret: String,
    url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OPNSenseRecordType {
    A(IpAddr),
    AAAA(Ipv6Addr),
}

// This feels weird, if we implement this it's going to be too generic an input on the signature
// So I'm thinking we keep the transformation logic local
// impl From<&str> for OPNSenseRecordType {
//     fn from(input: &str) -> Self {

//     }
// }

// TODO: We _could_ enumerate the REST resources, but honestly it's easier as a String
// TODO: This is getting a bit big, not sure how to break it up
//   On that note, I think another abstraction that holds the "business logic" of our
//   API transactions makes some sense, keep the client very plain
impl OPNsenseClient {
    pub fn new(key: String, secret: String, fqdn: String) -> OPNsenseClient {
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
    pub async fn get(&self, resource: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.act(reqwest::Method::GET, &self.settings_url(resource), None)
            .await
    }
    pub async fn post(
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
        // TODO: Not sure if this is the right approach, but changing the function signature to just Response doesn't capture that it can fail.
        //   Maybe it's an async thing?
        Ok(self.client.execute(req).await?)
    }
    pub async fn apply_changes(&self) -> Result<reqwest::Response, reqwest::Error> {
        self.post_raw("service/reconfigure", None).await
    }
    pub async fn get_all_host_overrides(&self) -> Result<Vec<OPNSenseEndpoint>, OPNSenseError> {
        let body = serde_json::json!({"current":1,"rowCount":-1,"sort":{"hostname":"asc"},"searchPhrase":""});
        let response = self.post("searchHostOverride", Some(body)).await;
        if response.is_err() {
            return Err(OPNSenseError::GenericFailure);
        }
        let records = response
            .expect("Error branch handled prior")
            .json::<Map<_, _>>()
            .await;
        if records.is_err() {
            return Err(OPNSenseError::UnconvertibleData);
        };
        let records = records.expect("Error branch handled before)");
        let records = records.get("rows");
        if records.is_none() {
            return Err(OPNSenseError::NoData);
        }
        let records = records.expect("Error branch handled prior");
        debug!("{records:?}");
        let result: Vec<OPNSenseEndpoint> = records
            .as_array()
            .expect("Unable to retrieve returned results as an array")
            .to_owned()
            .into_iter()
            .map(|x| x.into())
            .collect();
        debug!("{result:?}");
        Ok(result)
    }
}
