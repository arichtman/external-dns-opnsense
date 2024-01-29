#![allow(non_snake_case)]

use std::collections::HashMap;


use serde::{Deserialize, Serialize};
use serde_json::Value;

// TODO: see if we can use the ! whole-file approach
// Probably a bad idea since it'll bloat the code...
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Changes {
    create: Endpoints,
    updateOld: Endpoints,
    updateNew: Endpoints,
    delete: Endpoints,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Endpoints {
    items: Vec<Endpoint>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Endpoint {
    dnsName: String,
    // TODO: Switch this back, we did this just to make mocking out the From impl easier
    // targets: Vec<Target>,
    targets: Vec<String>,
    recordType: String,
    setIdentifier: String,
    recordTTL: u64,
    labels: Option<HashMap<String, String>>,
    providerSpecific: Option<Vec<ProviderSpecificProperty>>,
}

impl From<&Value> for Endpoint {
    fn from(data: &Value) -> Self {
        let fqdn = match (data["hostname"].as_str(), data["domain"].as_str()) {
            (Some(hostname), Some(domain)) => Ok(format!("{hostname}.{domain}")),
            (_, _) => Err("Record domain unprocessable."),
        };
        // TODO: Revisit error handling here.
        // It's not clear at what point we should be doing final validation of this
        //   and if we should just be panicking on unwrap or logging and skipping or...
        Endpoint {
            dnsName: fqdn.unwrap(),
            targets: vec!["".into()],
            recordType: "".into(),
            setIdentifier: "".into(),
            recordTTL: 300_u64,
            labels: None,
            providerSpecific: None,
        }
    }
}

// TODO: this could just be a vec of tuples...
// TODO: We might well not even need it for this application.
//   Smells like an escape hatch for more complex providers
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProviderSpecificProperty {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Target {
    // Type is a reserved keyword, hence the little mouse tail
    type_: String,
}
