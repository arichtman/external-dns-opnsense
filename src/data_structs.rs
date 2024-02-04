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

impl From<Vec<&Value>> for Endpoints {
    fn from(data: Vec<&Value>) -> Self {
        let endpoints: Vec<Endpoint> = data.into_iter().map(|x| x.into()).collect();
        Endpoints { items: endpoints }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Endpoint {
    dnsName: String,
    targets: String,
    recordType: String,
    setIdentifier: String,
    recordTTL: u64,
    labels: Option<HashMap<String, String>>,
    providerSpecific: Option<Value>,
}

impl From<&Value> for Endpoint {
    fn from(data: &Value) -> Self {
        let fqdn = match (data["hostname"].as_str(), data["domain"].as_str()) {
            (Some(hostname), Some(domain)) => Ok(format!("{hostname}.{domain}")),
            (_, _) => Err("Record domain unprocessable."),
        };
        let target = data.get("server").unwrap().to_string();
        let _record_type = data.get("rr").unwrap().to_string();
        Endpoint {
            dnsName: fqdn.unwrap(),
            targets: target,
            recordType: "".into(),
            setIdentifier: "".into(),
            recordTTL: 300_u64,
            labels: None,
            providerSpecific: None,
        }
    }
}
