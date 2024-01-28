#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// TODO: see if we can use the ! whole-file approach
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
    // targets: Vec<Target>,
    targets: Vec<String>,
    recordType: String,
    setIdentifier: String,
    recordTTL: u64,
    labels: HashMap<String, String>,
    providerSpecific: Vec<ProviderSpecificProperty>,
}

// TODO: this could just be a vec of tuples...
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProviderSpecificProperty {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Target {
    // Type is a reserved keyword
    type_: String,
}
