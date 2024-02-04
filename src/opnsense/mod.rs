use log::debug;

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

// TODO: We _could_ enumerate the REST resources, but honestly it's easier as a String
// TODO: This is getting a bit big, either shift it to a module or break it up
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
    pub async fn get_all_host_overrides(&self) -> Result<reqwest::Response, reqwest::Error> {
        let body = serde_json::json!({"current":1,"rowCount":-1,"sort":{"hostname":"asc"},"searchPhrase":""});
        self.post("searchHostOverride", Some(body)).await
    }
}
