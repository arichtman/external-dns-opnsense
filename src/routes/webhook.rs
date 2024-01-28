#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::Deserialize;

// TODO: see if we can use the ! whole-file approach
#[derive(Deserialize, Debug)]
struct Changes {
    create: Endpoints,
    updateOld: Endpoints,
    updateNew: Endpoints,
    delete: Endpoints,
}

#[derive(Deserialize, Debug)]
struct Endpoints {
    items: Vec<Endpoint>,
}

#[derive(Deserialize, Debug)]
struct Endpoint {
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
#[derive(Deserialize, Debug)]
struct ProviderSpecificProperty {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct Target {
    // Type is a reserved keyword
    type_: String,
}
