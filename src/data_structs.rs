// This is so our structs can use the names as specified by the API
#![allow(non_snake_case)]

use std::collections::HashMap;

use log::debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::opnsense::OPNSenseRecordType;

// TODO: see if we can use the ! whole-file approach
// Probably a bad idea since it'll bloat the code...
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Changes {
    create: EDNSEndpoints,
    updateOld: EDNSEndpoints,
    updateNew: EDNSEndpoints,
    delete: EDNSEndpoints,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EDNSEndpoints(Vec<EDNSEndpoint>);

// TODO: There must be a way to leverage one of these into the other
impl From<Vec<Value>> for EDNSEndpoints {
    fn from(data: Vec<Value>) -> Self {
        let endpoints: Vec<EDNSEndpoint> = data.into_iter().map(|x| (&x).into()).collect();
        EDNSEndpoints(endpoints)
    }
}
impl From<Vec<&Value>> for EDNSEndpoints {
    fn from(data: Vec<&Value>) -> Self {
        let endpoints: Vec<EDNSEndpoint> = data.into_iter().map(|x| x.into()).collect();
        EDNSEndpoints(endpoints)
    }
}

// TODO: I'm kindof keeping everything stringy for now so that these serialize nicely
//  into our request bodies. Though part of me is itching to reduce the Stringly-typeyness
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OPNSenseEndpoint {
    pub uuid: String,
    pub enabled: String,
    pub hostname: String,
    pub domain: String,
    pub rr: String,
    pub mxprio: String,
    pub mx: String,
    pub server: String,
    pub description: String,
}

impl From<Value> for OPNSenseEndpoint {
    fn from(data: Value) -> Self {
        // TODO: Surely the derive deserialize should handle this...
        debug!("{data:?}");
        OPNSenseEndpoint {
            uuid: data.get("uuid").unwrap().as_str().unwrap().to_string(),
            enabled: data
                .get("enabled")
                .expect("Field should be in object")
                .as_str()
                .expect("Field should be String type")
                .to_string(),
            // TODO: This seems _very_ verbose for just getting it without quotes...
            hostname: data.get("hostname").unwrap().as_str().unwrap().to_string(),
            domain: data["domain"].as_str().unwrap().to_string(),
            rr: data.get("rr").unwrap().as_str().unwrap().to_string(),
            // We can leave these blank as we don't manage MX records
            mxprio: String::from(""),
            mx: String::from(""),
            server: data.get("server").unwrap().as_str().unwrap().to_string(),
            description: data
                .get("description")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EDNSEndpoint {
    dnsName: String,
    targets: String,
    recordType: String,
    setIdentifier: String,
    recordTTL: u64,
    labels: Option<HashMap<String, String>>,
    providerSpecific: Option<Value>,
}

impl From<&Value> for EDNSEndpoint {
    fn from(data: &Value) -> Self {
        let fqdn = match (data["hostname"].as_str(), data["domain"].as_str()) {
            (Some(hostname), Some(domain)) => Ok(format!("{hostname}.{domain}")),
            (_, _) => Err("Record domain unprocessable."),
        };
        let target = data.get("server").unwrap().to_string();
        let _record_type = data.get("rr").unwrap().to_string();
        EDNSEndpoint {
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

impl From<OPNSenseEndpoint> for EDNSEndpoint {
    fn from(data: OPNSenseEndpoint) -> Self {
        let target = data.server.to_string();
        let _record_type = data.rr.to_string();
        EDNSEndpoint {
            dnsName: format!("{0}.{1}", data.hostname, data.domain),
            targets: target,
            recordType: "".into(),
            setIdentifier: "".into(),
            recordTTL: 300_u64,
            labels: None,
            providerSpecific: None,
        }
    }
}
impl From<Vec<OPNSenseEndpoint>> for EDNSEndpoints {
    fn from(data: Vec<OPNSenseEndpoint>) -> Self {
        let endpoints: Vec<EDNSEndpoint> = data.into_iter().map(|x| x.into()).collect();
        EDNSEndpoints(endpoints)
    }
}
